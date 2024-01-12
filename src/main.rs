use std::env;

use conduit::{
    app,
    jwt::{self, Jwt},
    AppState,
};
use sqlx::PgPool;
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

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url).await.unwrap();

    let jwt = Jwt::new(jwt::Config::try_from_env().unwrap());

    let state = AppState { pool, jwt };

    let app = app(state).layer(TraceLayer::new_for_http());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Listening on at http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
