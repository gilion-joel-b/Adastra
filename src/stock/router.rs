use axum::{routing::get, Router};

use super::handler;

pub fn setup_stock_routes() -> Router<()>{
    Router::new()
        .route("/stock/:ticker", get(handler::get_stock))
        .route("/stock/:ticker/history", get(handler::get_history))
        .route("/stock/:ticker/average", get(handler::get_daily_average))
}
