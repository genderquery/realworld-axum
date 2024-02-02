use axum::{debug_handler, extract::State, Json};
use serde::Serialize;

use crate::{error::AppError, AppState, PgPool};

#[debug_handler(state = AppState)]
pub async fn get_tags(State(pool): State<PgPool>) -> Result<Json<TagsResponse>, AppError> {
    todo!()
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TagsResponse {
    pub tags: Vec<String>,
}
