use std::sync::Arc;

use axum::{debug_handler, extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    error::AppError,
    jwt::{Claims, Jwt},
    password::{hash_password, verify_password},
    AppState, PgPool,
};

#[debug_handler(state = AppState)]
pub async fn register(
    State(pool): State<PgPool>,
    State(jwt): State<Jwt>,
    Json(payload): Json<NewUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    todo!()
}

#[debug_handler(state = AppState)]
pub async fn login(
    State(pool): State<PgPool>,
    State(jwt): State<Jwt>,
    Json(payload): Json<LoginUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    todo!()
}

#[debug_handler(state = AppState)]
pub async fn current_user(
    State(pool): State<PgPool>,
    State(jwt): State<Jwt>,
    claims: Claims,
) -> Result<Json<UserResponse>, AppError> {
    todo!()
}

#[debug_handler(state = AppState)]
pub async fn update(
    State(pool): State<PgPool>,
    State(jwt): State<Jwt>,
    claims: Claims,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    todo!()
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    user: User,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    email: String,
    token: String,
    username: String,
    bio: String,
    image: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewUserRequest {
    user: NewUser,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewUser {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginUserRequest {
    user: LoginUser,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginUser {
    email: String,
    password: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRequest {
    user: UpdateUser,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUser {
    username: Option<String>,
    email: Option<String>,
    password: Option<String>,
    bio: Option<String>,
    image: Option<String>,
}
