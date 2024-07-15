use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};

pub async fn get_stock(
    Path(ticker): Path<String>,
    Extension(stock_service): Extension<super::service::StockService>,
) -> impl IntoResponse {
    stock_service
        .search_stock_by_ticker(ticker)
        .await
        .map(|stock| Json(stock).into_response())
        .unwrap_or_else(|e| (StatusCode::NOT_FOUND, e.to_string()).into_response())
}

pub async fn get_history(
    Path(ticker): Path<String>,
    Extension(stock_service): Extension<super::service::StockService>,
) -> impl IntoResponse {
    stock_service
        .get_monthly_historical_prices(ticker)
        .await
        .map(|history| Json(history).into_response())
        .unwrap_or_else(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response())
}

