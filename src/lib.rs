use axum::{
    extract::FromRef,
    routing::{delete, get, post},
    Router,
};
use handlers::{articles, comments, favorites, profiles, tags, users};
use jwt::Jwt;
use sqlx::PgPool;

mod error;
mod handlers;
pub mod jwt;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt: Jwt,
}

#[rustfmt::skip]
pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/api/user",
            get(users::current_user).
            post(users::update)
        )
        .route("/api/users",
            post(users::register)
        )
        .route("/api/users/login",
            post(users::login)
        )
        .route("/api/profiles/:username",
            get(profiles::get_profile)
        )
        .route("/api/profiles/:username/follow",
            post(profiles::follow).
            delete(profiles::unfollow),
        )
        .route("/api/articles",
            get(articles::get_articles).
            post(articles::create_article),
        )
        .route("/api/articles/feed",
            get(articles::get_feed)
        )
        .route("/api/articles/:slug",
            get(articles::get_article).
            put(articles::update_article).
            delete(articles::delete_article),
        )
        .route("/api/articles/:slug/comments",
            get(comments::get_comments).
            post(comments::create_comment),
        )
        .route("/api/articles/:slug/comments/:id",
            delete(comments::delete_comment),
        )
        .route("/api/articles/:slug/favorite",
            post(favorites::favorite).
            delete(favorites::unfavorite),
        )
        .route("/api/tags",
            get(tags::get_tags)
        )
        .with_state(state)
}
