use crate::models::task_state::TaskState;
use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Main;
impl Main {
    pub fn render(&mut self, ctx: &egui::Context, tasks: &TaskState) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for task in &tasks.items {
                    let frame = egui::Frame::none()
                        .inner_margin(egui::Margin::symmetric(12, 8))
                        .outer_margin(egui::Margin::symmetric(0, 2));

                    frame.show(ui, |ui| {
                        ui.set_width(ui.available_width());

                        ui.horizontal(|ui| {
                            let _ = egui::CollapsingHeader::new(&task.title)
                                .default_open(false)
                                .show(ui, |ui| {
                                    ui.label(&task.short_desc);
                                    ui.label(format!("Status: {}", &task.status));
                                });

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button("Edit").clicked() {}
                                if ui.button("+").clicked() {}
                                if ui.button("Open").clicked() {}
                            });
                        });
                    });
                    ui.add_space(5.0);
                }
            });
        });
    }
}
