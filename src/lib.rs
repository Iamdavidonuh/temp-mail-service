//! lib.rs

use actix_web::{dev::Server, App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(
        || App::new()
).listen(listener)?
        .run();
    Ok(server)
}