use std::net::TcpListener;

use img_service::startup::run;
use img_service::configuration::get_configuration;
use img_service::telemetry::*;
use secrecy::ExposeSecret;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let addr = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(addr)
        .expect("Failed to bind rand port");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());
        // .connect_lazy(&configuration.database.connection_string().expose_secret())
        // .expect("Failed to create Postgres connection pool");
    run(listener, connection_pool)?.await
}