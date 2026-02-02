use actix_web::{HttpResponse, Responder};

pub async fn health_check() -> impl Responder {
    const NAME: &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    HttpResponse::Ok().body(format!("{} v{}", NAME, VERSION))
}
