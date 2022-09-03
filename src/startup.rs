use std::net::TcpListener;
use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;

use crate::routes::*;

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new( || {
        App::new()
            // .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
            // .route("/{name}", web::get().to(greet))
        })
        .listen(listener)?
        .run();
    Ok(server)
}
