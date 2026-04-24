use axum::Router;
use axum::routing::{delete, get, post, put};
use tokio::net::TcpListener;
use tower_http::cors::{CorsLayer, Any};
use axum::http::Method;
use axum::extract::Path;
use serde::{Deserialize, Serialize};
use axum::{
    http::{Request, StatusCode, header},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use axum::middleware;
use axum::body::Bytes;
use axum::response::IntoResponse;
use reqwest::Body as ReqwestBody;
use axum::extract::DefaultBodyLimit;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Claims {
    sub: String,
    role: String,
    exp: usize,
}


async fn auth_middleware(
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    let token = match auth_header {
        Some(h) if h.starts_with("Bearer ") => &h[7..],
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    let secret = std::env::var("JWT_SECRET").unwrap();

    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // (optional) attach claims to request extensions
    req.extensions_mut().insert(decoded.claims);

    Ok(next.run(req).await)
}

async fn forward_upload_images(
    Path(game_id): Path<i32>,
    req: Request<axum::body::Body>,
) -> Result<Response, StatusCode> {
    let client = reqwest::Client::new();

    let content_type = req
        .headers()
        .get(header::CONTENT_TYPE)
        .cloned()
        .ok_or(StatusCode::BAD_REQUEST)?;

    let body_bytes = axum::body::to_bytes(req.into_body(), usize::MAX)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let resp = client
        .post(format!("http://image-service:8003/images/{}", game_id))
        .header(header::CONTENT_TYPE, content_type)
        .body(body_bytes)
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    let status = resp.status();
    let body = resp.bytes().await.map_err(|_| StatusCode::BAD_GATEWAY)?;

    Ok((
        status,
        [(header::CONTENT_TYPE, "application/json")],
        body,
    ).into_response())
}

async fn forward_get_images(
    Path(game_id): Path<i32>,
) -> Result<Response, StatusCode> {
    let resp = reqwest::get(
        format!("http://image-service:8003/images/{}", game_id)
    )
    .await
    .map_err(|_| StatusCode::BAD_GATEWAY)?;

    let status = axum::http::StatusCode::from_u16(resp.status().as_u16())
        .unwrap();

    let body = resp.bytes().await.map_err(|_| StatusCode::BAD_GATEWAY)?;

    Ok((status, body).into_response())
}

async fn forward_delete_images(
    Path(id): Path<i32>,
) -> Result<Response, StatusCode> {
    let client = reqwest::Client::new();

    let resp = client
        .delete(format!("http://image-service:8003/images/{}", id))
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    let status = StatusCode::from_u16(resp.status().as_u16()).unwrap();

    Ok(status.into_response())
}

async fn forward_upload_videos(
    Path(game_id): Path<i32>,
    req: Request<axum::body::Body>,
) -> Result<Response, StatusCode> {
    let client = reqwest::Client::new();

    let headers = req.headers().clone();

    let stream = req.into_body().into_data_stream();
    let body = ReqwestBody::wrap_stream(stream);

    let mut request_builder = client
        .post(format!("http://video-service:8004/videos/{}", game_id));

    for (key, value) in headers.iter() {
        request_builder = request_builder.header(key, value);
    }

    let resp = request_builder
        .body(body)
        .send()
        .await
        .map_err(|e| {
            println!("Forward error: {:?}", e);
            StatusCode::BAD_GATEWAY
        })?;

    let status = resp.status();
    let body = resp.bytes().await.map_err(|_| StatusCode::BAD_GATEWAY)?;

    Ok((
        status,
        [(header::CONTENT_TYPE, "application/json")],
        body,
    ).into_response())
}

async fn forward_get_videos(
    Path(game_id): Path<i32>,
) -> Result<Response, StatusCode> {
    let resp = reqwest::get(
        format!("http://video-service:8004/videos/{}", game_id)
    )
    .await
    .map_err(|_| StatusCode::BAD_GATEWAY)?;

    let status = axum::http::StatusCode::from_u16(resp.status().as_u16())
        .unwrap();

    let body = resp.bytes().await.map_err(|_| StatusCode::BAD_GATEWAY)?;

    Ok((status, body).into_response())
}

async fn forward_delete_videos(
    Path(id): Path<i32>,
) -> Result<Response, StatusCode> {
    let client = reqwest::Client::new();

    let resp = client
        .delete(format!("http://video-service:8004/videos/{}", id))
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    let status = StatusCode::from_u16(resp.status().as_u16()).unwrap();

    Ok(status.into_response())
}

async fn forward_create_game(body: Bytes) -> impl axum::response::IntoResponse {
    let resp = reqwest::Client::new()
        .post("http://game-service:8001/games")
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .unwrap();

    let status = resp.status();
    let body = resp.bytes().await.unwrap();

    (
        status,
        [(axum::http::header::CONTENT_TYPE, "application/json")],
        body
    )
}

use axum::extract::Query;
use std::collections::HashMap;

async fn forward_games(
    Query(params): Query<HashMap<String, String>>,
) -> String {
    let client = reqwest::Client::new();

    let resp = client
        .get("http://game-service:8001/games")
        .query(&params)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    resp
}

async fn forward_game(Path(id): Path<i32>) -> String {
    reqwest::get(format!("http://game-service:8001/games/{}", id))
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}

async fn forward_delete_game(
    Path(id): Path<i32>,
) -> Result<Response, StatusCode> {
    let client = reqwest::Client::new();

    // delete images (ignore failure)
    let _ = client
        .delete(format!("http://image-service:8003/images/{}", id))
        .send()
        .await;

    // delete videos (ignore failure)
    let _ = client
        .delete(format!("http://video-service:8004/videos/{}", id))
        .send()
        .await;

    // delete game (main operation)
    let resp = client
        .delete(format!("http://game-service:8001/games/{}", id))
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    Ok(StatusCode::from_u16(resp.status().as_u16()).unwrap().into_response())
}

async fn forward_update_game(
    Path(id): Path<i32>,
    body: Bytes,
) -> Result<Response, StatusCode> {
    let client = reqwest::Client::new();

    let resp = client
        .put(format!("http://game-service:8001/games/{}", id))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    let status = StatusCode::from_u16(resp.status().as_u16()).unwrap();

    Ok(status.into_response())
}

async fn forward_login(body: Bytes) -> String {
    reqwest::Client::new()
        .post("http://auth-service:8002/login")
        .body(body)
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}

async fn forward_register(body: Bytes) -> String {
    reqwest::Client::new()
        .post("http://auth-service:8002/register")
        .body(body)
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
        .allow_headers(Any);

    let protected_routes = Router::new()
        .route("/api/games", post(forward_create_game))
        .route("/api/images/:game_id", post(forward_upload_images))
        .route("/api/videos/:game_id", post(forward_upload_videos))
        .route("/api/games/:id", delete(forward_delete_game))
        .route("/api/games/:id", put(forward_update_game))
        .route("/api/images/:game_id", delete(forward_delete_images))
        .route("/api/videos/:game_id", delete(forward_delete_videos))
        .route_layer(middleware::from_fn(auth_middleware));

    let app = Router::new()
        // public
        .route("/api/games", get(forward_games))
        .route("/api/games/:id", get(forward_game))
        .route("/api/images/:game_id", get(forward_get_images))
        
        .route("/api/videos/:game_id", get(forward_get_videos))
        
        // auth routes (forward to auth service)
        .route("/api/login", post(forward_login))
        .route("/api/register", post(forward_register))

        // protected
        .merge(protected_routes)

        .layer(cors)
        .layer(DefaultBodyLimit::max(1024 * 1024 * 500));

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("Gateway running on 0.0.0.0:8000");

    axum::serve(listener, app).await.unwrap();
}