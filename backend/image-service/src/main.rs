use axum::{
    routing::post,
    Router,
    extract::{Path, Multipart},
    response::IntoResponse,
};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;
use tower_http::cors::{CorsLayer, Any};
use axum::http::StatusCode;

async fn upload_images(Path(game_id): Path<String>, mut multipart: Multipart) -> impl IntoResponse {
    let upload_dir = format!("./uploads/{}", game_id);
    fs::create_dir_all(&upload_dir).unwrap();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().map(|f| f.to_string()).unwrap_or_else(|| Uuid::new_v4().to_string());
        let data = field.bytes().await.unwrap();

        let mut file_path = PathBuf::from(&upload_dir);
        file_path.push(file_name);

        fs::write(file_path, &data).unwrap();
    }

    (StatusCode::CREATED, "Images uploaded successfully")
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/images/:game_id", post(upload_images))
        .layer(cors);

    println!("Image service running on 0.0.0.0:8003");
    axum::Server::bind(&"0.0.0.0:8003".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}