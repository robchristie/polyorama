import {chromium} from '/nvme/development/emuella/worktrees/polyorama-proof/polyorama/node_modules/playwright/index.mjs';
import {mkdir,writeFile} from 'node:fs/promises';
import {join} from 'node:path';
const [url,out]=process.argv.slice(2); await mkdir(out,{recursive:false});
const record={schema:'viewer-same-source-lru-observation/1',url,started_unix_ms:Date.now(),completed:false,samples:[],errors:[],workers:0,cache_state:'fresh browser context; service warm after complete journeys; OS/source storage uncontrolled',limits:{compressed_bytes:1048576,decoded_bytes:4194304,gpu_bytes:16777216},latency_claim:'none; predeclared 60-second settlement deadline'};
const browser=await chromium.launch({headless:true,args:['--no-sandbox','--enable-unsafe-webgpu','--use-angle=vulkan','--enable-features=Vulkan,CDPScreenshotNewSurface','--disable-vulkan-surface','--disable-dev-shm-usage']});
let timer;
try {
 record.browser_version=browser.version();
 const page=await browser.newPage({viewport:{width:1440,height:900}});
 page.on('worker',()=>record.workers++);page.on('pageerror',e=>record.errors.push(String(e)));
 await page.addInitScript(()=>{window.__adapters=[];const original=GPUAdapter.prototype.requestDevice;GPUAdapter.prototype.requestDevice=async function(...args){const d=await original.apply(this,args);window.__adapters.push({vendor:this.info.vendor,architecture:this.info.architecture,device:this.info.device,description:this.info.description});return d;};});
 record.catalogue=await(await fetch(url+'/catalogue')).json();
 await page.goto(url+'/#compressed_mib=1&decoded_mib=4&gpu_mib=16');
 await page.waitForFunction(()=>window.emuellaViewer,null,{timeout:60000});
 timer=setInterval(async()=>{try{record.samples.push(await page.evaluate(()=>window.emuellaViewer.snapshot()));}catch{}},100);
 await page.waitForFunction(()=>{const s=window.emuellaViewer.snapshot();return s.errors.length||s.desired>0&&s.ready_demands===s.desired&&s.in_flight===0;},null,{timeout:60000});
 const s=record.final=await page.evaluate(()=>window.emuellaViewer.snapshot());record.adapters=await page.evaluate(()=>window.__adapters);
 if(s.errors.length)throw new Error(JSON.stringify(s.errors));
 if(s.primary_desired!==121||s.primary_ready!==121)throw new Error('121 primary overview regions not resident');
 if(s.worker.compressed_bin_evictions<=0||s.worker.representation_evictions!==0)throw new Error('same-source bin eviction not proved');
 if(s.worker.peak_compressed_bytes>1048576||s.decoded_peak_bytes>4194304||s.gpu_peak_bytes>16777216)throw new Error('resource caps exceeded');
 const tids=new Set(record.samples.flatMap(s=>s.worker?.decoded_evidence??[]).concat(s.worker.decoded_evidence).map(e=>e.tid));
 record.decoded_parent_tids=[...tids];
 if(tids.size!==1||!tids.has(record.catalogue[0].tid))throw new Error('more than the initial parent decoded');
 await page.screenshot({path:join(out,'large-overview-lru.png')});record.completed=true;
} catch(e){record.error=String(e);process.exitCode=4;} finally {clearInterval(timer);record.completed_unix_ms=Date.now();await writeFile(join(out,'lru.json'),JSON.stringify(record,null,2));await browser.close();}
