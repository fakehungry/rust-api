use actix_web::web::{self, Json};
use sqlx::PgPool;

use crate::{
    models::Task,
    utils::{CustomResponseBuilder, Response},
};

pub async fn get_task_by_id(
    pool: web::Data<PgPool>,
    task_id: web::Path<i32>,
) -> Response<Json<Task>> {
    let id = task_id.into_inner();
    let row = sqlx::query!(r#"SELECT * FROM tasks WHERE id = $1;"#, id)
        .fetch_one(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Failed to fetch task with id {}: {}", id, e);
            actix_web::error::ErrorNotFound("Task not found")
        })?;

    let task = crate::models::Task {
        id: row.id,
        title: row.title,
        description: row.description,
        priority: Some(row.priority),
    };

    let response = CustomResponseBuilder::new().body(Json(task)).build();

    Ok(response)
}
