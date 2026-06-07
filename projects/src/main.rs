use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/ping", get(ping));

    let addr = "0.0.0.0:8000".to_string();
    println!("Listening on http://localhost:8000/");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> &'static str {
    "Hello, world!"
}

async fn ping() -> &'static str {
    "pong"
}
