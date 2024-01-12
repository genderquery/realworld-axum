use axum::{debug_handler, extract::Path, Json};
use serde::{Deserialize, Serialize};

use crate::error::AppError;

use super::profiles::Profile;

#[debug_handler]
pub async fn get_comments(
    Path(slug): Path<String>,
) -> Result<Json<MultipleCommentsResponse>, AppError> {
    todo!()
}

#[debug_handler]
pub async fn create_comment(
    Path(slug): Path<String>,
) -> Result<Json<SingleCommentResponse>, AppError> {
    todo!()
}

#[debug_handler]
pub async fn delete_comment(Path((slug, id)): Path<(String, u32)>) -> Result<(), AppError> {
    todo!()
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCommentRequest {
    comment: NewComment,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewComment {
    body: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MultipleCommentsResponse {
    comments: Vec<Comment>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleCommentResponse {
    comment: Comment,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    id: u64,
    created_at: String,
    updated_at: String,
    body: String,
    author: Profile,
}
