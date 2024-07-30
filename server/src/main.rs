use axum::Router;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::cors::CorsLayer;

fn main() {
    let layer = CorsLayer::permissive();
    let router = Router::new().nest_service(
        "/",
        ServeDir::new("dist").not_found_service(ServeFile::new("dist/index.html")),
    ).layer(layer);
}
