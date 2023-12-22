use axum::{routing::get, Router};

pub fn app() -> Router {
    // build our application with a single route
    Router::new().route("/", get(|| async { "Hello, World!" }))
}
