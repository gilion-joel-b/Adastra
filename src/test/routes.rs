use axum::{routing::get, Router};

pub fn router() -> Router {
    Router::new().route("/", get(|| async { "Hello, World!" }))
        .route("/hello/:name", get(|params: axum::extract::Path<(String,)>| async {
            params.0
        }))
}
