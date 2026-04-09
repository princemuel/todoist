use core::net::Ipv4Addr;
use std::io;
use std::path::Path;

use actix_cors::Cors;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use rust_embed::RustEmbed;
use task_dal::migrations::run_migrations as run_task_migrations;

#[tokio::main]
async fn main() -> io::Result<()> {
    run_task_migrations().await;

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
            .configure(task_server::actions::views)
            .wrap(cors)
            .default_service(web::route().to(catch_all))
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8001))?
    .run()
    .await
}

fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../../site/dist/index.html"))
}

#[derive(RustEmbed)]
#[folder = "../site/dist"]
struct Assets;

fn serve_asset(path: &str) -> HttpResponse {
    let Some(file) = Path::new(path).file_name().and_then(|f| f.to_str()) else {
        return HttpResponse::BadRequest().body("404 Not Found");
    };

    match Assets::get(file) {
        Some(content) => HttpResponse::Ok()
            .content_type(mime_guess::from_path(file).first_or_octet_stream().as_ref())
            .append_header(("Cache-Control", "public, max-age=3600, must-revalidate"))
            .body(content.data),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

async fn catch_all(req: HttpRequest) -> impl Responder {
    if req.path().contains("/api/") {
        return HttpResponse::NotFound().finish();
    }

    if req.path().contains("site/dist") {
        return serve_asset(req.path());
    }

    let file_type = mime_guess::from_path(req.path())
        .first_raw()
        .unwrap_or("text/html");

    if !file_type.contains("text/html") {
        return serve_asset(req.path());
    }

    async { index() }.await
}
