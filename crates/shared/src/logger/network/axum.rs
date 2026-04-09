//! -----------------------------------------------------------------------------
//! Lightweight logging middleware for Axum, mirroring the Actix example you
//! posted. Logs the request method, URI, and response status with `tracing`.
//! -----------------------------------------------------------------------------
//! Usage
//! -----------------------------------------------------------------------------
//! ```
//! use axum::Router;
//! use axum::routing::get;
//! use axum_logger::Logger;
//!
//! #[tokio::main]
//! async fn main() {
//!     tracing_subscriber::fmt().init();
//!
//!     let app = Router::new()
//!         .route("/", get(|| async { "Hello Axum" }))
//!         .layer(Logger); // <- plug it in here
//!
//!     axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
//!         .serve(app.into_make_service())
//!         .await
//!         .unwrap();
//! }
//! ```
//! -----------------------------------------------------------------------------
use core::task::{Context, Poll};

use axum::body::Body;
use axum::http::{Request, Response};
use futures::future::BoxFuture;
use tower::{Layer, Service};

/// A simple logging layer for Axum built on Tower.
#[derive(Clone, Copy, Default, Debug)]
pub struct Logger;
impl<S> Layer<S> for Logger {
    type Service = LoggingMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service { LoggingMiddleware { inner } }
}

/// The service produced by [`Logger`].
#[derive(Clone, Debug)]
pub struct LoggingMiddleware<S> {
    inner: S,
}

impl<S, ReqBody> Service<Request<ReqBody>> for LoggingMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response<Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Send + Sync + 'static,
    ReqBody: Send + 'static,
{
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Response = Response<Body>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let uri = req.uri().clone();
        let method = req.method().clone();
        let future = self.inner.call(req);

        Box::pin(async move {
            let res = future.await?;
            tracing::info!("Request: {method} {uri} {}", res.status());
            Ok(res)
        })
    }
}
