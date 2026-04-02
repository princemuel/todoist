//! The entry point for the axum server.
use core::net::{Ipv4Addr, SocketAddr};

use axum::Router;
use task_dal::migrations::run_migrations;

mod actions;

#[tokio::main]
async fn main() {
    run_migrations().await;
    // Build our application with a route
    let app = actions::views(Router::new());

    // Specify the address and port to bind the
    // server
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
