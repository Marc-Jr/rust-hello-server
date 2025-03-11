use axum::{Router, routing::get};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Create the application router
    let app = Router::new().route("/", get(handle_request));

    // Set the server address
    let address = SocketAddr::from(([127, 0, 0, 1], 5000));
    println!("Server is running at http://{}", address);

    // Create a listener for incoming connections
    let listener = TcpListener::bind(address).await.expect("Failed to bind address");

    // Start serving requests
    if let Err(e) = axum::serve(listener, app.into_make_service()).await {
        eprintln!("Server error: {}", e);
    }
}

// Handler for GET /
async fn handle_request() -> &'static str {
    "Hello, World!"
}