use axum::{Router, routing};

#[tokio::main]
async fn main() {
    println!("Starting server on port 3000...");
    let app = Router::new().route("/", routing::get(|| async { "HELLO WORLD"}));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
