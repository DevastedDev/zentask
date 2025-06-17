use eframe::egui;
use eframe::egui::panel::TopBottomSide;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Topbar;
impl Topbar {
    pub fn render(&mut self, ctx: &egui::Context,project_name:&String) {
        egui::TopBottomPanel::new(TopBottomSide::Top, "top_bar").show(ctx, |ui| {
            ui.add_space(7.0);
            ui.horizontal(|ui| {
                ui.heading(format!("{project_name}") );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    let _ = ui.button("Chat Mode");
                    let _ = ui.button("Plan Mode");
                    let _ = ui.button("Tasks");
                });
            });
            ui.add_space(7.0);
        });
    }
}
