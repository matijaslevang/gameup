use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};

use serde::{Deserialize, Serialize};

use sqlx::{PgPool, types::chrono::NaiveDate};

use tokio::{
    net::TcpListener,
    time::{sleep, Duration},
};

use std::env;

#[derive(Serialize)]
struct CreateGameResponse {
    id: i32,
}

#[derive(Serialize, sqlx::FromRow)]
struct Game {
    id: i32,
    name: String,
    genre: String,
    description: String,
    release_date: NaiveDate,
    video_url: Option<String>,
}

#[derive(Deserialize)]
struct CreateGame {
    name: String,
    genre: String,
    description: Option<String>,
    release_date: Option<NaiveDate>,
}

#[derive(Deserialize)]
struct GameQuery {
    name: Option<String>,
    genre: Option<String>,
}

async fn get_games(
    State(pool): State<PgPool>,
    Query(params): Query<GameQuery>,
) -> Json<Vec<Game>> {
    let mut query = String::from(
        "SELECT id, name, genre, description, release_date, video_url FROM games"
    );

    let mut conditions = vec![];
    let mut values: Vec<String> = vec![];

    if let Some(name) = params.name {
        if !name.trim().is_empty() {
            conditions.push(format!("name ILIKE ${}", values.len() + 1));
            values.push(format!("%{}%", name));
        }
    }

    if let Some(genre) = params.genre {
        if !genre.trim().is_empty() {
            conditions.push(format!("genre = ${}", values.len() + 1));
            values.push(genre);
        }
    }

    if !conditions.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&conditions.join(" AND "));
    }

    let mut q = sqlx::query_as::<_, Game>(&query);

    for value in values {
        q = q.bind(value);
    }

    let games = q.fetch_all(&pool).await.unwrap();

    Json(games)
}

async fn create_game(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateGame>,
) -> Json<CreateGameResponse> {
    let id: i32 = sqlx::query_scalar(
        r#"
        INSERT INTO games (name, genre, description, release_date)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#
    )
    .bind(payload.name)
    .bind(payload.genre)
    .bind(payload.description)
    .bind(payload.release_date)
    .fetch_one(&pool)
    .await
    .unwrap();

    Json(CreateGameResponse { id })
}

async fn get_game(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
) -> Json<Game> {
    let game = sqlx::query_as::<_, Game>(
        "SELECT id, name, genre, description, release_date, video_url FROM games WHERE id = $1"
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    .unwrap();

    Json(game)
}

async fn delete_game(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
) -> StatusCode {
    let result = sqlx::query("DELETE FROM games WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn update_game(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
    Json(payload): Json<CreateGame>,
) -> StatusCode {
    let result = sqlx::query(
        r#"
        UPDATE games
        SET name = $1,
            genre = $2,
            description = $3,
            release_date = $4
        WHERE id = $5
        "#
    )
    .bind(payload.name)
    .bind(payload.genre)
    .bind(payload.description)
    .bind(payload.release_date)
    .bind(id)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
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

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS games (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            genre TEXT NOT NULL,
            description TEXT NOT NULL,
            release_date DATE NOT NULL,
            video_url TEXT,
            created_at TIMESTAMP DEFAULT NOW()
        );
        "#
    )
    .execute(&pool)
    .await
    .unwrap();

    let app = Router::new()
        .route("/games", get(get_games).post(create_game))
        .route("/games/:id", get(get_game).put(update_game).delete(delete_game))
        .with_state(pool);

    let listener = TcpListener::bind("0.0.0.0:8001").await.unwrap();

    println!("Game service running on 0.0.0.0:8001");

    axum::serve(listener, app).await.unwrap();
}