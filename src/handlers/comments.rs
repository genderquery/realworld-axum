use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};

use super::profiles::Profile;

#[debug_handler]
pub async fn get_comments() {
    todo!()
}

#[debug_handler]
pub async fn create_comment() {
    todo!()
}

#[debug_handler]
pub async fn delete_comment() {
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
