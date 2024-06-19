use std::net::{Ipv4Addr, SocketAddrV4};


use axum::{routing::get, Router};

const LOCALHOST: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);

struct AppServer {
    app: Router,
    listener: tokio::net::TcpListener,

}

impl AppServer {
    pub async fn new() -> Self {
        AppServer{
            app: Router::new().route("/", get(|| async { "Hello, World!" })),
            listener: tokio::net::TcpListener::bind(LOCALHOST).await.unwrap()
        }
    } 

    pub async fn serve() {
        let _app: Router = Router::new().route("/", get(|| async { "Hello, World!" }));
        let listener = tokio::net::TcpListener::bind(LOCALHOST).await.unwrap();
    }
} 
async fn serve() {
    let _app: Router = Router::new().route("/", get(|| async { "Hello, World!" }));
    let listener = tokio::net::TcpListener::bind(LOCALHOST).await.unwrap();
}
