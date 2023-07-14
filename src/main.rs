use axum::{response::Redirect, routing::get, Router};
use tower_http::services::{ServeDir, ServeFile};

use maud::html;

#[tokio::main]
async fn main() {
    let static_files = ServeDir::new(".")
        .not_found_service(ServeFile::new("404.html"))
        .append_index_html_on_directories(true);


    let app = Router::new()
        .nest_service("/", static_files)
        .route(
            "/keys",
            get(|| async { html! { p { "hi there"}} })
        );

    let port = std::env::args().skip(1).next().unwrap();

    let ip_port = format!("0.0.0.0:{}", port);
    axum::Server::bind(&ip_port.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
