use std::net::SocketAddr;
use std::time::Duration;

use conduit::{app, jwt, AppState};
use http::header;
use http_body::Body;
use http_body_util::{BodyExt, Empty};
use hyper::client::conn::http1::handshake;
use hyper::{body::Bytes, client::conn::http1::SendRequest};
use hyper::{Request, StatusCode};
use hyper_util::rt::TokioIo;
use jsonwebtoken::{DecodingKey, EncodingKey};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

async fn spawn_server() -> SocketAddr {
    let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
    let address = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app(state())).await.unwrap();
    });

    address
}

async fn client<A, B>(address: A) -> SendRequest<B>
where
    A: ToSocketAddrs,
    B: Body + 'static + Send,
    B::Data: Send,
    B::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    let stream = TcpStream::connect(address).await.unwrap();
    let io = TokioIo::new(stream);

    let (sender, conn) = handshake(io).await.unwrap();

    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    sender
}

fn state() -> AppState {
    AppState {
        jwt: jwt::Config {
            expiration: Duration::from_secs(3600),
            encoding_key: EncodingKey::from_secret("secret".as_bytes()),
            decoding_key: DecodingKey::from_secret("secret".as_bytes()),
        },
    }
}

#[tokio::test]
async fn test_authorization() {
    let address = spawn_server().await;
    let mut client = client(address).await;

    let request = Request::builder()
        .uri(format!("http://{address}/"))
        .header(header::HOST, "localhost")
        .body(Empty::<Bytes>::new())
        .unwrap();

    let response = client.send_request(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let request = Request::builder()
        .uri(format!("http://{address}/token"))
        .header(header::HOST, "localhost")
        .body(Empty::<Bytes>::new())
        .unwrap();

    let response = client.send_request(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let token = response.into_body().collect().await.unwrap().to_bytes();
    let token = String::from_utf8(token.into()).unwrap();

    let request = Request::builder()
        .uri(format!("http://{address}/"))
        .header(header::HOST, "localhost")
        .header(header::AUTHORIZATION, format!("Token {token}"))
        .body(Empty::<Bytes>::new())
        .unwrap();

    let response = client.send_request(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
