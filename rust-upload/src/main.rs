use axum::{routing::get, Router};
use loadfile::{index, upload};
use std::{fs::File, io::Read, path::Path};
use clap::Parser;

#[derive(Parser)]
#[command(name = "file_uploader")]
#[command(about = "A simple file uploader", long_about = None)]
struct Cli {
    /// Path to the file to upload
    file_path: String,
}

#[tokio::main]
async fn main() {
    // Parse command-line arguments
    let cli = Cli::parse();

    // Read the file content
    let mut file = File::open(&cli.file_path).expect("Failed to open file");
    let mut file_data = Vec::new();
    file.read_to_end(&mut file_data)
        .expect("Failed to read file data");

    // Extract the file name from the path
    let path = Path::new(&cli.file_path);
    let file_name = path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("uploaded_file");

    // Upload the file directly (in-memory) to the server
    upload(file_data, file_name.to_string()).await;

    // Set up the Axum router
    let app = Router::new().route("/", get(index));

    // Bind the server to a socket address
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to start listener!");

    // Serve the application
    axum::serve(listener, app)
        .await
        .expect("Failed to serve 'app'!");
}

mod loadfile;

//main.rs