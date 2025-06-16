use crate::models::{taskitem};
use eframe::{egui::{panel::{Side, TopBottomSide}, Color32}, *};
use std::env;
use std::env::current_dir;

#[derive(Debug)]
pub struct MyApp {
    project_name:String,
    tasks: Vec<taskitem::TaskItem>,
    cwd: String,
}


impl MyApp{
    pub fn new(cc: &eframe::CreationContext<'_>) ->Self{
        cc.egui_ctx.set_visuals(egui::Visuals::light());

        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert("work_sans".to_owned(), egui::FontData::from_static(include_bytes!("../assets/space_mono.ttf")).into());
        fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "work_sans".to_owned());
        cc.egui_ctx.set_fonts(fonts);


        let current_directory = env::current_dir().unwrap();



        Self {
            project_name: String::from("Test"),
            tasks: vec![],
            cwd: String::from(current_dir().unwrap().to_str().unwrap())
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
        egui::TopBottomPanel::new(TopBottomSide::Top,"top_bar").resizable(true).show(ctx, |ui| {
            ui.add_space(7.0);
            ui.horizontal(|ui|{
                ui.button("Tasks");
                ui.button("Plan Mode");
                ui.button("Chat Mode");
            });
            ui.add_space(7.0);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("{self:#?}"));
        });
    }
}
