use actix_web::{
    http::StatusCode,
    web::{self},
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    models::Task,
    utils::{CustomResponseBuilder, Response},
};

#[derive(Deserialize)]
pub struct FormData {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
}

pub async fn create_task(pool: web::Data<PgPool>, form: web::Json<FormData>) -> Response<Task> {
    let result = sqlx::query!(
        r#"
        INSERT INTO tasks (title, description, priority)
        VALUES ($1, $2, $3)
        RETURNING id, title, description, priority;
        "#,
        form.title,
        form.description,
        form.priority
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let task = Task {
        id: result.id,
        title: result.title,
        description: result.description,
        priority: Some(result.priority),
    };

    let response = CustomResponseBuilder::new()
        .body(task)
        .status_code(StatusCode::CREATED)
        .build();

    Ok(response)
}
