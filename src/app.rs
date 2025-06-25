use crate::models::appcommands::AppCommands;
use crate::models::task_state::TaskState;
use crate::models::taskitem::TaskItem;
use crate::ui::main::Main;
use crate::ui::sidebar::Sidebar;
use crate::ui::style::{get_font, set_styles};
use crate::ui::topbar::Topbar;
use eframe::egui::{Align2, Color32};
use eframe::*;
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct MyApp {
    project_name: String,
    #[serde(skip)]
    task_state:TaskState,
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
    add_task_window: bool,
    #[serde(skip)]
    selected_directory: Option<PathBuf>,
    #[serde(skip)]
    add_task_cnf : TaskItem
}

impl MyApp {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        set_styles(cc);
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
        self.task_state.items.clear();
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
                    self.task_state.load_task_struct(task);
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
                        ui.colored_label(Color32::GREEN,"Found existing project (.agent), Please click continue to open the project");
                        if ui.button("Continue").clicked() {
                            self.cwd = dir.to_string_lossy().to_string();
                            self.load_project();
                            self.selected_directory = None;
                            self.open_dir_window = false;
                        }
                    }else {
                        ui.colored_label(Color32::RED,"Click continue to initialize a new project");
                        if ui.button("Continue").clicked() {
                            self.cwd = dir.to_string_lossy().to_string();
                            self.create_project();
                            self.selected_directory = None;
                            self.open_dir_window = false;
                        }
                    }
                }
            });
    }

    pub fn render_add_task_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("Add Task")
            .resizable(true)
            .drag_to_scroll(true)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("📝 Add new task");
                    ui.with_layout(
                        egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.add(egui::Button::new("❌ Close").fill(ui.visuals().warn_fg_color)).clicked() {
                                self.add_task_window = false;
                            }
                        }
                    );
                });

                ui.add_space(16.0);

                ui.group(|ui| {
                    ui.label("Title");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.add_task_cnf.title)
                            .hint_text("Enter a title")
                            .desired_width(f32::INFINITY)
                    );
                    ui.add_space(8.0);

                    ui.label("Short Description");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.add_task_cnf.short_desc)
                            .hint_text("Short summary (optional)")
                            .desired_width(f32::INFINITY)
                    );
                    ui.add_space(8.0);

                    ui.label("Description");
                    ui.add(
                        egui::TextEdit::multiline(&mut self.add_task_cnf.description)
                            .hint_text("Full description...")
                            .desired_rows(4)
                            .desired_width(f32::INFINITY)
                    );
                });

                ui.add_space(16.0);

                ui.horizontal(|ui| {
                    ui.with_layout(
                        egui::Layout::right_to_left(egui::Align::Min), |ui| {
                            if ui.add(
                                egui::Button::new("➕ Add Task")
                                    .fill(ui.visuals().selection.bg_fill)
                                    .min_size(egui::Vec2::new(100.0, 30.0))
                            ).clicked() {
                                match self.task_state.add_new_task(&self.cwd,&mut self.add_task_cnf) {
                                    Some(command) => { self.handle_action(command);},
                                    None => { self.handle_action(AppCommands::AddTaskFailed);}
                                }
                            }
                            if ui.add(
                                egui::Button::new("Cancel")
                                    .min_size(egui::Vec2::new(80.0, 30.0))
                            ).clicked() {
                                self.add_task_window = false;
                            }
                        }
                    );
                });
            });

    }
    pub fn handle_action(&mut self, command: AppCommands) {
        match command {
            AppCommands::OpenNewProject => {
                self.open_dir_window = true;
            },
            AppCommands::AddTask => {
                self.add_task_window = true;
            },
            AppCommands::AddTaskSucess => {
                let mut toasts = Toasts::new()
                    .anchor(Align2::RIGHT_BOTTOM, (-10.0, -10.0))
                    .direction(egui::Direction::BottomUp);

                toasts.add(Toast {
                    text: "Task Added!".into(),
                    kind: ToastKind::Info,
                    options: ToastOptions::default()
                        .duration_in_seconds(5.0)
                        .show_progress(true),
                    ..Default::default()
                });

            },
            AppCommands::AddTaskFailed => {
                let mut toasts = Toasts::new()
                    .anchor(Align2::RIGHT_BOTTOM, (-10.0, -10.0))
                    .direction(egui::Direction::BottomUp);
                toasts.add(Toast {
                    text: "Task Failed to add!".into(),
                    kind: ToastKind::Error,
                    options: ToastOptions::default()
                        .duration_in_seconds(5.0)
                        .show_progress(true),
                    ..Default::default()
                });
            }
            _ => {},
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        catppuccin_egui::set_theme(&ctx, catppuccin_egui::FRAPPE);
        if self.open_dir_window {
            self.choose_dialog(&ctx);
        } else {
            if let Some(command) = self.sidebar.render(&ctx) {
                self.handle_action(command);
            }
            let (task_state, project_name) = (&self.task_state, &self.project_name);
            self.topbar.render(&ctx, project_name);
            self.main.render(&ctx, task_state);


            if self.add_task_window{
                self.render_add_task_window(&ctx)
            }
        }
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        set_value(storage, eframe::APP_KEY, self)
    }
}
