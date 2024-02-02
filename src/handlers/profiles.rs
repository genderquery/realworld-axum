use axum::{
    debug_handler,
    extract::{Path, State},
    Json,
};
use diesel::QueryDsl;
use serde::Serialize;

use crate::{error::AppError, jwt::Claims, schema, AppState, PgPool};

#[debug_handler(state = AppState)]
pub async fn get_profile(
    State(pool): State<PgPool>,
    maybe_claims: Option<Claims>,
    Path(username): Path<String>,
) -> Result<Json<ProfileResponse>, AppError> {
    todo!()
}

#[debug_handler(state = AppState)]
pub async fn follow(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(username): Path<String>,
) -> Result<Json<ProfileResponse>, AppError> {
    todo!()
}

#[debug_handler(state = AppState)]
pub async fn unfollow(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(username): Path<String>,
) -> Result<Json<ProfileResponse>, AppError> {
    todo!()
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileResponse {
    pub profile: Profile,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub username: String,
    pub bio: String,
    pub image: String,
    pub following: bool,
}
