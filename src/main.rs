use axum::{
    Router,
    routing::{get, post},
    extract::Multipart,
    http::StatusCode,
    response::IntoResponse,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    // Create the application router
    let app = Router::new()
        .route("/", get(handle_request)) // Handle GET requests to the root path
        .route("/upload", post(upload_file)); // Handle POST requests to /upload

    // Set the server address
    let address = SocketAddr::from(([127, 0, 0, 1], 2007));
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
    "Hello, World! , welcome to my world."
}

// Handler for POST /upload
async fn upload_file(mut multipart: Multipart) -> impl IntoResponse {
    // Loop through the fields in the multipart form
    while let Some(field) = multipart.next_field().await.unwrap() {
        // Get the name of the field (e.g., "file")
        let name = field.name().unwrap().to_string();

        // Get the file name of the uploaded file
        let file_name = field.file_name().unwrap().to_string();

        // Get the file data as bytes
        let data = field.bytes().await.unwrap();

        // Create a file path to save the uploaded file
        let file_path = format!("./uploads/{}", file_name);

        // Create the file and write the data to it
        let mut file = File::create(&file_path).await.unwrap();
        file.write_all(&data).await.unwrap();

        // Print a message to the console
        println!("Uploaded file '{}' saved as '{}'", name, file_path);
    }

    // Return a success message to the client
    (StatusCode::OK, "File uploaded successfully")
}