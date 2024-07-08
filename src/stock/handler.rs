use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};

pub async fn get_stock(
    Path(ticker): Path<String>,
    Extension(stock_service): Extension<super::service::StockService>,
) -> impl IntoResponse {

    stock_service.search_stock_by_ticker(ticker).await
        .map(|stock| Json(stock).into_response())
        .unwrap_or_else(|_| StatusCode::NOT_FOUND.into_response())
}
