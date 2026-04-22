use axum::{
    extract::{Multipart, Path},
    routing::post,
    Router,
    http::StatusCode,
};
use std::{fs, io::Write};
use tokio::net::TcpListener;

async fn upload_images(
    Path(game_id): Path<i32>,
    mut multipart: Multipart,
) -> Result<StatusCode, StatusCode> {
    let upload_dir = format!("/app/uploads/{}", game_id);

    // create directory if it doesn't exist
    fs::create_dir_all(&upload_dir).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field
            .file_name()
            .map(|name| name.to_string())
            .unwrap_or_else(|| "file".to_string());

        let data = field.bytes().await.unwrap(); // now it's safe

        let file_path = format!("{}/{}", upload_dir, file_name);

        let mut file = std::fs::File::create(file_path).unwrap();
        file.write_all(&data).unwrap();
    }

    Ok(StatusCode::OK)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/images/:game_id", post(upload_images));

    let listener = TcpListener::bind("0.0.0.0:8003").await.unwrap();

    println!("Image service running on 0.0.0.0:8003");

    axum::serve(listener, app).await.unwrap();
}