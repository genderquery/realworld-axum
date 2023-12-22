use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::{
    extract::FromRef,
    routing::{get, post},
    Router,
};

mod error;
pub mod jwt;
mod password;
mod users;

pub fn app(state: AppState) -> Router {
    // build our application with a single route
    Router::new()
        .route("/api/users", post(users::register))
        .route("/api/users/login", post(users::login))
        .route("/api/user", get(users::get_current_user))
        .with_state(state)
}

pub type MockDb = Arc<RwLock<HashMap<Username, (Username, Email, PasswordHash)>>>;
pub type Username = String;
pub type Email = String;
pub type PasswordHash = String;

#[derive(Clone)]
pub struct AppState {
    pub jwt: jwt::Config,
    pub db: MockDb,
}

impl FromRef<AppState> for jwt::Config {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.jwt.clone()
    }
}
