use axum::{async_trait, extract::State, Json};
use axum_macros::debug_handler;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::{
    db,
    error::AppError,
    jwt::{self, Claims},
    password::{hash_password, verify_password},
    validation::{
        validate_not_empty, validate_unique_email, validate_unique_username, Validate,
        ValidationErrors, ValidationErrorsWrapper,
    },
    AppState, Database,
};

#[debug_handler(state = AppState)]
pub async fn register(
    State(pool): State<Pool<Postgres>>,
    State(jwt): State<jwt::Config>,
    Json(payload): Json<NewUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    let new_user = payload.user;
    new_user.validate(&pool).await?;

    let password_hash = hash_password(&new_user.password)?;
    let user = db::create_user(&pool, &new_user.username, &new_user.email, &password_hash).await?;
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
    login_user.validate(&pool).await?;

    let maybe_user = db::get_user_by_username(&pool, &login_user.username).await?;
    let user = match maybe_user {
        Some(user) => user,
        None => {
            return Err(AppError::Unauthorized);
        }
    };

    // TODO handle other errors
    if verify_password(&login_user.password, &user.password_hash).is_err() {
        return Err(AppError::Unauthorized);
    }

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
    let maybe_user = db::get_user_by_username(&pool, &claims.username).await?;
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
    update_user.validate(&pool).await?;

    let mut user = match db::get_user_by_username(&pool, &claims.username).await? {
        Some(user) => user,
        None => {
            return Err(AppError::Unauthorized);
        }
    };

    if let Some(username) = update_user.username {
        user.username = username;
    }
    if let Some(email) = update_user.email {
        user.email = email;
    }
    if let Some(password) = update_user.password {
        user.password_hash = hash_password(&password)?;
    }
    if let Some(bio) = update_user.bio {
        user.bio = Some(bio);
    }
    if let Some(image) = update_user.image {
        user.image = Some(image);
    }

    db::update_user(
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

#[async_trait]
impl Validate for NewUser {
    async fn validate(&self, pool: &Database) -> Result<(), ValidationErrorsWrapper> {
        let mut errors = ValidationErrors::new();

        if validate_not_empty(&mut errors, "username", &self.username) {
            validate_unique_username(&mut errors, &self.username, pool).await?;
        }
        if validate_not_empty(&mut errors, "email", &self.email) {
            validate_unique_email(&mut errors, &self.email, pool).await?;
        }
        validate_not_empty(&mut errors, "password", &self.password);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(ValidationErrorsWrapper::ValidationErrors(errors))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginUserRequest {
    user: LoginUser,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    username: String,
    password: String,
}

#[async_trait]
impl Validate for LoginUser {
    async fn validate(&self, _: &Database) -> Result<(), ValidationErrorsWrapper> {
        let mut errors = ValidationErrors::new();

        validate_not_empty(&mut errors, "username", &self.username);
        validate_not_empty(&mut errors, "password", &self.password);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(ValidationErrorsWrapper::ValidationErrors(errors))
        }
    }
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

#[async_trait]
impl Validate for UpdateUser {
    async fn validate(&self, pool: &Database) -> Result<(), ValidationErrorsWrapper> {
        let mut errors = ValidationErrors::new();

        if let Some(username) = self.username.as_ref() {
            if validate_not_empty(&mut errors, "username", username) {
                validate_unique_username(&mut errors, username, pool).await?;
            }
        }
        if let Some(email) = self.email.as_ref() {
            if validate_not_empty(&mut errors, "email", email) {
                validate_unique_email(&mut errors, email, pool).await?;
            }
        }
        if let Some(password) = self.password.as_ref() {
            validate_not_empty(&mut errors, "password", password);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(ValidationErrorsWrapper::ValidationErrors(errors))
        }
    }
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
