use axum::{routing::get, Router};

use super::controller;

pub fn setup_stock_routes() -> Router<()>{
    Router::new()
        .route("/stock/:ticker", get(controller::get_stock))
}
