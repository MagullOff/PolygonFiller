#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use egui::Vec2;
    use polygon_filler::consts::IMAGE_SIZE;
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions {
        min_window_size: Some(Vec2 {
            x: IMAGE_SIZE as f32 + 50.0,
            y: IMAGE_SIZE as f32 + 50.0,
        }),
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        "Polygon Filler",
        native_options,
        Box::new(|cc| Box::new(polygon_filler::PolygonFiller::new(cc))),
    );
}
