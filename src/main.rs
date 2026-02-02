use std::net::TcpListener;

use actix_web::{
    App, HttpServer,
    dev::Server,
    web::{self, Data},
};
use rust_api::{
    configuration::{DatabaseSettings, Settings},
    routes::health_check,
};
use sqlx::{PgPool, postgres::PgPoolOptions};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);
        let address = format!("{}:{}", "localhost", 8080); // TODO: make port configurable
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, connection_pool).await?;

        Ok(Self { port, server })
    }
}

pub fn get_connection_pool(database_settings: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(database_settings.connect_options())
}

async fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, anyhow::Error> {
    let db_pool = Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health-check", web::get().to(health_check))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_config = DatabaseSettings {
        username: "hean".to_string(),
        password: "heankub22".into(),
        port: 5432,
        host: "localhost".to_string(),
        database_name: "rust_db".to_string(),
        require_ssl: false,
    };
    let application = Application::build(Settings {
        database: db_config,
    }) // TODO: Refine later
    .await
    .expect("Failed to build application.");
    application.server.await?;
    Ok(())
}
