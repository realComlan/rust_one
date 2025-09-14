use axum::{Router, routing::get};
use std::net::SocketAddr;
use std::env;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Init logging
    tracing_subscriber::fmt().with_env_filter("info").init();

    // Route simple: GET /health
    let app = Router::new().route("/health", get(health_check));

    // Read the PORT from the env file
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    // run our app with hyper, listening globally on port 3000
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("🚀 Server running at http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

// Handler pour GET /health
async fn health_check() -> &'static str {
    "OK"
}
