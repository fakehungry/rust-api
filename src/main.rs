use actix_web::{App, HttpServer, web};
use rust_api::routes::health_check;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/health-check", web::get().to(health_check)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
