use axum::{routing::get, Router};
use grid_generator::gridgeneration;
pub mod grid_generator;
#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, Rust!" }));
    gridgeneration::generate_grid();
    println!("Running on http://localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}