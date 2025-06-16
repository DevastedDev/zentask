use crate::models::taskitem::TaskItem;
use crate::ui::main::Main;
use crate::ui::sidebar::Sidebar;
use crate::ui::topbar::Topbar;
use eframe::egui::Vec2;
use eframe::*;
use serde::Deserialize;
use serde_json;
use std::env;
use std::fs;

#[derive(Debug, Deserialize,Default)]
pub struct MyApp {
    project_name: String,
    tasks: Vec<TaskItem>,
    cwd: String,
    #[serde(skip)] main: Main,
    #[serde(skip)] topbar: Topbar,
    #[serde(skip)] sidebar: Sidebar,
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "work_sans".to_owned(),
            egui::FontData::from_static(include_bytes!("../assets/space_mono.ttf")).into(),
        );
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "work_sans".to_owned());
        cc.egui_ctx.set_fonts(fonts);
        cc.egui_ctx.style_mut(|style| {
            style.spacing.button_padding = Vec2::new(10.0, 6.0);
            style.spacing.item_spacing = Vec2::new(6.0, 4.0);
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
        cc.egui_ctx.set_pixels_per_point(1.1);

        let current_directory = env::current_dir().unwrap().to_string_lossy().to_string();
        let file_path = format!("{}/.agent/data.json", current_directory);

        let mut data_json: MyApp =
            serde_json::from_str(fs::read_to_string(file_path).unwrap().as_str()).expect("REASON");
        data_json.load_tasks(current_directory);
        Self {
            main: Main,
            topbar: Topbar,
            sidebar: Sidebar,
            ..data_json
        }
    }
    fn load_tasks(&mut self, cwd: String) {
        let dir_data = fs::read_dir(format!("{cwd}/.agent/")).unwrap();
        let task_files: Vec<_> = dir_data
            .filter_map(|d| d.ok())
            .filter(|entry| {
                entry
                    .file_name()
                    .to_str()
                    .map(|name| name.contains("task_"))
                    .unwrap_or(false)
            })
            .collect();

        for entry in task_files {
            let file_name = format!("{cwd}/.agent/{}", entry.file_name().to_string_lossy());
            let task_item: TaskItem =
                serde_json::from_str(&fs::read_to_string(&file_name).unwrap()).unwrap();
            self.tasks.push(task_item);
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        catppuccin_egui::set_theme(&ctx, catppuccin_egui::MOCHA);
        let (tasks,cwd) = (&self.tasks,&self.cwd);
        self.sidebar.render(&ctx);
        self.topbar.render(&ctx);
        self.main.render(&ctx,tasks);
    }
}
