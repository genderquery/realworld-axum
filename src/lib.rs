use axum::{
    extract::FromRef,
    routing::{get, post},
    Router,
};
use handlers::users;
use sqlx::{Pool, Postgres};

mod db;
mod error;
mod handlers;
pub mod jwt;
mod password;
mod validation;

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/api/users", post(users::register))
        .route("/api/users/login", post(users::login))
        .route(
            "/api/user",
            get(users::get_current_user).put(users::update_user),
        )
        .with_state(state)
}

pub type Database = Pool<Postgres>;

#[derive(Clone)]
pub struct AppState {
    pub jwt: jwt::Config,
    pub db: Database,
}

impl FromRef<AppState> for jwt::Config {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.jwt.clone()
    }
}

impl FromRef<AppState> for Database {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.db.clone()
    }
}
