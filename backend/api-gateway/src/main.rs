use axum::{routing::get, Router};
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

#[derive(Debug, Serialize, Deserialize)]
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

async fn forward_login(body: String) -> String {
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

async fn forward_register(body: String) -> String {
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
        .route("/api/games/:id", get(forward_game)) // or protect only write ops
        .route_layer(middleware::from_fn(auth_middleware));

    let app = Router::new()
        // public
        .route("/api/games", get(forward_games))

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