use axum::{debug_handler, Json};
use serde::Serialize;

use crate::error::AppError;

#[debug_handler]
pub async fn get_tags() -> Result<Json<TagsResponse>, AppError> {
    todo!()
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TagsResponse {
    tags: Vec<String>,
}
