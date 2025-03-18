use tokio::fs::File as TokioFile;
use tokio::io::AsyncWriteExt;
use std::path::Path;

pub async fn index() -> String {
    // Static HTML form can be kept for browser-based uploads (optional)
    "<html><body><form method='POST' enctype='multipart/form-data'>
        <input type='file' name='fileupload' required>
        <button type='submit'>Upload File</button>
    </form></body></html>".to_string()
}

// The upload function now accepts file data and file name
pub async fn upload(file_data: Vec<u8>, file_name: String) {
    // Define the file path where the file will be saved
    let file_path = Path::new("files").join(file_name);

    // Create or open the file
    let mut file_handle = TokioFile::create(file_path)
        .await
        .expect("Failed to open file handle!");

    // Write the file data
    file_handle
        .write_all(&file_data)
        .await
        .expect("Failed to write data to file!");
}

// loadfile.rs