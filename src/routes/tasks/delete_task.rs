use actix_web::web;
use sqlx::PgPool;

use crate::{
    models::Task,
    utils::{CustomResponseBuilder, Response},
};

pub async fn delete_task(pool: web::Data<PgPool>, task_id: web::Path<i32>) -> Response<Task> {
    let id = task_id.into_inner();
    let _ = sqlx::query!(
        r#"
        DELETE FROM tasks
        WHERE id = $1
        RETURNING id, title, description, priority;
        "#,
        id
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to delete task with id {}: {}", id, e);
        actix_web::error::ErrorNotFound("Task not found")
    })?;

    let response = CustomResponseBuilder::new().build();
    Ok(response)
}
