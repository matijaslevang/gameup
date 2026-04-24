use axum::{
    routing::post,
    Router,
    Json,
    extract::State,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::{env, time::{SystemTime, UNIX_EPOCH}};
use tokio::net::TcpListener;
use jsonwebtoken::{encode, Header, EncodingKey};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, PasswordHash};
use argon2::password_hash::rand_core::OsRng;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    jwt_secret: String,
}

#[derive(Deserialize)]
struct RegisterRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(sqlx::FromRow)]
struct User {
    username: String,
    password_hash: String,
    role: String,
}

#[derive(Serialize)]
struct AuthResponse {
    token: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    role: String,
    exp: usize,
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<&'static str, StatusCode> {
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();

    sqlx::query(
        "INSERT INTO users (username, password_hash, role) VALUES ($1, $2, 'user')"
    )
    .bind(payload.username)
    .bind(password_hash)
    .execute(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok("User registered")
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, password_hash, role FROM users WHERE username = $1"
    )
    .bind(&payload.username)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() + 60 * 60; // 1 hour

    let claims = Claims {
        sub: user.username,
        role: user.role,
        exp: exp as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse { token }))
}

#[tokio::main]
async fn main() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to DB");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username TEXT NOT NULL,
            password_hash TEXT NOT NULL,
            role TEXT NOT NULL
        );
        "#
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO users (username, password_hash, role)
        VALUES ($1, $2, 'admin')
        ON CONFLICT (username) DO NOTHING"
    )
    .bind("asd")
    .bind({
        use argon2::{Argon2, PasswordHasher};
        use argon2::password_hash::{SaltString, rand_core::OsRng};

        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password("asd".as_bytes(), &salt)
            .unwrap()
            .to_string()
    })
    .execute(&pool)
    .await
    .unwrap();

    let state = AppState { pool, jwt_secret };

    let app = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8002").await.unwrap();

    println!("Auth service running on 8002");

    axum::serve(listener, app).await.unwrap();
}