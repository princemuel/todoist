use core::convert::Infallible;
use core::net::{Ipv4Addr, SocketAddr};

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, body};
use hyper_util::rt::TokioIo;
use tokio::{net, task};

mod actions;

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

/// Handles an incoming request and routes it to the appropriate handler.
///
/// # Arguments
/// * `req` - The incoming request
///
/// # Returns
/// A `Result` containing the response to the request or an error
async fn handle(req: Request<body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    let path = req.uri().path();
    let response = match (req.method(), path) {
        (&Method::GET, "/api/v1/tasks") => actions::get::get_all().await,
        _ => Ok(not_found()),
    };
    match response {
        Ok(value) => Ok(value),
        Err(err) => Ok(err.into_hyper_response()),
    }
}

/// Returns a 404 Not Found response.
fn not_found() -> Response<Full<Bytes>> {
    Response::builder()
        .status(404)
        .body(Full::new(Bytes::from("Not Found")))
        .unwrap()
}
