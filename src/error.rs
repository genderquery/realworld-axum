use axum::response::IntoResponse;

pub enum AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {}
    }
}
