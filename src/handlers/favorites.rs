use axum::{debug_handler, extract::Path, Json};

use crate::error::AppError;

use super::articles::SingleArticleResponse;

#[debug_handler]
pub async fn favorite(Path(slug): Path<String>) -> Result<Json<SingleArticleResponse>, AppError> {
    todo!()
}

#[debug_handler]
pub async fn unfavorite(Path(slug): Path<String>) -> Result<Json<SingleArticleResponse>, AppError> {
    todo!()
}
