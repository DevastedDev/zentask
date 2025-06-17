use crate::models::taskitem::TaskItem;
use crate::ui::main::Main;
use crate::ui::sidebar::Sidebar;
use crate::ui::style::{get_font, set_styles};
use crate::ui::topbar::Topbar;
use eframe::egui::Color32;
use eframe::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct MyApp {
    project_name: String,
    #[serde(skip)]
    tasks: Vec<TaskItem>,
    cwd: String,
    #[serde(skip)]
    main: Main,
    #[serde(skip)]
    topbar: Topbar,
    #[serde(skip)]
    sidebar: Sidebar,
    #[serde(skip)]
    open_dir_window: bool,
    #[serde(skip)]
    selected_directory: Option<PathBuf>,
}

impl MyApp {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        // Set Global Styles
        set_styles(cc);
        // Sets The Font Globally
        cc.egui_ctx.set_fonts(get_font());

        let mut app = if let Some(storage) = cc.storage {
            get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Self::default()
        };

        if app.cwd.is_empty() || !Path::new(&app.cwd).exists() {
            app.open_dir_window = true;
        } else {
            let dir_path = format!("{}/.agent/", { &app.cwd });
            if !Path::new(&dir_path).exists() {
                app.open_dir_window = true;
            } else {
                app.load_project();
            }
        }

        Self {
            main: Main,
            topbar: Topbar,
            sidebar: Sidebar,
            ..app
        }
    }

    fn load_tasks(&mut self) {
        let dir_data = fs::read_dir(format!("{}/.agent/", self.cwd)).unwrap();
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
            let file_name = format!(
                "{}/.agent/{}",
                self.cwd,
                entry.file_name().to_string_lossy()
            );
            if let Ok(val) = fs::read_to_string(file_name) {
                if let Ok(task) = serde_json::from_str::<TaskItem>(val.as_str()) {
                    self.tasks.push(task);
                }
            }
        }
    }

    fn create_project(&mut self) {
        let agent_dir = format!("{}/.agent/", self.cwd);
        if let Err(_) = fs::create_dir_all(agent_dir) {
            return;
        }
        let ft_create = format!("{}/.agent/data.json", self.cwd);
        let data = serde_json::json!({
            "project_name":"Project",
            "cwd":self.cwd
        });
        if let Ok(_) = fs::write(ft_create, serde_json::to_string_pretty(&data).unwrap()) {
            self.load_project()
        }
    }

    pub fn load_project(&mut self) {
        let data_file = format!("{}/.agent/data.json", self.cwd);
        if let Ok(val) = fs::read_to_string(data_file) {
            let data = serde_json::from_str::<MyApp>(val.as_str()).unwrap();
            self.project_name = data.project_name;
            self.cwd = data.cwd;
        }
        self.load_tasks();
    }

    fn choose_dialog(&mut self, ctx: &egui::Context) {
        egui::Window::new("Select/Open a Project")
            .resizable(true)
            .collapsible(false)
            .default_width(600.0)
            .show(ctx, |ui| {
                ui.heading("Select Project Directory");
                if ui.button("Browse...").clicked(){
                    if let Some(val) = rfd::FileDialog::new().pick_folder(){
                        self.selected_directory = Some(val)
                    }
                }

                if let Some(ref dir) = self.selected_directory{
                    ui.add_space(20.0);
                    ui.label(format!("Selected Directory : {}",dir.to_string_lossy()));

                    let folder = dir.join(".agent");
                    if folder.exists() {
                        ui.colored_label(Color32::GREEN,"Found existing project (.agent), Please click continue to create a project");
                        if ui.button("Continue").clicked() {
                            self.cwd = dir.to_string_lossy().to_string();
                            self.load_project();
                            self.selected_directory=None;
                            self.open_dir_window = false;
                        }
                    }else {
                        ui.colored_label(Color32::RED,"Click continue to initialize a new project");
                        if ui.button("Continue").clicked() {
                            self.cwd = dir.to_string_lossy().to_string();
                            self.create_project();
                            self.selected_directory=None;
                            self.open_dir_window= false;
                        }
                    }
                }
            });
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        catppuccin_egui::set_theme(&ctx, catppuccin_egui::MOCHA);
        if self.open_dir_window {
            self.choose_dialog(&ctx);
        } else {
            let (tasks, project_name) = (&self.tasks, &self.project_name);
            self.sidebar.render(&ctx);
            self.topbar.render(&ctx,project_name);
            self.main.render(&ctx, tasks);
        }
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        set_value(storage, "app_info", self)
    }
}
