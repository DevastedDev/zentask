use eframe::egui;
use eframe::egui::panel::Side;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Sidebar;
impl Sidebar {
    pub fn render(&mut self, ctx: &egui::Context) {
        egui::SidePanel::new(Side::Left, "left_bar")
            .resizable(true)
            .show(ctx, |ui| {
                ui.add_space(13.0);
                ui.vertical_centered(|ui| ui.heading("ZenTask"));
                ui.add_space(7.0);
                ui.separator();
                ui.vertical_centered(|ui| {
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                        if ui.button("Open New Project").clicked(){
                        }
                        ui.separator();
                    });
                });
            });
    }
}
