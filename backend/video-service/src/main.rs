use axum::{
    extract::{Multipart, Path},
    routing::{post, get},
    Router,
    http::StatusCode,
    Json,
};
use std::{fs, io::Write};
use tower_http::services::ServeDir;
use tokio::io::AsyncWriteExt;
use axum::extract::DefaultBodyLimit;


async fn upload_videos(
    Path(game_id): Path<i32>,
    mut multipart: Multipart,
) -> Result<StatusCode, StatusCode> {
    let upload_dir = format!("/app/video/{}", game_id);

    if let Err(e) = tokio::fs::create_dir_all(&upload_dir).await {
        println!("Failed to create dir: {:?}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    while let Some(field) = match multipart.next_field().await {
        Ok(field) => field,
        Err(e) => {
            println!("Multipart error: {:?}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    } {
        let file_name = field
            .file_name()
            .map(|n| n.to_string())
            .unwrap_or_else(|| "video.mp4".to_string());

        let file_path = format!("{}/{}", upload_dir, file_name);

        let mut file = match tokio::fs::File::create(&file_path).await {
            Ok(f) => f,
            Err(e) => {
                println!("File create error: {:?}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        let mut field = field;
        loop {
            match field.chunk().await {
                Ok(Some(chunk)) => {
                    if let Err(e) = file.write_all(&chunk).await {
                        println!("Write error: {:?}", e);
                        return Err(StatusCode::INTERNAL_SERVER_ERROR);
                    }
                }
                Ok(None) => break,
                Err(e) => {
                    println!("Chunk read error: {:?}", e);
                    return Err(StatusCode::BAD_REQUEST);
                }
            }
        }
    }

    println!("UPLOAD DONE for game {}", game_id);
    Ok(StatusCode::OK)
}

async fn get_videos(Path(game_id): Path<i32>) -> Json<Vec<String>> {
    let dir = format!("/app/video/{}", game_id);

    let mut urls = vec![];

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if let Some(file_name) = entry.file_name().to_str() {
                let url = format!(
                    "http://localhost:8004/video/{}/{}",
                    game_id,
                    file_name
                );
                urls.push(url);
            }
        }
    }

    Json(urls)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/videos/:game_id", post(upload_videos))
        .route("/videos/:game_id", get(get_videos))
        .nest_service("/video", ServeDir::new("/app/video"))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 500));

    println!("Video service running on 0.0.0.0:8004");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8004")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}