use std::net::TcpListener;

use img_service::startup::run;
use img_service::configuration::get_configuration;
use img_service::telemetry::*;
use secrecy::ExposeSecret;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let listener = TcpListener::bind("127.0.0.1:8000")
        .expect("Failed to bind rand port");
    
    let connection = PgPool::connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");
    run(listener, connection)?.await
}