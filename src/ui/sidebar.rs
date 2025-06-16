use eframe::egui;
use eframe::egui::panel::Side;
use serde::{Deserialize, Serialize};
use crate::app::MyApp;

#[derive(Debug,Serialize,Deserialize)]
#[derive(Default)]
pub struct Sidebar;
impl Sidebar{
    pub fn render(&mut self,ctx: &egui::Context){
        egui::SidePanel::new(Side::Left, "left_bar")
            .resizable(true)
            .show(ctx, |ui| {
                ui.add_space(13.0);
                ui.vertical_centered(|ui| ui.label("PlanKite"));
                ui.add_space(13.0);
                ui.separator();
                ui.vertical_centered(|ui| {
                    let _ = ui.button("Regular Button");
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                        let _ = ui.button("Bottom");
                        ui.separator();
                    });
                });
            });
    }
}
