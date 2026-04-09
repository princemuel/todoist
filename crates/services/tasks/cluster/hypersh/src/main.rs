use core::convert::Infallible;
use core::net::{Ipv4Addr, SocketAddr};

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, body};
use hyper_util::rt::TokioIo;
use shared::extract_hyper_header_token;
use shared::hyper_utils::extract_header::extract_token;
use task_dal::migrations::run_migrations;
use tokio::{net, task};
mod actions;
use task_dal::tasks::descriptors::SqlxPostgresDescriptor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    let listener = net::TcpListener::bind(addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(handle))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

/// Handles an incoming request and routes it to
/// the appropriate handler.
///
/// # Arguments
/// * `req` - The incoming request
///
/// # Returns
/// A `Result` containing the response to the
/// request or an error
async fn handle(req: Request<body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    run_migrations().await;

    let path = req.uri().path();
    // Split the path into segments
    let segments: Vec<_> = path.trim_start_matches('/').split('/').collect();

    let response = match (req.method(), segments.as_slice()) {
        (&Method::GET, ["api", "v1", "tasks"]) => {
            let token = extract_hyper_header_token!(&req);
            actions::get::get_all::<SqlxPostgresDescriptor>(token).await
        }
        (&Method::POST, ["api", "v1", "tasks"]) => {
            let token = extract_hyper_header_token!(&req);
            actions::create::create::<SqlxPostgresDescriptor>(req, token).await
        }
        (&Method::PATCH, ["api", "v1", "tasks"]) => {
            let token = extract_hyper_header_token!(&req);
            // Extract and parse the JSON body
            actions::update::update::<SqlxPostgresDescriptor>(req, token).await
        }

        (&Method::GET, ["api", "v1", "tasks", name]) => {
            let token = extract_hyper_header_token!(&req);
            actions::get::get_by_name::<SqlxPostgresDescriptor>(name, token).await
        }
        (&Method::DELETE, ["api", "v1", "tasks", name]) => {
            let token = extract_hyper_header_token!(&req);
            actions::delete::delete::<SqlxPostgresDescriptor>(name, token).await
        }
        _ => Ok(not_found()),
    };

    match response {
        Ok(value) => Ok(value),
        Err(err) => Ok(err.into_response()),
    }
}

/// Returns a 404 Not Found response.
fn not_found() -> Response<Full<Bytes>> {
    Response::builder()
        .status(404)
        .body(Full::new(Bytes::from("Not Found")))
        .unwrap()
}
