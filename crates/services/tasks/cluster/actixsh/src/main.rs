//! The main entry point for the actix server
use core::net::Ipv4Addr;
use std::io;

use actix_web::{App, HttpServer};
use tactix::actions;
use task_dal::migrations::run_migrations as run_task_migrations;

#[tokio::main]
async fn main() -> io::Result<()> {
    run_task_migrations().await;

    HttpServer::new(|| App::new().configure(actions::views))
        .workers(4)
        .bind((Ipv4Addr::UNSPECIFIED, 8080))?
        .run()
        .await
}
