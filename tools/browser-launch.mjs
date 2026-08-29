export function hostedLinuxWebGpuLaunchOptions() {
  const headless = process.env.POLYORAMA_BROWSER_HEADFUL !== '1';
  const args = [
    '--no-sandbox',
    '--enable-unsafe-webgpu',
    '--use-webgpu-adapter=swiftshader',
    '--use-gpu-in-tests',
    '--use-gl=angle',
    '--use-angle=swiftshader',
    '--enable-unsafe-swiftshader',
    '--enable-features=CDPScreenshotNewSurface',
  ];
  if (!headless) args.push('--ozone-platform=x11');
  return { headless, args };
}
