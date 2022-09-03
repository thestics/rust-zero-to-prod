use std::net::TcpListener;

use img_service::startup::run;
use img_service::configuration::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind rand port");
    run(listener)?.await
}