use std::net::TcpListener;

use img_service::startup::run;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind rand port");
    run(listener)?.await
}