pub mod upload;
pub mod search;
pub mod datatypes;

use axum::{extract::DefaultBodyLimit, Router};

pub fn get_api() -> Router {
    Router::new()
        .route("/upload", axum::routing::post(upload::upload))
            .layer(DefaultBodyLimit::max(50 * 1024 * 1024))
        .route("/search", axum::routing::post(search::search))
}