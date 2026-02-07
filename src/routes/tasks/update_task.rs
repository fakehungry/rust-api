use actix_web::web::{self, Json};
use sqlx::PgPool;

use crate::{
    models::{Task, TaskFormData},
    utils::{CustomResponseBuilder, Response},
};

pub async fn update_task(
    pool: web::Data<PgPool>,
    task_id: web::Path<i32>,
    form: web::Json<TaskFormData>,
) -> Response<Json<Task>> {
    let id = task_id.into_inner();
    let row = sqlx::query!(
        r#"
        UPDATE tasks
        SET title = $1, description = $2, priority = $3, updated_at = NOW()
        WHERE id = $4
        RETURNING id, title, description, priority;
        "#,
        form.title,
        form.description,
        form.priority,
        id
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let task = Task {
        id: row.id,
        title: row.title,
        description: row.description,
        priority: Some(row.priority),
    };

    let response = CustomResponseBuilder::new().body(Json(task)).build();

    Ok(response)
}
