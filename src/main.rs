use eframe::*;
mod app;
mod models;

use app::MyApp;

fn main(){
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "New TodoList",
        options,
        Box::new(|cc| {
            Ok(Box::new(MyApp::new(cc)))
        }),
    ).expect("Failed to start the application")
}
