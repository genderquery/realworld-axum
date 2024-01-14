use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum AppError {
    InternalServerError(Box<dyn std::error::Error>),
    Unauthorized,
    NotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::InternalServerError(error) => {
                tracing::error!("{}", error);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            AppError::Unauthorized => StatusCode::UNAUTHORIZED.into_response(),
            AppError::NotFound => StatusCode::NOT_FOUND.into_response(),
        }
    }
}

impl From<password_hash::Error> for AppError {
    fn from(value: password_hash::Error) -> Self {
        AppError::InternalServerError(value.into())
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        AppError::InternalServerError(value.into())
    }
}
