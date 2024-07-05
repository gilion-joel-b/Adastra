use axum::{extract::Path, response::IntoResponse, Json};

use super::domain::Stock;


pub async fn get_stock(Path(ticker): Path<String>) -> impl IntoResponse {
    let stock = Stock::new(ticker, 100.0);
    Json(stock)
}
