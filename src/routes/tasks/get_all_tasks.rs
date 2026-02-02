use actix_web::{HttpResponse, web};
use sqlx::PgPool;

// TODO: Separation of concerns: move DB logic to a separate module
pub async fn get_all_tasks(pool: web::Data<PgPool>) -> Result<HttpResponse, actix_web::Error> {
    let rows = sqlx::query!(r#"SELECT * FROM tasks;"#)
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Failed to fetch tasks: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to fetch tasks")
        })?;
    let tasks: Vec<_> = rows
        .into_iter()
        .map(|row| {
            serde_json::json!({
                "id": row.id,
                "title": row.title,
                "description": row.description,
                "priority": row.priority,
            })
        })
        .collect();
    Ok(HttpResponse::Ok().json(tasks))
}
