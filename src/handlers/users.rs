use axum::Json;
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};

use crate::{error::AppError, jwt::Claims};

#[debug_handler]
pub async fn register(Json(payload): Json<NewUserRequest>) -> Result<Json<UserResponse>, AppError> {
    // TODO: validate payload
    // TODO: check if username/email exists
    // TODO: generate token
    todo!()
}

#[debug_handler]
pub async fn login(Json(payload): Json<LoginUserRequest>) -> Result<Json<UserResponse>, AppError> {
    todo!()
}

#[debug_handler]
pub async fn current_user() -> Result<Json<UserResponse>, AppError> {
    todo!()
}

#[debug_handler]
pub async fn update(
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
    user: NewUser,
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
