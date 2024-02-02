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
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: String,
    pub image: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewUserRequest {
    pub user: NewUser,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginUserRequest {
    pub user: LoginUser,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRequest {
    pub user: UpdateUser,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
}
