use std::net::TcpListener;

use img_service::startup::run;
use img_service::configuration::get_configuration;
use sqlx::{Connection, PgPool};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind rand port");
    
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    run(listener, connection)?.await
}