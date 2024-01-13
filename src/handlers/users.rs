use axum::{debug_handler, extract::State, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    error::AppError,
    jwt::{Claims, Jwt},
    password::{hash_password, verify_password},
    AppState,
};

#[debug_handler(state = AppState)]
pub async fn register(
    State(pool): State<PgPool>,
    State(jwt): State<Jwt>,
    Json(payload): Json<NewUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let NewUserRequest {
        user: NewUser {
            username,
            email,
            password,
        },
    } = payload;

    // TODO: validation

    let password_hash = hash_password(&password)?;

    let user_id = sqlx::query_scalar!(
        r#"insert into users (email, username, password_hash) values ($1, $2, $3) returning id"#,
        email,
        username,
        password_hash
    )
    .fetch_one(&pool)
    .await?;

    let token = jwt.encode(user_id)?;

    Ok(Json(UserResponse {
        user: User {
            email,
            token,
            username,
            bio: "".into(),
            image: "".into(),
        },
    }))
}

#[debug_handler(state = AppState)]
pub async fn login(
    State(pool): State<PgPool>,
    State(jwt): State<Jwt>,
    Json(payload): Json<LoginUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let LoginUserRequest {
        user: LoginUser { email, password },
    } = payload;

    // TODO: validation

    let user = sqlx::query!(
        r#"select id, email, username, password_hash, bio, image from users where email = $1"#,
        email
    )
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::Unauthorized)?;

    verify_password(&password, &user.password_hash).map_err(|err| match err {
        password_hash::Error::Password => AppError::Unauthorized,
        _ => AppError::InternalServerError(err.into()),
    })?;

    let token = jwt.encode(user.id)?;

    Ok(Json(UserResponse {
        user: User {
            email: user.email,
            token,
            username: user.username,
            bio: user.bio,
            image: user.image,
        },
    }))
}

#[debug_handler(state = AppState)]
pub async fn current_user(
    State(pool): State<PgPool>,
    State(jwt): State<Jwt>,
    claims: Claims,
) -> Result<Json<UserResponse>, AppError> {
    let user = sqlx::query!(
        r#"select id, email, username, bio, image from users where id = $1"#,
        claims.user_id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::Unauthorized)?;

    let token = jwt.encode(user.id)?;

    Ok(Json(UserResponse {
        user: User {
            email: user.email,
            token,
            username: user.username,
            bio: user.bio,
            image: user.image,
        },
    }))
}

#[debug_handler(state = AppState)]
pub async fn update(
    State(pool): State<PgPool>,
    State(jwt): State<Jwt>,
    claims: Claims,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let UpdateUserRequest {
        user:
            UpdateUser {
                username,
                email,
                password,
                bio,
                image,
            },
    } = payload;

    let password_hash = if let Some(password) = password {
        Some(hash_password(&password)?)
    } else {
        None
    };

    let user = sqlx::query!(
        r#"
            update users
            set email = coalesce($1, users.email),
                username = coalesce($2, users.username),
                password_hash = coalesce($3, users.password_hash),
                bio = coalesce($4, users.bio),
                image = coalesce($5, users.image)
            where id = $6
            returning id, email, username, bio, image
        "#,
        username,
        email,
        password_hash,
        bio,
        image,
        claims.user_id
    )
    .fetch_one(&pool)
    .await?;

    let token = jwt.encode(user.id)?;

    Ok(Json(UserResponse {
        user: User {
            email: user.email,
            token,
            username: user.username,
            bio: user.bio,
            image: user.image,
        },
    }))
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
