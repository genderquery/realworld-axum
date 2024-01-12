use axum::{debug_handler, extract::State, Json};
use serde::Serialize;
use sqlx::PgPool;

use crate::error::AppError;

#[debug_handler]
pub async fn get_tags(State(pool): State<PgPool>) -> Result<Json<TagsResponse>, AppError> {
    todo!()
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TagsResponse {
    tags: Vec<String>,
}
