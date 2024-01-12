use axum::{
    debug_handler,
    extract::{Path, State},
    Json,
};
use sqlx::PgPool;

use crate::error::AppError;

use super::articles::SingleArticleResponse;

#[debug_handler]
pub async fn favorite(
    State(pool): State<PgPool>,
    Path(slug): Path<String>,
) -> Result<Json<SingleArticleResponse>, AppError> {
    todo!()
}

#[debug_handler]
pub async fn unfavorite(
    State(pool): State<PgPool>,
    Path(slug): Path<String>,
) -> Result<Json<SingleArticleResponse>, AppError> {
    todo!()
}
