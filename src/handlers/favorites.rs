use axum::{
    debug_handler,
    extract::{Path, State},
    Json,
};

use crate::{error::AppError, jwt::Claims, AppState, PgPool};

use super::articles::SingleArticleResponse;

#[debug_handler(state = AppState)]
pub async fn favorite(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(slug): Path<String>,
) -> Result<Json<SingleArticleResponse>, AppError> {
    todo!()
}

#[debug_handler(state = AppState)]
pub async fn unfavorite(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(slug): Path<String>,
) -> Result<Json<SingleArticleResponse>, AppError> {
    todo!()
}
