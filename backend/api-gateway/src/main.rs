use axum::Router;
use axum::routing::{get, post};
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

async fn forward_games() -> String {
    let resp = reqwest::get("http://game-service:8001/games")
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
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let protected_routes = Router::new()
        .route("/api/games", post(forward_create_game))
        .route("/api/images/:game_id", post(forward_upload_images))
        .route_layer(middleware::from_fn(auth_middleware));

    let app = Router::new()
        // public
        .route("/api/games", get(forward_games))
        .route("/api/games/:id", get(forward_game))
        
        // auth routes (forward to auth service)
        .route("/api/login", post(forward_login))
        .route("/api/register", post(forward_register))

        // protected
        .merge(protected_routes)

        .layer(cors);

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("Gateway running on 0.0.0.0:8000");

    axum::serve(listener, app).await.unwrap();
}