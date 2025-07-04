use crate::models::appcommands::AppCommands;
use eframe::egui;
use eframe::egui::panel::Side;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Sidebar;
impl Sidebar {
    pub fn render(&mut self, ctx: &egui::Context) -> Option<AppCommands> {
        let mut to_send: Option<AppCommands> = None;
        egui::SidePanel::new(Side::Left, "left_bar")
            .resizable(true)
            .show(ctx, |ui| {
                ui.add_space(10.0);
                ui.vertical_centered(|ui| {
                    ui.heading("ZenTask");
                });
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
                ui.vertical_centered(|ui| {
                    if ui.button("Add Task").clicked() {
                        to_send = Some(AppCommands::AddTask)
                    }
                });

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(10.0);
                    if ui.button("Open New Project").clicked() {
                        to_send = Some(AppCommands::OpenNewProject)
                    }
                    ui.separator();
                });
            });
        to_send
    }
}
