use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::validation::ValidationErrors;

pub enum AppError {
    ValidationErrors(ValidationErrors),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::ValidationErrors(errors) =>
            // TODO errors json
            {
                StatusCode::UNPROCESSABLE_ENTITY.into_response()
            }
        }
    }
}

impl From<ValidationErrors> for AppError {
    fn from(error: ValidationErrors) -> Self {
        AppError::ValidationErrors(error)
    }
}
