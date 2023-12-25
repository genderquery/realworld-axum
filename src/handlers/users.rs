use axum::{extract::State, Json};
use axum_macros::debug_handler;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::{
    db,
    error::AppError,
    jwt::{self, Claims},
    password::{hash_password, verify_password},
    validation::{validate_not_empty, ValidationErrors},
    AppState, Database,
};

#[debug_handler(state = AppState)]
pub async fn register(
    State(pool): State<Pool<Postgres>>,
    State(jwt): State<jwt::Config>,
    Json(payload): Json<NewUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    let new_user = payload.user;

    let mut errors = ValidationErrors::new();
    if validate_not_empty(&mut errors, "username", &new_user.username) {
        validate_username_unique(&mut errors, &pool, &new_user.username).await?;
    }
    if validate_not_empty(&mut errors, "email", &new_user.email) {
        validate_email_unique(&mut errors, &pool, &new_user.email).await?;
    }
    validate_not_empty(&mut errors, "password", &new_user.password);
    errors.into_result()?;

    let password_hash = hash_password(&new_user.password)?;
    let user =
        db::users::create(&pool, &new_user.username, &new_user.email, &password_hash).await?;
    let token = jwt::create_token(&jwt, &user.username, &user.email)?;

    Ok((
        StatusCode::CREATED,
        Json(UserResponse {
            user: User {
                username: user.username,
                email: user.email,
                token,
            },
        }),
    ))
}

#[debug_handler(state = AppState)]
pub async fn login(
    State(pool): State<Pool<Postgres>>,
    State(jwt): State<jwt::Config>,
    Json(payload): Json<LoginUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let login_user = payload.user;

    let mut errors = ValidationErrors::new();
    validate_not_empty(&mut errors, "email", &login_user.email);
    validate_not_empty(&mut errors, "password", &login_user.password);
    errors.into_result()?;

    let maybe_user = db::users::get_by_email(&pool, &login_user.email).await?;
    let user = match maybe_user {
        Some(user) => user,
        None => {
            return Err(AppError::Unauthorized);
        }
    };

    verify_password(&login_user.password, &user.password_hash)?;

    let token = jwt::create_token(&jwt, &user.username, &user.email)?;

    Ok(Json(UserResponse {
        user: User {
            username: user.username,
            email: user.email,
            token,
        },
    }))
}

#[debug_handler(state = AppState)]
pub async fn get_current_user(
    State(pool): State<Pool<Postgres>>,
    State(jwt): State<jwt::Config>,
    claims: Claims,
) -> Result<Json<UserResponse>, AppError> {
    // TODO: we should be using user id because the user can change their username
    let maybe_user = db::users::get_by_username(&pool, &claims.username).await?;
    let user = match maybe_user {
        Some(user) => user,
        None => {
            return Err(AppError::Unauthorized);
        }
    };

    let token = jwt::create_token(&jwt, &user.username, &user.email)?;

    Ok(Json(UserResponse {
        user: User {
            username: user.username,
            email: user.email,
            token,
        },
    }))
}

#[debug_handler(state = AppState)]
pub async fn update_user(
    State(pool): State<Pool<Postgres>>,
    State(jwt): State<jwt::Config>,
    claims: Claims,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let update_user = payload.user;

    let mut user = match db::users::get_by_username(&pool, &claims.username).await? {
        Some(user) => user,
        None => {
            return Err(AppError::Unauthorized);
        }
    };

    let mut errors = ValidationErrors::new();
    if let Some(username) = update_user.username {
        if validate_not_empty(&mut errors, "username", &username) {
            validate_username_unique(&mut errors, &pool, &username).await?;
        }
        user.username = username;
    }
    if let Some(email) = update_user.email {
        if validate_not_empty(&mut errors, "email", &email) {
            validate_email_unique(&mut errors, &pool, &email).await?;
        }
        user.email = email;
    }
    if let Some(password) = update_user.password {
        validate_not_empty(&mut errors, "password", &password);
        user.password_hash = hash_password(&password)?;
    }
    if let Some(bio) = update_user.bio {
        user.bio = Some(bio);
    }
    if let Some(image) = update_user.image {
        user.image = Some(image);
    }
    errors.into_result()?;

    db::users::update(
        &pool,
        &user.username,
        &user.email,
        &user.password_hash,
        user.bio.as_deref(),
        user.image.as_deref(),
    )
    .await?;

    let token = jwt::create_token(&jwt, &user.username, &user.email)?;

    Ok(Json(UserResponse {
        user: User {
            username: user.username,
            email: user.email,
            token,
        },
    }))
}

#[derive(Debug, Deserialize)]
pub struct NewUserRequest {
    user: NewUser,
}

#[derive(Debug, Deserialize)]
pub struct NewUser {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserRequest {
    user: LoginUser,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    user: UpdateUser,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    username: Option<String>,
    email: Option<String>,
    password: Option<String>,
    bio: Option<String>,
    image: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    user: User,
}

#[derive(Debug, Serialize)]
pub struct User {
    username: String,
    email: String,
    token: String,
}

async fn validate_username_unique(
    errors: &mut ValidationErrors,
    pool: &Database,
    username: &str,
) -> Result<bool, sqlx::Error> {
    if db::users::get_by_username(pool, username).await?.is_some() {
        errors.add("username", "unique");
        Ok(false)
    } else {
        Ok(true)
    }
}

async fn validate_email_unique(
    errors: &mut ValidationErrors,
    pool: &Database,
    email: &str,
) -> Result<bool, sqlx::Error> {
    if db::users::get_by_email(pool, email).await?.is_some() {
        errors.add("email", "unique");
        Ok(false)
    } else {
        Ok(true)
    }
}
