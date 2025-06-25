use crate::models::subtask::SubTask;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize,Default,Clone)]
pub struct TaskItem {
    pub title: String,
    pub short_desc: String,
    pub description: String,
    pub sub_tasks: Vec<SubTask>,
    pub status: String,
}
