use axum::{
    routing::{delete, get, post},
    Router,
};
use handlers::{articles, comments, favorites, profiles, tags, users};

pub mod error;
mod handlers;
mod repos;

pub trait AppState: Clone + Send + Sync + 'static {}

#[rustfmt::skip]
pub fn app<A: AppState>(state: A) -> Router {
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

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use axum::{
        body::Body,
        http::{self, header, request, HeaderValue, Method, Request, StatusCode},
        response::Response,
    };
    use http_body_util::BodyExt;
    use serde_json::json;
    use tower::ServiceExt;

    use super::*;

    #[derive(Default, Clone)]
    struct MockAppState {}

    impl AppState for MockAppState {}

    trait RequestUtils {
        fn json(self, value: &serde_json::Value) -> Result<Request<Body>, http::Error>;
    }

    impl RequestUtils for request::Builder {
        fn json(self, value: &serde_json::Value) -> Result<Request<Body>, http::Error> {
            self.header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_vec(value).unwrap()))
        }
    }

    #[async_trait]
    trait ResponseUtils {
        async fn json(self) -> serde_json::Value;
        fn content_type(&self) -> &HeaderValue;
    }

    #[async_trait]
    impl ResponseUtils for Response<Body> {
        async fn json(self) -> serde_json::Value {
            let body = self.into_body().collect().await.unwrap().to_bytes();
            serde_json::from_slice(&body).unwrap()
        }

        fn content_type(&self) -> &HeaderValue {
            self.headers().get(header::CONTENT_TYPE).unwrap()
        }
    }

    #[tokio::test]
    async fn test_user_registration() {
        let state = MockAppState::default();
        let app = app(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/users")
                    .method(Method::POST)
                    .json(&json!(
                        {
                            "user": {
                                "username": "newuser",
                                "email": "email@example.com",
                                "password": "mypassword",
                            }
                        }
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
        assert_eq!(response.content_type(), mime::APPLICATION_JSON.as_ref());

        let body = response.json().await;
        assert_eq!(body["user"]["username"], "newuser");
    }
}
