use axum::{extract::Multipart, response::Html};
use std::{fs::File, io::Write};

pub async fn index() -> Html<&'static str> {
    Html(std::include_str!("../public/index.html"))
}

pub async fn upload(mut multipart: Multipart) {
    while let Some(field) = multipart
        .next_field()
        .await
        .expect("Failed to get next field!")
    {
        if field.name().unwrap() != "fileupload" {
            continue;
        }

        let file_name = field.file_name().unwrap();

        let file_path = format!("files/{}", file_name);

        let data = field.bytes().await.unwrap();

        let mut file_handle = File::create(file_path).expect("Failed to open file handle!");

        file_handle.write_all(&data).expect("Failed to write data!");
    }
}
