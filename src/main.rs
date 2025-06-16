use eframe::*;
mod app;
mod models;
mod ui;
mod utils;

use app::MyApp;

fn main(){
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_maximized(true)
            .with_fullscreen(false),
        ..NativeOptions::default()
    };
    run_native(
        "Project Little Star",
        options,
        Box::new(|cc| {
            Ok(Box::new(MyApp::new(cc)))
        }),
    ).expect("Failed to start the application")
}
