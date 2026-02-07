use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
}

#[derive(Deserialize)]
pub struct TaskFormData {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
}
