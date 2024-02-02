use axum::{
    debug_handler,
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{error::AppError, jwt::Claims, AppState, PgPool};

use super::profiles::Profile;

#[debug_handler(state = AppState)]
pub async fn get_comments(
    State(pool): State<PgPool>,
    maybe_claims: Option<Claims>,
    Path(slug): Path<String>,
) -> Result<Json<MultipleCommentsResponse>, AppError> {
    todo!()
}

#[debug_handler(state = AppState)]
pub async fn create_comment(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(slug): Path<String>,
) -> Result<Json<SingleCommentResponse>, AppError> {
    todo!()
}

#[debug_handler(state = AppState)]
pub async fn delete_comment(
    State(pool): State<PgPool>,
    claims: Claims,
    Path((slug, id)): Path<(String, u32)>,
) -> Result<(), AppError> {
    todo!()
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCommentRequest {
    pub comment: NewComment,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewComment {
    pub body: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MultipleCommentsResponse {
    pub comments: Vec<Comment>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleCommentResponse {
    pub comment: Comment,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: u64,
    pub created_at: String,
    pub updated_at: String,
    pub body: String,
    pub author: Profile,
}
