use std::error::Error as StdError;

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use serde_json::json;

use crate::{jwt::TokenError, validation::ValidationErrors};

#[derive(Debug)]
#[non_exhaustive]
pub enum AppError {
    InternalServerError(Box<dyn StdError>),
    Unauthorized,
    ValidationErrors(ValidationErrors),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::InternalServerError(error) => {
                tracing::error!("{error}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            AppError::Unauthorized => StatusCode::UNAUTHORIZED.into_response(),
            AppError::ValidationErrors(errors) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({
                    "errors": errors
                })),
            )
                .into_response(),
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

impl From<ValidationErrors> for AppError {
    fn from(error: ValidationErrors) -> Self {
        AppError::ValidationErrors(error)
    }
}
