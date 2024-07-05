use std::net::{Ipv4Addr, SocketAddrV4};

use axum::{routing::get, Router};

mod stock;

const LOCALHOST: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = tokio::net::TcpListener::bind(LOCALHOST).await?;
    let app: Router = Router::new().route("/", get(|| async { "Hello, World!" }))
        .merge(stock::router::setup_stock_routes());

    Ok(axum::serve::serve(listener, app).await?)
}
