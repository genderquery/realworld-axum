use axum::{extract::State, Json};
use axum_macros::debug_handler;
use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    error::AppError,
    jwt::{self, Claims},
    password::{hash_password, verify_password},
    validation::{Validate, ValidationErrors},
    AppState,
};

#[debug_handler]
pub async fn register(
    State(state): State<AppState>,
    Json(user): Json<NewUser>,
) -> Result<(StatusCode, Json<User>), AppError> {
    user.validate()?;
    validate_unique_username(&state, &user.username)?;
    validate_unique_email(&state, &user.email)?;

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
    user.validate()?;

    let (username, email, password_hash) =
        match state.db.read().unwrap().get(&user.username).cloned() {
            Some(user) => user,
            None => return Err(AppError::Unauthorized),
        };

    if verify_password(&user.password, &password_hash).is_err() {
        return Err(AppError::Unauthorized);
    }

    let token = jwt::create_token(&state.jwt, &username, &email)?;

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

#[debug_handler]
pub async fn update_user(
    State(state): State<AppState>,
    claims: Claims,
    Json(update_user): Json<UpdateUser>,
) -> Result<Json<User>, AppError> {
    update_user.validate()?;

    if let Some(username) = update_user.username.as_ref() {
        validate_unique_username(&state, username)?;
    }
    if let Some(email) = update_user.email.as_ref() {
        validate_unique_email(&state, email)?;
    }

    let mut db = state.db.write().unwrap();
    let (username, email, password_hash) = db.get_mut(&claims.username).unwrap();

    if let Some(new_username) = update_user.username.as_ref() {
        *username = new_username.to_owned();
    }
    if let Some(new_email) = update_user.email.as_ref() {
        *email = new_email.to_owned();
    }
    if let Some(new_password) = update_user.password.as_ref() {
        *password_hash = hash_password(new_password)?;
    }

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

impl Validate for NewUser {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        validate_not_empty(&mut errors, "username", &self.username);
        validate_not_empty(&mut errors, "email", &self.email);
        validate_not_empty(&mut errors, "password", &self.password);

        errors.is_empty().then_some(()).ok_or(errors)
    }
}

fn validate_unique_username(state: &AppState, username: &str) -> Result<(), ValidationErrors> {
    let mut errors = ValidationErrors::new();

    if state.db.read().unwrap().contains_key(username) {
        errors.add("username", "has already been taken");
    }

    errors.is_empty().then_some(()).ok_or(errors)
}

fn validate_unique_email(state: &AppState, email: &str) -> Result<(), ValidationErrors> {
    let mut errors = ValidationErrors::new();

    let db = state.db.read().unwrap();
    if db.values().any(|(_, e, _)| e == email) {
        errors.add("email", "has already been taken");
    }

    errors.is_empty().then_some(()).ok_or(errors)
}

fn validate_not_empty(errors: &mut ValidationErrors, field: &'static str, value: &str) {
    if value.is_empty() {
        errors.add(field, "can't be blank");
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    username: String,
    password: String,
}

impl Validate for LoginUser {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        validate_not_empty(&mut errors, "username", &self.username);
        validate_not_empty(&mut errors, "password", &self.password);

        errors.is_empty().then_some(()).ok_or(errors)
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    username: Option<String>,
    email: Option<String>,
    password: Option<String>,
}

impl Validate for UpdateUser {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if let Some(username) = self.username.as_ref() {
            validate_not_empty(&mut errors, "username", username);
        }
        if let Some(email) = self.email.as_ref() {
            validate_not_empty(&mut errors, "email", email);
        }
        if let Some(password) = self.password.as_ref() {
            validate_not_empty(&mut errors, "password", password);
        }

        errors.is_empty().then_some(()).ok_or(errors)
    }
}

#[derive(Debug, Serialize)]
pub struct User {
    username: String,
    email: String,
    token: String,
}
