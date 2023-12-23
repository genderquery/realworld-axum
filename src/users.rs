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

#[derive(Debug, Deserialize)]
pub struct NewUser {
    username: String,
    email: String,
    password: String,
}

impl Validate for NewUser {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if self.username.is_empty() {
            errors.add("username", "can't be blank");
        }

        if self.email.is_empty() {
            errors.add("email", "can't be blank");
        }

        if self.password.is_empty() {
            errors.add("password", "can't be blank");
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

fn validate_unique_username(state: &AppState, username: &str) -> Result<(), ValidationErrors> {
    let mut errors = ValidationErrors::new();

    if state.db.read().unwrap().contains_key(username) {
        errors.add("username", "has already been taken");
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_unique_email(state: &AppState, email: &str) -> Result<(), ValidationErrors> {
    let mut errors = ValidationErrors::new();

    if state
        .db
        .read()
        .unwrap()
        .values()
        .any(|(_, e, _)| e == email)
    {
        errors.add("email", "has already been taken");
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
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

        if self.username.is_empty() {
            errors.add("username", "can't be blank");
        }

        if self.password.is_empty() {
            errors.add("password", "can't be blank");
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[derive(Debug, Serialize)]
pub struct User {
    username: String,
    email: String,
    token: String,
}
