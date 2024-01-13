use axum::{
    debug_handler,
    extract::{Path, State},
    Json,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::{error::AppError, jwt::Claims, AppState};

#[debug_handler(state = AppState)]
pub async fn get_profile(
    State(pool): State<PgPool>,
    maybe_claims: Option<Claims>,
    Path(username): Path<String>,
) -> Result<Json<ProfileResponse>, AppError> {
    let profile = sqlx::query!(
        r#"
            select
                username,
                bio,
                image,
                exists (
                    select 1 from follows
                    where followee = users.id and follower = $1
                ) "following!"
            from users
            where username = $2
        "#,
        maybe_claims.map(|claims| claims.user_id),
        username
    )
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound)?;

    Ok(Json(ProfileResponse {
        profile: Profile {
            username: profile.username,
            bio: profile.bio,
            image: profile.image,
            following: profile.following,
        },
    }))
}

#[debug_handler(state = AppState)]
pub async fn follow(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(username): Path<String>,
) -> Result<Json<ProfileResponse>, AppError> {
    let followee = sqlx::query!(
        r#"
            select id, username, bio, image
            from users
            where username = $1
        "#,
        username
    )
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound)?;

    // TODO: handle double follow
    sqlx::query!(
        r#"
            insert into follows
            (follower, followee)
            values ($1, $2)
        "#,
        claims.user_id,
        followee.id,
    )
    .execute(&pool)
    .await?;

    Ok(Json(ProfileResponse {
        profile: Profile {
            username: followee.username,
            bio: followee.bio,
            image: followee.image,
            following: true,
        },
    }))
}

#[debug_handler(state = AppState)]
pub async fn unfollow(
    State(pool): State<PgPool>,
    claims: Claims,
    Path(username): Path<String>,
) -> Result<Json<ProfileResponse>, AppError> {
    let followee = sqlx::query!(
        r#"
            select id, username, bio, image
            from users
            where username = $1
        "#,
        username
    )
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound)?;

    sqlx::query!(
        r#"
            delete from follows
            where follower = $1 and followee = $2
        "#,
        claims.user_id,
        followee.id,
    )
    .execute(&pool)
    .await?;

    Ok(Json(ProfileResponse {
        profile: Profile {
            username: followee.username,
            bio: followee.bio,
            image: followee.image,
            following: false,
        },
    }))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileResponse {
    profile: Profile,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    username: String,
    bio: String,
    image: String,
    following: bool,
}
