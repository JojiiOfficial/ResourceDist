use crate::api_types::error::ErrorResponse;
use actix_web::{error::BlockingError, http::StatusCode, HttpResponse, ResponseError};
use std::fmt::Debug;

/// API error that gets send to the user
#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    #[error("Internal server error")]
    Internal,
    #[error("Not found")]
    NotFound,
    #[error("Bad request")]
    BadRequest,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("Ratelimit")]
    Ratelimit,
}

impl ResponseError for Error {
    #[inline]
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::BadRequest => StatusCode::BAD_REQUEST,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::Forbidden => StatusCode::FORBIDDEN,
            Error::Ratelimit => StatusCode::TOO_MANY_REQUESTS,
        }
    }

    #[inline]
    fn error_response(&self) -> HttpResponse {
        let error = self.name().to_string();
        let msg = self.to_string();

        let err_resp = ErrorResponse::new(error, msg);
        let status_code = self.status_code();
        HttpResponse::build(status_code).json(err_resp)
    }
}

impl Error {
    #[inline]
    fn name(&self) -> &str {
        match self {
            Error::Internal => "Internal",
            Error::NotFound => "NotFound",
            Error::BadRequest => "BadRequest",
            Error::Unauthorized => "Unauthorized",
            Error::Forbidden => "Forbidden",
            Error::Ratelimit => "Ratelimit",
        }
    }
}

#[inline]
pub fn map_internal<T: Debug>(err: T) -> Error {
    log::error!("FATAL INTERNAL ERROR: {err:#?}");
    Error::Internal
}

impl From<BlockingError> for Error {
    #[inline]
    fn from(_: BlockingError) -> Self {
        Self::Internal
    }
}

impl From<std::io::Error> for Error {
    #[inline]
    fn from(e: std::io::Error) -> Self {
        log::error!("io: {e:?}");
        Self::Internal
    }
}
