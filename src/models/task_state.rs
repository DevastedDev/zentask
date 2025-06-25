use std::{fs, path::{Path, PathBuf}};

use serde::Deserialize;
use crate::models::{appcommands::AppCommands, taskitem::TaskItem};


#[derive(Debug,Default,Deserialize)]
pub struct TaskState{
    pub items: Vec<TaskItem>
}

impl TaskState{
    pub fn load_task_struct(&mut self,task:TaskItem){
        self.items.push(task);
    }

    pub fn add_new_task(&mut self, cwd: &str, task: &mut TaskItem) -> Option<AppCommands> {
        task.status = String::from("pending");
        task.sub_tasks = vec![];
        let new_count: usize = self.items.len() + 1;
        let file_name = format!("{}/task_{}.json", cwd, new_count);

        // Check if directory exists
        if !Path::new(cwd).exists() {
            eprintln!("Directory does not exist: {}", cwd);
            return None;
        }

        let contents = match serde_json::to_string_pretty::<TaskItem>(&task) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Serialization failed: {e}");
                return None;
            }
        };

        println!("Attempting to write to file: {}", file_name);

        match fs::write(&file_name, contents) {
            Ok(_) => {
                self.items.push(task.clone());
                Some(AppCommands::AddTaskSucess)
            }
            Err(e) => {
                eprintln!("Failed to write file: {e}");
                None
            }
        }
    }

}
