use axum::{routing::get, Router};

pub mod pathfinding;
use pathfinding::a_star;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, Rust!" }));

    a_star::test();

    println!("Running on http://localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}