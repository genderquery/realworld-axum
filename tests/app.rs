use std::net::SocketAddr;

use conduit::app;
use http_body::Body;
use http_body_util::{BodyExt, Empty};
use hyper::client::conn::http1::handshake;
use hyper::{body::Bytes, client::conn::http1::SendRequest};
use hyper::{Request, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

async fn spawn_server() -> SocketAddr {
    let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
    let address = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app()).await.unwrap();
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

#[tokio::test]
async fn test_app() {
    let address = spawn_server().await;
    let mut client = client(address).await;

    let req = Request::builder()
        .uri(format!("http://{address}/"))
        .header(hyper::header::HOST, "localhost")
        .body(Empty::<Bytes>::new())
        .unwrap();

    let response = client.send_request(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();

    assert_eq!(&body[..], b"Hello, World!");
}
