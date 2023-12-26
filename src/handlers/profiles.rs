use axum::{
    extract::{Path, State},
    Json,
};
use serde::Serialize;
use sqlx::{Pool, Postgres};

use crate::{
    db::{profiles, users},
    error::AppError,
    jwt::Claims,
};

pub async fn get_profile(
    State(pool): State<Pool<Postgres>>,
    claims: Option<Claims>,
    Path(username): Path<String>,
) -> Result<Json<ProfileResponse>, AppError> {
    let profile = if let Some(claims) = claims {
        let user = users::get_by_username(&pool, &username).await?;
        if let Some(user) = user {
            profiles::get_profile_as_user(&pool, user.id, claims.user_id)
                .await?
                .expect("profile should exist for user")
        } else {
            return Err(AppError::NotFound);
        }
    } else {
        let user = users::get_by_username(&pool, &username).await?;
        if let Some(user) = user {
            profiles::get_profile(&pool, user.id)
                .await?
                .expect("profile should exist for user")
        } else {
            return Err(AppError::NotFound);
        }
    };

    Ok(Json(ProfileResponse {
        profile: Profile {
            username: profile.username,
            bio: profile.bio.unwrap_or_else(|| "".into()),
            image: profile.image.unwrap_or_else(|| "".into()),
            following: profile.following,
        },
    }))
}

pub async fn follow_user(
    State(pool): State<Pool<Postgres>>,
    claims: Claims,
    Path(username): Path<String>,
) -> Result<Json<ProfileResponse>, AppError> {
    let user = users::get_by_username(&pool, &username).await?;
    let profile = if let Some(user) = user {
        profiles::follow_user(&pool, user.id, claims.user_id).await?;
        profiles::get_profile_as_user(&pool, user.id, claims.user_id)
            .await?
            .expect("profile should exist for user")
    } else {
        return Err(AppError::NotFound);
    };

    Ok(Json(ProfileResponse {
        profile: Profile {
            username: profile.username,
            bio: profile.bio.unwrap_or_else(|| "".into()),
            image: profile.image.unwrap_or_else(|| "".into()),
            following: profile.following,
        },
    }))
}

pub async fn unfollow_user(
    State(pool): State<Pool<Postgres>>,
    claims: Claims,
    Path(username): Path<String>,
) -> Result<Json<ProfileResponse>, AppError> {
    let user = users::get_by_username(&pool, &username).await?;
    let profile = if let Some(user) = user {
        profiles::unfollow_user(&pool, user.id, claims.user_id).await?;
        profiles::get_profile_as_user(&pool, user.id, claims.user_id)
            .await?
            .expect("profile should exist for user")
    } else {
        return Err(AppError::NotFound);
    };

    Ok(Json(ProfileResponse {
        profile: Profile {
            username: profile.username,
            bio: profile.bio.unwrap_or_else(|| "".into()),
            image: profile.image.unwrap_or_else(|| "".into()),
            following: profile.following,
        },
    }))
}

#[derive(Debug, Serialize)]
pub struct ProfileResponse {
    profile: Profile,
}

#[derive(Debug, Serialize)]
pub struct Profile {
    username: String,
    bio: String,
    image: String,
    following: bool,
}
