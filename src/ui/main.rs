use crate::models::taskitem::TaskItem;
use eframe::egui;
use eframe::epaint::Shadow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Main;
impl Main {
    pub fn render(&mut self, ctx: &egui::Context, tasks: &[TaskItem]) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for task in tasks {
                    // Create a frame with border and shadow
                    let frame = egui::Frame::default()
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 0, 0)))
                        .shadow(Shadow {
                            color: egui::Color32::from_black_alpha(80),
                            offset:[1,1],
                            blur:4,
                            spread:1
                        })
                        .inner_margin(egui::Margin::same(8))
                        .outer_margin(egui::Margin::same(2));

                    frame.show(ui, |ui| {
                        ui.set_width(ui.available_width());

                        ui.horizontal(|ui| {
                            let _ = egui::CollapsingHeader::new(&task.title)
                                .default_open(false)
                                .show(ui, |ui| {
                                    ui.label(&task.short_desc);
                                    ui.label(format!("Status: {}", &task.status));
                                });

                            // Push buttons to the right side
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button("Edit").clicked() {
                                }
                                if ui.button("+").clicked() {
                                }
                                if ui.button("Open").clicked() {
                                }
                            });
                        });
                    });
                    ui.add_space(5.0);
                }
            });
        });
    }

}
