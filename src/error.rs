use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
