use axum::{routing::{get,post}, Router};
use grid_generator::gridgeneration;
pub mod grid_generator;
pub mod pathfinding;
use pathfinding::dijkstra;
pub mod common;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, Rust!" }))
        .route("/solution", post(dijkstra::solution_handler));
    gridgeneration::generate_grid();
    dijkstra::test();

    println!("Running on http://localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}