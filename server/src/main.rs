use axum::response::Html;
use axum::routing::get;
use axum::Router;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let layer = CorsLayer::permissive();
    let router = Router::new()
    .nest_service(
        "/",
        ServeDir::new("dist").not_found_service(ServeFile::new("dist/index.html")),
    )
    // .nest_service(
    //     "/assets",
    //     ServeDir::new("dist/assets"),
    // )
    // .route("/app", get(index))
    .layer(layer);


    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

// // Include utf-8 file at **compile** time.
// async fn index() -> Html<&'static str> {
//     Html(std::include_str!("../../dist/index.html"))
// }