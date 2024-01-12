use axum::{
    debug_handler,
    extract::{Path, State},
    Json,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::error::AppError;

#[debug_handler]
pub async fn get_profile(
    State(pool): State<PgPool>,
    Path(username): Path<String>,
) -> Result<Json<ProfileResponse>, AppError> {
    todo!()
}

#[debug_handler]
pub async fn follow(
    State(pool): State<PgPool>,
    Path(username): Path<String>,
) -> Result<Json<ProfileResponse>, AppError> {
    todo!()
}

#[debug_handler]
pub async fn unfollow(
    State(pool): State<PgPool>,
    Path(username): Path<String>,
) -> Result<Json<ProfileResponse>, AppError> {
    todo!()
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileResponse {
    profile: Profile,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    username: String,
    bio: String,
    image: String,
    following: bool,
}
