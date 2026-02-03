use actix_web::web;
use serde::Serialize;
use sqlx::PgPool;

use crate::utils::custom_response::{CustomResponseBuilder, Response};

#[derive(Serialize)]
pub struct Task {
    id: i32,
    title: String,
    description: Option<String>,
    priority: Option<String>,
}

// TODO: Separation of concerns: move DB logic to a separate module
pub async fn get_all_tasks(pool: web::Data<PgPool>) -> Response<Vec<Task>> {
    let rows = sqlx::query!(r#"SELECT * FROM tasks;"#)
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Failed to fetch tasks: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to fetch tasks")
        })?;

    let tasks: Vec<Task> = rows
        .into_iter()
        .map(|row| Task {
            id: row.id,
            title: row.title,
            description: row.description,
            priority: Some(row.priority),
        })
        .collect();

    let response = CustomResponseBuilder::new().body(tasks).build();

    Ok(response)
}
