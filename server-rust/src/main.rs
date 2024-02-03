use axum::{routing::get, Router};

pub mod pathfinding;
use pathfinding::a_star;

pub mod common;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, Rust!" }));

    a_star::test();
    let a = common::Node{x: 1.0, y: 2.0};
    println!("{} {}", a.x, a.y);
    println!("Running on http://localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}