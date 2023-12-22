use std::error::Error as StdError;

use axum::response::{IntoResponse, Response};
use http::StatusCode;

use crate::jwt::TokenError;

#[derive(Debug)]
#[non_exhaustive]
pub enum AppError {
    InternalServerError(Box<dyn StdError>),
    Unauthorized,
    UsernameInUse,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::InternalServerError(error) => {
                tracing::error!("{error}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            AppError::Unauthorized => StatusCode::UNAUTHORIZED.into_response(),
            AppError::UsernameInUse => StatusCode::UNPROCESSABLE_ENTITY.into_response(),
        }
    }
}

impl From<TokenError> for AppError {
    fn from(error: TokenError) -> Self {
        AppError::InternalServerError(error.into())
    }
}

impl From<password_hash::Error> for AppError {
    fn from(error: password_hash::Error) -> Self {
        AppError::InternalServerError(error.into())
    }
}
