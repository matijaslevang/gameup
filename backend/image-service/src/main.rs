use axum::{
    extract::{DefaultBodyLimit, Multipart, Path},
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use std::{
    fs,
    io::Write,
};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

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

async fn get_images(Path(game_id): Path<i32>) -> Json<Vec<String>> {
    let dir = format!("/app/uploads/{}", game_id);

    let mut urls = vec![];
    println!("hey we are here");
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if let Some(file_name) = entry.file_name().to_str() {
                let url = format!(
                    "http://localhost:8003/uploads/{}/{}",
                    game_id,
                    file_name
                );
                urls.push(url);
            }
        }
    }

    Json(urls)
}

async fn delete_images(
    Path(game_id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let dir = format!("/app/uploads/{}", game_id);

    if std::path::Path::new(&dir).exists() {
        std::fs::remove_dir_all(&dir)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(StatusCode::NO_CONTENT)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/images/:game_id", post(upload_images))
        .route("/images/:game_id", get(get_images))
        .route("/images/:game_id", delete(delete_images))
        .nest_service("/uploads", ServeDir::new("/app/uploads"))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 500));

    let listener = TcpListener::bind("0.0.0.0:8003").await.unwrap();

    println!("Image service running on 0.0.0.0:8003");

    axum::serve(listener, app).await.unwrap();
}