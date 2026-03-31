//! Defines the error type passed between layers
//! and servers.
use core::fmt;

use serde::{Deserialize, Serialize};

/// The error struct that is passed between
/// layers and servers.
#[derive(Debug, Deserialize, Serialize, thiserror::Error)]
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
/// This is used to determine the HTTP status
/// code to return.
#[derive(PartialEq, Eq, Debug, Deserialize, Serialize, thiserror::Error)]
pub enum ErrorStatus {
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
                ErrorStatus::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
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
                ErrorStatus::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
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
        /// Converts the error into a Hyper HTTP
        /// response.
        ///
        /// # Panics
        /// Panics if the response builder fails
        /// to construct a valid response,
        /// or if the error message cannot be
        /// serialized to JSON.
        pub fn into_hyper_response(self) -> Response<Full<Bytes>> {
            let status = match self.status {
                ErrorStatus::BadRequest => StatusCode::BAD_REQUEST,
                ErrorStatus::Unauthorized => StatusCode::UNAUTHORIZED,
                ErrorStatus::Forbidden => StatusCode::FORBIDDEN,
                ErrorStatus::NotFound => StatusCode::NOT_FOUND,
                ErrorStatus::Conflict => StatusCode::CONFLICT,
                ErrorStatus::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            };

            #[allow(clippy::expect_used)]
            let body = serde_json::to_string(&self.message).expect("invalid error message");
            #[allow(clippy::unwrap_used)]
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
                ErrorStatus::Unknown => Status::InternalServerError,
            };

            Response::build()
                .status(status)
                .sized_body(self.message.len(), Cursor::new(self.message))
                .ok()
        }
    }
}
