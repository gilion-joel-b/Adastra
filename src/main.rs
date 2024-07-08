use std::net::{Ipv4Addr, SocketAddrV4};

use axum::{routing::get, Extension, Router};
use yahoo_finance_api::YahooConnector;

mod common;
mod stock;


const LOCALHOST: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = tokio::net::TcpListener::bind(LOCALHOST).await?;

    let stock_service = stock::service::StockService::new(YahooConnector::new()?);


    let app: Router = Router::new().route("/", get(|| async { "Hello, World!" }))
        .merge(stock::router::setup_stock_routes())
        .layer(Extension(stock_service));
        


    Ok(axum::serve::serve(listener, app).await?)
}
