use axum::{
    routing::{delete, get, post},
    Router,
};
use handlers::{articles, comments, favorites, profiles, tags, users};

pub mod error;
mod handlers;

pub trait AppState: Clone + Send + Sync + 'static {}

pub fn app<A: AppState>(state: A) -> Router {
    Router::new()
        .route("/api/user", get(users::current_user).post(users::update))
        .route("/api/users", post(users::register))
        .route("/api/users/login", post(users::login))
        .route("/api/profiles/:username", get(profiles::get_profile))
        .route(
            "/api/profiles/:username/follow",
            post(profiles::follow).delete(profiles::unfollow),
        )
        .route(
            "/api/articles",
            get(articles::get_articles).post(articles::create_article),
        )
        .route("/api/articles/feed", get(articles::get_feed))
        .route(
            "/api/articles/:slug",
            get(articles::get_article)
                .put(articles::update_article)
                .delete(articles::delete_article),
        )
        .route(
            "/api/articles/:slug/comments",
            get(comments::get_comments).post(comments::create_comment),
        )
        .route(
            "/api/articles/:slug/comments/:id",
            delete(comments::delete_comment),
        )
        .route(
            "/api/articles/:slug/favorite",
            post(favorites::favorite).delete(favorites::unfavorite),
        )
        .route("/api/tags", get(tags::get_tags))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::*;

    #[derive(Default, Clone)]
    struct MockAppState {}

    impl AppState for MockAppState {}

    #[tokio::test]
    async fn hello_world() {
        let state = MockAppState::default();
        let app = app(state);

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Hello, World!");
    }
}
