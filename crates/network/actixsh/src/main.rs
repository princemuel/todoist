//! The main entry point for the actix server
use core::net::Ipv4Addr;
use std::io;

use actix_web::{App, HttpRequest, HttpServer, Responder, web};

mod actions;

#[tokio::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().configure(actions::views))
        .workers(4)
        .bind((Ipv4Addr::UNSPECIFIED, 8080))?
        .run()
        .await
}

/// A basic handler that recieves a request and returns a greeting.
///
/// # Arguments
/// * `req` - The request object
///
/// # Returns
/// A string response
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}
