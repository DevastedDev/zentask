use crate::models::{taskitem};
use eframe::{egui::panel::{Side, TopBottomSide}, *};
use std::env;
use serde_json;
use std::fs;
use eframe::egui::Vec2;
use serde::Deserialize;
use crate::models::taskitem::TaskItem;

#[derive(Debug,Deserialize)]
pub struct MyApp {
    project_name:String,
    tasks: Vec<TaskItem>,
    cwd: String,
}


impl MyApp{
    pub fn new(cc: &eframe::CreationContext<'_>) ->Self{
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert("work_sans".to_owned(), egui::FontData::from_static(include_bytes!("../assets/space_mono.ttf")).into());
        fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "work_sans".to_owned());
        cc.egui_ctx.set_fonts(fonts);
        cc.egui_ctx.style_mut(|style| {
            style.spacing.button_padding = Vec2::new(12.0, 8.0);
            style.spacing.item_spacing = Vec2::new(8.0, 6.0);
            style.spacing.indent = 20.0;
            style.text_styles.insert(
                egui::TextStyle::Button,
                egui::FontId::new(16.0, egui::FontFamily::Proportional),
            );
            style.text_styles.insert(
                egui::TextStyle::Body,
                egui::FontId::new(15.0, egui::FontFamily::Proportional),
            );
            style.text_styles.insert(
                egui::TextStyle::Heading,
                egui::FontId::new(20.0, egui::FontFamily::Proportional),
            );
        });
        cc.egui_ctx.set_pixels_per_point(1.4);
        let current_directory = env::current_dir().unwrap().to_string_lossy().to_string();
        let file_path = format!("{}/.agent/data.json",current_directory);
        let mut data_json: MyApp = serde_json::from_str(fs::read_to_string(file_path).unwrap().as_str()).expect("REASON");
        data_json.load_tasks(current_directory);

        println!("{data_json:#?}");
        Self {
            ..data_json
        }
    }
    fn load_tasks(&mut self, cwd: String) {
        let dir_data = fs::read_dir(format!("{cwd}/.agent/")).unwrap();
        let task_files: Vec<_> = dir_data
            .filter_map(|d| d.ok())
            .filter(|entry| {
                entry.file_name().to_str()
                    .map(|name| name.contains("task_"))
                    .unwrap_or(false)
            })
            .collect();

        for entry in task_files {
            let file_name = format!("{cwd}/.agent/{}", entry.file_name().to_string_lossy());
            let task_item: TaskItem = serde_json::from_str(&fs::read_to_string(&file_name).unwrap()).unwrap();
            self.tasks.push(task_item);
            println!("Loaded task: {:#?}", self.tasks.last().unwrap());
        }
    }

}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::new(Side::Left,"left_bar").resizable(true).show(ctx, |ui| {
            ui.add_space(13.0);
            ui.vertical_centered(|ui|{
                ui.label("PlanKite")
            });
        });
        egui::TopBottomPanel::new(TopBottomSide::Top,"top_bar").show(ctx, |ui| {
            ui.add_space(7.0);
            ui.horizontal(|ui|{
                ui.button("Tasks");
                ui.button("Plan Mode");
                ui.button("Chat Mode");
            });
            ui.add_space(7.0);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(format!("{self:#?}"));
            });
        });
    }
}
