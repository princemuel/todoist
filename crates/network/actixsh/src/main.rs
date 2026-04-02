//! The main entry point for the actix server
use core::net::Ipv4Addr;
use std::io;

use actix_web::{App, HttpServer};
use actixsh::actions;
use dal::migrations::run_migrations;

#[tokio::main]
async fn main() -> io::Result<()> {
    run_migrations().await;

    HttpServer::new(|| App::new().configure(actions::views))
        .workers(4)
        .bind((Ipv4Addr::UNSPECIFIED, 8080))?
        .run()
        .await
}
