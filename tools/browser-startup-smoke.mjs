// Deterministic startup contracts against the exact packaged bytes, without timing gates.
import assert from 'node:assert/strict';
import { chromium } from 'playwright';
import { hostedLinuxWebGpuLaunchOptions } from './browser-launch.mjs';
import { createProductionServer } from './browser-serve.mjs';
const directory = process.argv[2] || 'target/browser-production';
const browser = await chromium.launch(hostedLinuxWebGpuLaunchOptions());
try {
  if (process.argv[3]) {
    const previous = createProductionServer({ directory: process.argv[3] });
    await new Promise(resolve => previous.listen(0, '127.0.0.1', resolve));
    const port = previous.address().port;
    const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
    const page = await context.newPage();
    await page.goto(`http://127.0.0.1:${port}/lab/`);
    await page.waitForFunction(() => window.__POLYORAMA_STARTUP?.status === 'ready');
    const oldId = previous.manifest.apps.find(app => app.name === 'lab').contentId;
    const oldEtag = previous.requests.find(r => r.url === '/lab/').headers.etag;
    await page.goto('about:blank');
    previous.closeAllConnections(); await new Promise(resolve => previous.close(resolve));
    const current = createProductionServer({ directory, retainedDirectories: [process.argv[3]] });
    await new Promise(resolve => current.listen(port, '127.0.0.1', resolve));
    try {
      const newId = current.manifest.apps.find(app => app.name === 'lab').contentId;
      assert.notEqual(oldId, newId, 'publication check needs two distinct builds');
      await page.goto(`http://127.0.0.1:${port}/lab/`);
      await page.waitForFunction(() => window.__POLYORAMA_STARTUP?.status === 'ready');
      const assets = await page.evaluate(() => performance.getEntriesByType('resource').map(r => r.name).filter(name => name.includes('/assets/')));
      assert(assets.length > 0 && assets.every(name => name.includes(`/assets/${newId}/`)), 'revisit must use only new package assets');
      assert(current.requests.some(r => r.url === '/lab/' && r.status === 200 && r.ifNoneMatch === oldEtag), 'normal revisit must revalidate mutable HTML');
      console.log(`Normal cached revisit upgraded ${oldId} -> ${newId} without mixing packages`);
    } finally { await context.close(); current.closeAllConnections(); await new Promise(resolve => current.close(resolve)); }
  }
  for (const basePath of ['/', '/preview/nested/']) {
    const server = createProductionServer({ directory, basePath });
    await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
    try {
      for (const app of ['lab', 'gallery']) {
        const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
        const page = await context.newPage();
        const errors = []; page.on('pageerror', e => errors.push(String(e)));
        await page.goto(`http://127.0.0.1:${server.address().port}${basePath}${app}/`);
        await page.waitForFunction(() => window.__POLYORAMA_STARTUP?.status !== 'starting' && window.__POLYORAMA_STARTUP, null, { timeout: 60000 });
        const report = await page.evaluate(() => window.__POLYORAMA_STARTUP);
        assert.equal(report.status, 'ready', JSON.stringify(report));
        for (const milestone of ['wasm_init_begin', 'wasm_init_end', 'application_construct_begin', 'application_construct_end', 'framework_start_begin', 'framework_start_end', 'workspace_frame_submitted', 'rendering_opportunity_proxy', 'first_useful_content']) assert(report.milestones[milestone], milestone);
        assert.equal(errors.length, 0, errors.join('\n'));
        await context.close();
        console.log(`Startup complete: ${basePath}${app}/`);
      }
      for (const app of ['lab', 'gallery', 'viewer']) for (const failure of app === 'gallery' ? ['webgpu'] : ['webgpu', 'worker']) {
        const context = await browser.newContext();
        if (failure === 'worker') await context.route('**/worker.js', route => route.abort());
        else await context.addInitScript(() => Object.defineProperty(navigator, 'gpu', { value: undefined }));
        const page = await context.newPage();
        await page.goto(`http://127.0.0.1:${server.address().port}${basePath}${app}/`);
        await page.waitForFunction(() => window.__POLYORAMA_STARTUP?.status === 'failed', null, { timeout: 60000 });
        const result = await page.evaluate(() => ({ startup: window.__POLYORAMA_STARTUP, text: document.getElementById('loading').textContent, ready: document.body.classList.contains('ready') }));
        assert(result.startup.failure.phase.includes(failure === 'worker' ? 'worker' : 'framework'), JSON.stringify(result));
        assert(result.text.includes('Initialisation failed')); assert.equal(result.ready, false);
        assert.equal(result.startup.cleanup, failure === 'worker' ? 'destroyed' : 'not_constructed', 'failed startup must release its handle');
        await context.close(); console.log(`Startup failure handled: ${basePath}${app}/${failure}`);
      }
    } finally { server.closeAllConnections(); await new Promise(resolve => server.close(resolve)); }
  }
} finally { await browser.close(); }
