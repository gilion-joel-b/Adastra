mod server;

use std::{iter::Map, net::{Ipv4Addr, SocketAddrV4}};


use axum::{routing::get, Router};

const LOCALHOST: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);

struct AppState {
    context: Map<String, String>,
}

#[tokio::main]
async fn main() {
    let _app: Router = Router::new().route("/", get(|| async { "Hello, World!" }));
    let listener = tokio::net::TcpListener::bind(LOCALHOST).await.unwrap();
}
