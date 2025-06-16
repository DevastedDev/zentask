use eframe::egui;
use eframe::egui::panel::TopBottomSide;
use eframe::egui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Topbar;
impl Topbar {
    pub fn render(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::new(TopBottomSide::Top, "top_bar").show(ctx, |ui| {
            ui.add_space(7.0);
            ui.horizontal(|ui| {
                ui.heading("ZenTask");
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
