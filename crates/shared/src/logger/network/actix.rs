use core::pin::Pin;
use core::task::{Context, Poll};

use actix_web::Error;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use futures_util::future::{Ready, ok};

/// A simple logging layer for Actix Web.
#[derive(Clone, Copy, Default, Debug)]
pub struct Logger;

impl<S, B> Transform<S, ServiceRequest> for Logger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Error = Error;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    type InitError = ();
    type Response = ServiceResponse<B>;
    type Transform = LoggingMiddleware<S>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoggingMiddleware { inner: service })
    }
}

#[derive(Clone, Debug)]
pub struct LoggingMiddleware<S> {
    inner: S,
}
impl<S, B> Service<ServiceRequest> for LoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
    type Response = ServiceResponse<B>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let future = self.inner.call(req);
        Box::pin(async move {
            let res = future.await?;
            let request_info = format!(
                "{} {} {}",
                res.request().method(),
                res.request().uri(),
                res.status().as_str()
            );
            tracing::info!("Request: {}", request_info);
            Ok(res)
        })
    }
}
