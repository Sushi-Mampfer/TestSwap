mod api;

use std::{net::SocketAddr, sync::LazyLock};

use axum::{Router};
use sqlx::{SqlitePool};
use tower_http::{cors::{Any, CorsLayer}, services::ServeDir};

use crate::api::get_api;

static DB: LazyLock<SqlitePool> = LazyLock::new(|| {
    SqlitePool::connect_lazy("sqlite://db.sqlite").unwrap()
});

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let app = Router::new();
    let html = ServeDir::new("static").append_index_html_on_directories(true);
    let images = ServeDir::new("files");
    let api = get_api();
    let app = app.nest_service("/api", api)
        .nest_service("/image", images)
        .fallback_service(html).layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum_server::bind(addr).serve(app.into_make_service()).await.unwrap();

}
