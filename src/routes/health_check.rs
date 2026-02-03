use serde::Serialize;

use crate::utils::{CustomResponseBuilder, Response};

#[derive(Serialize)]
pub struct HealthCheckResponse {
    name: &'static str,
    version: &'static str,
}

pub async fn health_check() -> Response<HealthCheckResponse> {
    const NAME: &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let response = HealthCheckResponse {
        name: NAME,
        version: VERSION,
    };
    Ok(CustomResponseBuilder::new().body(response).build())
}
