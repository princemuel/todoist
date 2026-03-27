//! Defines the error type passed between layers and servers.
use core::fmt;

use serde::{Deserialize, Serialize};

/// The error struct that is passed between layers and servers.
#[derive(Debug, Deserialize, Serialize, thiserror::Error)]
pub struct NanoServiceError {
    /// The error message.
    pub message: String,
    /// The status of the error.
    pub status:  NanoServiceErrorStatus,
}

impl NanoServiceError {
    /// Creates a new [`NanoServiceError`].
    #[must_use]
    pub fn new(message: String, status: NanoServiceErrorStatus) -> Self {
        Self { message, status }
    }
}

impl fmt::Display for NanoServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.message) }
}

/// The status of the error.
/// This is used to determine the HTTP status code to return.
#[derive(PartialEq, Eq, Debug, Deserialize, Serialize, thiserror::Error)]
pub enum NanoServiceErrorStatus {
    #[error("Bad Request")]
    BadRequest,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("You are forbidden to access the requested resource.")]
    Forbidden,
    #[error("The requested resource was not found")]
    NotFound,
    #[error("Conflict")]
    Conflict,
    #[error("Unknown Internal Error")]
    Unknown,
}

#[macro_export]
macro_rules! safe_eject {
    ($err:expr, $status:expr) => {
        $err.map_err(|x| NanoServiceError::new(x.to_string(), $status))
    };
    ($err:expr, $status:expr, $ctx:expr) => {
        $err.map_err(|x| NanoServiceError::new(format!("{}: {}", $ctx, x.to_string()), $status))
    };
}

#[cfg(feature = "actix")]
mod actix_impl {
    use actix_web::HttpResponse;
    use actix_web::error::ResponseError;
    use actix_web::http::StatusCode;

    use super::{NanoServiceError, NanoServiceErrorStatus};

    impl ResponseError for NanoServiceError {
        fn status_code(&self) -> StatusCode {
            match self.status {
                NanoServiceErrorStatus::BadRequest => StatusCode::BAD_REQUEST,
                NanoServiceErrorStatus::Unauthorized => StatusCode::UNAUTHORIZED,
                NanoServiceErrorStatus::Forbidden => StatusCode::FORBIDDEN,
                NanoServiceErrorStatus::NotFound => StatusCode::NOT_FOUND,
                NanoServiceErrorStatus::Conflict => StatusCode::CONFLICT,
                NanoServiceErrorStatus::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
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

    use super::{NanoServiceError, NanoServiceErrorStatus};

    /// Implementing the [`IntoResponse`] trait for Axum.
    impl IntoResponse for NanoServiceError {
        fn into_response(self) -> Response {
            let status = match self.status {
                NanoServiceErrorStatus::BadRequest => StatusCode::BAD_REQUEST,
                NanoServiceErrorStatus::Unauthorized => StatusCode::UNAUTHORIZED,
                NanoServiceErrorStatus::Forbidden => StatusCode::FORBIDDEN,
                NanoServiceErrorStatus::NotFound => StatusCode::NOT_FOUND,
                NanoServiceErrorStatus::Conflict => StatusCode::CONFLICT,
                NanoServiceErrorStatus::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
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

    use super::{NanoServiceError, NanoServiceErrorStatus};

    impl NanoServiceError {
        /// Converts the error into a Hyper HTTP response.
        ///
        /// # Panics
        /// Panics if the response builder fails to construct a valid response,
        /// or if the error message cannot be serialized to JSON.
        pub fn into_hyper_response(self) -> Response<Full<Bytes>> {
            let status = match self.status {
                NanoServiceErrorStatus::BadRequest => StatusCode::BAD_REQUEST,
                NanoServiceErrorStatus::Unauthorized => StatusCode::UNAUTHORIZED,
                NanoServiceErrorStatus::Forbidden => StatusCode::FORBIDDEN,
                NanoServiceErrorStatus::NotFound => StatusCode::NOT_FOUND,
                NanoServiceErrorStatus::Conflict => StatusCode::CONFLICT,
                NanoServiceErrorStatus::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            };

            let body = serde_json::to_string(&self.message).unwrap();
            Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .status(status)
                .body(Full::new(Bytes::from(body)))
                .unwrap()
        }
    }
}

#[cfg(feature = "rocket")]
mod rocket_impl {
    use std::io::Cursor;

    use rocket::http::Status;
    use rocket::request::Request;
    use rocket::response::{self, Responder, Response};

    use super::{NanoServiceError, NanoServiceErrorStatus};

    #[rocket::async_trait]
    impl<'r> Responder<'r, 'static> for NanoServiceError {
        fn respond_to(self, _req: &'r Request<'_>) -> response::Result<'static> {
            let status = match self.status {
                NanoServiceErrorStatus::BadRequest => Status::BadRequest,
                NanoServiceErrorStatus::Unauthorized => Status::Unauthorized,
                NanoServiceErrorStatus::Forbidden => Status::Forbidden,
                NanoServiceErrorStatus::NotFound => Status::NotFound,
                NanoServiceErrorStatus::Conflict => Status::Conflict,
                NanoServiceErrorStatus::Unknown => Status::InternalServerError,
            };

            Response::build()
                .status(status)
                .sized_body(self.message.len(), Cursor::new(self.message))
                .ok()
        }
    }
}
