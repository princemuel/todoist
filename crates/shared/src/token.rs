/// The houses data from the token in the
/// header.
#[derive(Clone, Debug)]
pub struct AuthToken {
    /// The token from the header
    pub message: String,
}

impl AuthToken {
    #[must_use]
    pub fn new(message: String) -> Self { Self { message } }
}

#[cfg(feature = "actix")]
mod actix_impl {
    pub use actix_web::FromRequest as ActixFromRequest;
    use actix_web::HttpRequest;
    use actix_web::dev::Payload;
    use futures::future::{Ready, err, ok};

    use super::AuthToken;
    use crate::errors::{Error, ErrorStatus};

    impl ActixFromRequest for AuthToken {
        type Error = Error;
        type Future = Ready<Result<Self, Error>>;

        fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
            let Some(raw_data) = req.headers().get("token") else {
                return err(Error {
                    status: ErrorStatus::Unauthorized,
                    message: "Token not found in header under key 'token'".to_owned(),
                });
            };

            let message = match raw_data.to_str() {
                Ok(token) => token.to_owned(),
                Err(_) => {
                    return err(Error {
                        status: ErrorStatus::Unauthorized,
                        message: "token not a valid string".to_owned(),
                    });
                }
            };

            ok(Self::new(message))
        }
    }
}

#[cfg(feature = "axum")]
mod axum_impl {
    pub use axum::extract::FromRequestParts as AxumFromRequestParts;
    use axum::http::request::Parts;

    use super::AuthToken;
    use crate::errors::{Error, ErrorStatus};

    impl<S> AxumFromRequestParts<S> for AuthToken
    where
        S: Send + Sync,
    {
        type Rejection = Error;

        async fn from_request_parts(
            parts: &mut Parts,
            _state: &S,
        ) -> Result<Self, Self::Rejection> {
            let raw_data = parts.headers.get("token").ok_or_else(|| Error {
                status: ErrorStatus::Unauthorized,
                message: "Token not found in header under key 'token'".to_owned(),
            })?;

            let message = raw_data
                .to_str()
                .map_err(|e| Error::new(e.to_string(), ErrorStatus::Unauthorized))?
                .to_owned();

            Ok(Self::new(message))
        }
    }
}

#[cfg(feature = "rocket")]
mod rocket_impl {
    use rocket::http::Status;
    use rocket::outcome::Outcome;
    use rocket::request;
    pub use rocket::request::FromRequest as RocketFromRequest;
    use rocket::request::Request;

    use super::AuthToken;
    use crate::errors::{Error, ErrorStatus};

    #[rocket::async_trait]
    impl<'r> RocketFromRequest<'r> for AuthToken {
        type Error = Error;

        async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
            match request.headers().get_one("token") {
                Some(token) => Outcome::Success(Self::new(token.to_owned())),
                None => Outcome::Error((Status::Unauthorized, Error {
                    status: ErrorStatus::Unauthorized,
                    message: "token not in header under key 'token'".to_owned(),
                })),
            }
        }
    }
}

#[cfg(feature = "actix")]
pub use actix_impl::ActixFromRequest;
#[cfg(feature = "axum")]
pub use axum_impl::AxumFromRequestParts;
#[cfg(feature = "rocket")]
pub use rocket_impl::RocketFromRequest;
