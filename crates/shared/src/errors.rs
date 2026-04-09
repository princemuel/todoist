//! Defines the error type passed between layers
//! and servers.
use core::fmt;

use serde::{Deserialize, Serialize};

/// The error struct that is passed between
/// layers and servers.
#[derive(Clone, Debug, Deserialize, Serialize, thiserror::Error)]
pub struct Error {
    /// The error message.
    pub message: String,
    /// The status of the error.
    pub status: ErrorStatus,
}

impl Error {
    /// Creates a new [`Error`].
    #[must_use]
    pub fn new(message: String, status: ErrorStatus) -> Self { Self { message, status } }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.message) }
}

/// The status of the error.
/// This is used to determine the HTTP status code to return.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize, Serialize, thiserror::Error)]
pub enum ErrorStatus {
    /// 400 Bad Request: The request was malformed or invalid
    #[error("Bad Request")]
    BadRequest,
    /// 401 Unauthorized: Authentication required or failed
    #[error("Unauthorized")]
    Unauthorized,
    /// 403 Forbidden: Authenticated but lacks permission
    #[error("You are forbidden to access the requested resource.")]
    Forbidden,
    /// 404 Not Found: Resource does not exist
    #[error("The requested resource was not found")]
    NotFound,
    /// 409 Conflict: Request conflicts with existing resource state
    #[error("Conflict")]
    Conflict,
    /// 422 Unprocessable Entity: Validation error or semantic issue
    #[error("Unprocessable Entity: The request cannot be processed due to semantic errors")]
    UnprocessableEntity,
    /// 429 Too Many Requests: Rate limit exceeded
    #[error("Too Many Requests: Rate limit exceeded")]
    TooManyRequests,
    /// 500 Internal Server Error: Server error
    #[error("Internal Server Error")]
    InternalServerError,
    /// 502 Bad Gateway: Invalid response from upstream service
    #[error("Bad Gateway: Invalid response from upstream service")]
    BadGateway,
    /// 503 Service Unavailable: Server temporarily unable to handle request
    #[error("Service Unavailable: The service is temporarily unavailable")]
    ServiceUnavailable,
    /// 504 Gateway Timeout: Upstream service timeout
    #[error("Gateway Timeout: The upstream service did not respond in time")]
    GatewayTimeout,
    /// Unknown or unmapped error
    #[error("Unknown Internal Error")]
    Unknown,
}

impl From<u16> for ErrorStatus {
    fn from(code: u16) -> Self {
        match code {
            400 => Self::BadRequest,
            401 => Self::Unauthorized,
            403 => Self::Forbidden,
            404 => Self::NotFound,
            409 => Self::Conflict,
            422 => Self::UnprocessableEntity,
            429 => Self::TooManyRequests,
            500 => Self::InternalServerError,
            502 => Self::BadGateway,
            503 => Self::ServiceUnavailable,
            504 => Self::GatewayTimeout,
            _ => Self::Unknown,
        }
    }
}

#[macro_export]
macro_rules! safe_eject {
    ($e:expr, $err_status:expr) => {
        $e.map_err(|x| Error::new(x.to_string(), $err_status))
    };
    ($e:expr, $err_status:expr, $ctx:expr) => {
        $e.map_err(|x| Error::new(format!("{}: {}", $ctx, x.to_string()), $err_status))
    };
}

#[cfg(feature = "actix")]
mod actix_impl {
    use actix_web::HttpResponse;
    use actix_web::error::ResponseError;
    use actix_web::http::StatusCode;

    use super::{Error, ErrorStatus};

    impl ResponseError for Error {
        fn status_code(&self) -> StatusCode {
            match self.status {
                ErrorStatus::BadRequest => StatusCode::BAD_REQUEST,
                ErrorStatus::Unauthorized => StatusCode::UNAUTHORIZED,
                ErrorStatus::Forbidden => StatusCode::FORBIDDEN,
                ErrorStatus::NotFound => StatusCode::NOT_FOUND,
                ErrorStatus::Conflict => StatusCode::CONFLICT,
                ErrorStatus::UnprocessableEntity => StatusCode::UNPROCESSABLE_ENTITY,
                ErrorStatus::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
                ErrorStatus::InternalServerError | ErrorStatus::Unknown => {
                    StatusCode::INTERNAL_SERVER_ERROR
                }
                ErrorStatus::BadGateway => StatusCode::BAD_GATEWAY,
                ErrorStatus::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
                ErrorStatus::GatewayTimeout => StatusCode::GATEWAY_TIMEOUT,
            }
        }

        fn error_response(&self) -> HttpResponse {
            let status = self.status_code();
            HttpResponse::build(status).json(&self.message)
        }
    }
}

#[cfg(feature = "axum")]
mod axum_impl {
    use axum::Json;
    use axum::http::StatusCode;
    use axum::response::{IntoResponse, Response};

    use super::{Error, ErrorStatus};

    /// Implementing the [`IntoResponse`] trait
    /// for Axum.
    impl IntoResponse for Error {
        fn into_response(self) -> Response {
            let status = match self.status {
                ErrorStatus::BadRequest => StatusCode::BAD_REQUEST,
                ErrorStatus::Unauthorized => StatusCode::UNAUTHORIZED,
                ErrorStatus::Forbidden => StatusCode::FORBIDDEN,
                ErrorStatus::NotFound => StatusCode::NOT_FOUND,
                ErrorStatus::Conflict => StatusCode::CONFLICT,
                ErrorStatus::UnprocessableEntity => StatusCode::UNPROCESSABLE_ENTITY,
                ErrorStatus::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
                ErrorStatus::InternalServerError | ErrorStatus::Unknown => {
                    StatusCode::INTERNAL_SERVER_ERROR
                }
                ErrorStatus::BadGateway => StatusCode::BAD_GATEWAY,
                ErrorStatus::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
                ErrorStatus::GatewayTimeout => StatusCode::GATEWAY_TIMEOUT,
            };

            (status, Json(self.message)).into_response()
        }
    }
}

#[cfg(feature = "hyper")]
mod hyper_impl {
    use http_body_util::Full;
    use hyper::body::Bytes;
    use hyper::{Response, StatusCode, header};

    use super::{Error, ErrorStatus};

    impl Error {
        /// Converts the error into a Hyper HTTP response.
        ///
        /// # Panics
        /// Panics if the response builder fails to construct a valid response,
        /// or if the error message cannot be serialized to JSON.
        pub fn into_response(self) -> Response<Full<Bytes>> {
            let status = match self.status {
                ErrorStatus::BadRequest => StatusCode::BAD_REQUEST,
                ErrorStatus::Unauthorized => StatusCode::UNAUTHORIZED,
                ErrorStatus::Forbidden => StatusCode::FORBIDDEN,
                ErrorStatus::NotFound => StatusCode::NOT_FOUND,
                ErrorStatus::Conflict => StatusCode::CONFLICT,
                ErrorStatus::UnprocessableEntity => StatusCode::UNPROCESSABLE_ENTITY,
                ErrorStatus::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
                ErrorStatus::InternalServerError | ErrorStatus::Unknown => {
                    StatusCode::INTERNAL_SERVER_ERROR
                }
                ErrorStatus::BadGateway => StatusCode::BAD_GATEWAY,
                ErrorStatus::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
                ErrorStatus::GatewayTimeout => StatusCode::GATEWAY_TIMEOUT,
            };

            #[expect(clippy::expect_used)]
            let body = serde_json::to_string(&self.message).expect("invalid error message");
            #[expect(clippy::expect_used)]
            Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .status(status)
                .body(Full::new(Bytes::from(body)))
                .expect("Failed to parse response body")
        }
    }
}

#[cfg(feature = "rocket")]
mod rocket_impl {
    use std::io::Cursor;

    use rocket::http::Status;
    use rocket::request::Request;
    use rocket::response::{self, Responder, Response};

    use super::{Error, ErrorStatus};

    #[rocket::async_trait]
    impl<'r> Responder<'r, 'static> for Error {
        fn respond_to(self, _req: &'r Request<'_>) -> response::Result<'static> {
            let status = match self.status {
                ErrorStatus::BadRequest => Status::BadRequest,
                ErrorStatus::Unauthorized => Status::Unauthorized,
                ErrorStatus::Forbidden => Status::Forbidden,
                ErrorStatus::NotFound => Status::NotFound,
                ErrorStatus::Conflict => Status::Conflict,
                ErrorStatus::UnprocessableEntity => Status::UnprocessableEntity,
                ErrorStatus::TooManyRequests => Status::TooManyRequests,
                ErrorStatus::InternalServerError | ErrorStatus::Unknown => {
                    Status::InternalServerError
                }
                ErrorStatus::BadGateway => Status::BadGateway,
                ErrorStatus::ServiceUnavailable => Status::ServiceUnavailable,
                ErrorStatus::GatewayTimeout => Status::GatewayTimeout,
            };

            Response::build()
                .status(status)
                .sized_body(self.message.len(), Cursor::new(self.message))
                .ok()
        }
    }
}
