use std::net::SocketAddr;
use std::time::Duration;

use axum::body::Body;
use conduit::{app, jwt, AppState, MockDb};
use http::{header, Method, Response};
use http_body_util::BodyExt;
use hyper::body::Incoming;
use hyper::client::conn::http1::handshake;
use hyper::client::conn::http1::SendRequest;
use hyper::{Request, StatusCode};
use hyper_util::rt::TokioIo;
use jsonwebtoken::{DecodingKey, EncodingKey};
use serde_json::{json, Value};
use tokio::net::{TcpListener, TcpStream};

async fn spawn_server() -> SocketAddr {
    let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
    let address = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app(state())).await.unwrap();
    });

    address
}

struct Client {
    address: SocketAddr,
    sender: SendRequest<Body>,
}

impl Client {
    async fn from_addr(address: SocketAddr) -> Self {
        let stream = TcpStream::connect(address).await.unwrap();
        let io = TokioIo::new(stream);

        let (sender, conn) = handshake(io).await.unwrap();

        tokio::task::spawn(async move {
            if let Err(err) = conn.await {
                println!("Connection failed: {:?}", err);
            }
        });

        Self { address, sender }
    }

    async fn get_current_user(&mut self, token: Option<&str>) -> Response<Incoming> {
        let builder = Request::builder()
            .uri(format!("http://{}/api/user", self.address))
            .header(header::HOST, "localhost");
        let builder = if let Some(token) = token {
            builder.header(header::AUTHORIZATION, format!("Token {token}"))
        } else {
            builder
        };
        let request = builder.body(Body::empty()).unwrap();

        self.sender.send_request(request).await.unwrap()
    }

    async fn register(&mut self) -> Response<Incoming> {
        let request = Request::builder()
            .uri(format!("http://{}/api/users", self.address))
            .header(header::HOST, "localhost")
            .method(Method::POST)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                serde_json::to_vec(&json!({
                    "username": "example",
                    "email": "example@example.com",
                    "password": "example"
                }))
                .unwrap(),
            ))
            .unwrap();

        self.sender.send_request(request).await.unwrap()
    }

    async fn login(&mut self) -> Response<Incoming> {
        let request = Request::builder()
            .uri(format!("http://{}/api/users/login", self.address))
            .header(header::HOST, "localhost")
            .method(Method::POST)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                serde_json::to_vec(&json!({
                    "username": "example",
                    "password": "example"
                }))
                .unwrap(),
            ))
            .unwrap();

        self.sender.send_request(request).await.unwrap()
    }
}

fn state() -> AppState {
    AppState {
        jwt: jwt::Config {
            expiration: Duration::from_secs(3600),
            encoding_key: EncodingKey::from_secret("secret".as_bytes()),
            decoding_key: DecodingKey::from_secret("secret".as_bytes()),
        },
        db: MockDb::default(),
    }
}

#[tokio::test]
async fn test_authorization() {
    let address = spawn_server().await;
    let mut client = Client::from_addr(address).await;

    let response = client.get_current_user(None).await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let response = client.register().await;

    assert_eq!(response.status(), StatusCode::CREATED);

    let response = client.login().await;

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();
    let token = body["token"].as_str().unwrap();

    let response = client.get_current_user(Some(token)).await;

    assert_eq!(response.status(), StatusCode::OK);
}
