use axum::{routing::{get, post}, Json, Router};
use serde::Serialize;
use sqlx::PgPool;
use tokio::net::TcpListener;
use std::env;
use std::time::Duration;
use tokio::time::sleep;
use axum::extract::State;
use serde::Deserialize;
use axum::http::Method;
use axum::extract::Path;

#[derive(Serialize, sqlx::FromRow)]
struct Game {
    id: i32,
    name: String,
    genre: String,
}

#[derive(Deserialize)]
struct CreateGame {
    name: String,
    genre: String,
}

async fn get_games(pool: axum::extract::State<PgPool>) -> Json<Vec<Game>> {
    let games = sqlx::query_as::<_, Game>("SELECT id, name, genre FROM games")
        .fetch_all(&pool.0)
        .await
        .unwrap();

    Json(games)
}

async fn create_game(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateGame>,
) -> &'static str {
    sqlx::query("INSERT INTO games (name, genre) VALUES ($1, $2)")
        .bind(payload.name)
        .bind(payload.genre)
        .execute(&pool)
        .await
        .unwrap();

    "Game created"
}

async fn get_game(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
) -> Json<Game> {
    let game = sqlx::query_as::<_, Game>(
        "SELECT id, name, genre FROM games WHERE id = $1"
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    .unwrap();

    Json(game)
}

#[tokio::main]
async fn main() {
    let database_url = env::var("DATABASE_URL").unwrap();

    let pool = loop {
        match PgPool::connect(&database_url).await {
            Ok(pool) => break pool,
            Err(_) => {
                println!("Waiting for database...");
                sleep(Duration::from_secs(2)).await;
            }
        }
    };

    let app = Router::new()
        .route("/games", get(get_games).post(create_game))
        .route("/games/:id", get(get_game))
        .with_state(pool);

    let listener = TcpListener::bind("0.0.0.0:8001").await.unwrap();

    println!("Game service running on 0.0.0.0:8001");

    axum::serve(listener, app).await.unwrap();
}