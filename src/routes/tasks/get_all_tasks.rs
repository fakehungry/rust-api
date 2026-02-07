use actix_web::web;
use serde::Serialize;
use sqlx::PgPool;

use crate::models::Task;
use crate::utils::Pagination;
use crate::utils::custom_response::{CustomResponseBuilder, Response};

#[derive(Serialize)]
pub struct TaskResponse {
    tasks: Vec<Task>,
    total_count: i64,
}

// TODO: Separation of concerns: move DB logic to a separate module
pub async fn get_all_tasks(
    pool: web::Data<PgPool>,
    query: web::Query<Pagination>,
) -> Response<TaskResponse> {
    let pagination = query.into_inner();
    if pagination.limit == None || pagination.offset == None {
        return Err(actix_web::error::ErrorBadRequest(
            "Both limit and offset must be provided",
        ));
    }
    let rows = sqlx::query!(
        r#"
        SELECT id, title, description, priority, COUNT(*) OVER() AS total_count
        FROM tasks
        ORDER BY id
        LIMIT $1 OFFSET $2;
        "#,
        pagination.limit.unwrap() as i64,
        pagination.offset.unwrap() as i64,
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch tasks: {}", e);
        actix_web::error::ErrorInternalServerError("Failed to fetch tasks")
    })?;

    let total_count = rows.first().map_or(0, |r| r.total_count.unwrap_or(0));
    let tasks: Vec<Task> = rows
        .into_iter()
        .map(|row| Task {
            id: row.id,
            title: row.title,
            description: row.description,
            priority: Some(row.priority),
        })
        .collect();
    let response = CustomResponseBuilder::new()
        .body(TaskResponse { tasks, total_count })
        .pagination(pagination)
        .build();

    Ok(response)
}
