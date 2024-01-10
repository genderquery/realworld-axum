use std::env;

use conduit::{
    app,
    jwt::{self, Jwt},
    AppState,
};
use jsonwebtoken::{DecodingKey, EncodingKey};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "conduit=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let state = AppState {
        jwt: Jwt::new(jwt::Config {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            duration_secs: 15 * 60,
        }),
    };
    let app = app(state).layer(TraceLayer::new_for_http());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Listening on at http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
