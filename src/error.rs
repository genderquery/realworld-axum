use std::{collections::HashMap, error::Error as StdError};

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
                    "errors": validation_errors_to_json(errors)
                })),
            )
                .into_response(),
        }
    }
}

// API tests expect these specific errors messages
fn validation_errors_to_json(errors: ValidationErrors) -> HashMap<&'static str, Vec<&'static str>> {
    errors
        .0
        .iter()
        .map(|(field, errs)| {
            (
                *field,
                errs.iter()
                    .map(|err| match *err {
                        "not_empty" => "can't be blank",
                        "unique" => "has already been taken",
                        _ => err,
                    })
                    .collect(),
            )
        })
        .collect()
}

impl From<TokenError> for AppError {
    fn from(error: TokenError) -> Self {
        AppError::InternalServerError(error.into())
    }
}

impl From<password_hash::Error> for AppError {
    fn from(error: password_hash::Error) -> Self {
        match error {
            password_hash::Error::Password => AppError::Unauthorized,
            _ => AppError::InternalServerError(error.into()),
        }
    }
}

impl From<ValidationErrors> for AppError {
    fn from(error: ValidationErrors) -> Self {
        AppError::ValidationErrors(error)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        AppError::InternalServerError(error.into())
    }
}
