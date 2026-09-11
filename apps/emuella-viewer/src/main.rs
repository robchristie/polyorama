#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    let args: Vec<String> = std::env::args().collect();
    let value = |flag: &str| args.windows(2).find(|v| v[0] == flag).map(|v| v[1].clone());
    let server = value("--server").unwrap_or_else(|| "http://127.0.0.1:8088".into());
    let compressed = value("--compressed-mib")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(64)
        << 20;
    let decoded = value("--decoded-mib")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(16)
        << 20;
    let gpu = value("--gpu-mib")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(64)
        << 20;
    let output = value("--script-output");
    let script = output.is_some();
    let workload =
        value("--workload").map(|path| std::fs::read_to_string(path).expect("read workload"));
    let mut native_options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1440., 900.])
            .with_min_inner_size([960., 640.]),
        ..Default::default()
    };
    // Consumer opt-in: preserve every other surface and application default.
    if let Some(mode) = value("--present-mode") {
        assert_eq!(mode, "auto-no-vsync", "unsupported --present-mode");
        native_options.wgpu_options.surface.present_mode = wgpu::PresentMode::AutoNoVsync;
    }
    eframe::run_native(
        "Emuella image viewer",
        native_options,
        Box::new(move |cc| {
            let mut app = emuella_viewer::ViewerApp::new(
                cc, server, compressed, decoded, gpu, script, output,
            );
            if let Some(json) = workload {
                app.set_script_workload(&json)
                    .map_err(std::io::Error::other)?;
            }
            Ok(Box::new(app))
        }),
    )
}
#[cfg(target_arch = "wasm32")]
fn main() {}
