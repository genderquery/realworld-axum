use axum::{extract::State, Json};
use axum_macros::debug_handler;
use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    error::AppError,
    jwt::{self, Claims},
    password::{hash_password, verify_password},
    AppState,
};

#[debug_handler]
pub async fn register(
    State(state): State<AppState>,
    Json(user): Json<NewUser>,
) -> Result<(StatusCode, Json<User>), AppError> {
    if state.db.read().unwrap().contains_key(&user.username) {
        return Err(AppError::UsernameInUse);
    }

    let password_hash = hash_password(&user.password)?;

    state.db.write().unwrap().insert(
        user.username.clone(),
        (
            user.username.clone(),
            user.email.clone(),
            password_hash.clone(),
        ),
    );

    let token = jwt::create_token(&state.jwt, &user.username, &user.email)?;

    Ok((
        StatusCode::CREATED,
        Json(User {
            username: user.username,
            email: user.email,
            token,
        }),
    ))
}

#[debug_handler]
pub async fn login(
    State(state): State<AppState>,
    Json(user): Json<LoginUser>,
) -> Result<Json<User>, AppError> {
    let db = state.db.read().unwrap();

    let (username, email, password_hash) = match db.get(&user.username) {
        Some(user) => user,
        None => return Err(AppError::Unauthorized),
    };

    if verify_password(&user.password, password_hash).is_err() {
        return Err(AppError::Unauthorized);
    }

    let token = jwt::create_token(&state.jwt, username, email)?;

    Ok(Json(User {
        username: username.to_owned(),
        email: email.to_owned(),
        token,
    }))
}

#[debug_handler]
pub async fn get_current_user(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<User>, AppError> {
    let db = state.db.read().unwrap();
    let (username, email, _) = match db.get(&claims.username) {
        Some(user) => user,
        None => return Err(AppError::Unauthorized),
    };

    let token = jwt::create_token(&state.jwt, username, email)?;

    Ok(Json(User {
        username: username.to_owned(),
        email: email.to_owned(),
        token,
    }))
}

#[derive(Debug, Deserialize)]
pub struct NewUser {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct User {
    username: String,
    email: String,
    token: String,
}
