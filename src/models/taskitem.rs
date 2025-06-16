use crate::models::subtask::SubTask;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskItem {
    title: String,
    short_desc: String,
    description: String,
    sub_tasks: Vec<SubTask>,
    status: String,
}

impl TaskItem {}
