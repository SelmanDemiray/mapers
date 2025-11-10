use axum::{
    extract::{Extension, Path, Query},
    response::Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use crate::emulators::{get_emulator_by_id, EmulatorInfo};

#[derive(Serialize, sqlx::FromRow)]
pub struct Game {
    pub id: i32,
    pub title: String,
    pub system: String,
    pub file_path: String,
    pub emulator_id: String,
    pub emulator_type: String,
    pub added_at: chrono::NaiveDateTime,
    #[sqlx(default)]
    pub user_id: Option<i32>,
    #[sqlx(default)]
    pub file_size: Option<i64>,
    #[sqlx(default)]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct GameWithEmulator {
    #[serde(flatten)]
    pub game: Game,
    pub emulator: EmulatorInfo,
    pub launch_url: String,
}

#[derive(Deserialize)]
pub struct GameQuery {
    system: Option<String>,
    emulator: Option<String>,
}

pub async fn get_games(
    Extension(pool): Extension<Arc<PgPool>>,
    Query(query): Query<GameQuery>,
) -> Result<Json<Vec<GameWithEmulator>>, axum::http::StatusCode> {
    let games: Vec<Game> = if let Some(system) = &query.system {
        if let Some(emulator) = &query.emulator {
            sqlx::query_as("SELECT * FROM games WHERE system = $1 AND emulator_id = $2 ORDER BY title")
                .bind(system)
                .bind(emulator)
                .fetch_all(pool.as_ref())
                .await
        } else {
            sqlx::query_as("SELECT * FROM games WHERE system = $1 ORDER BY title")
                .bind(system)
                .fetch_all(pool.as_ref())
                .await
        }
    } else if let Some(emulator) = &query.emulator {
        sqlx::query_as("SELECT * FROM games WHERE emulator_id = $1 ORDER BY title")
            .bind(emulator)
            .fetch_all(pool.as_ref())
            .await
    } else {
        sqlx::query_as("SELECT id, title, system, file_path, emulator_id, emulator_type, added_at, user_id, file_size, metadata FROM games ORDER BY title")
            .fetch_all(pool.as_ref())
            .await
    }
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    let games_with_emulators: Vec<GameWithEmulator> = games
        .into_iter()
        .filter_map(|game| {
            get_emulator_by_id(&game.emulator_id).map(|emulator| {
                let launch_url = match emulator.emulator_type {
                    crate::emulators::EmulatorType::RetroArchCore => {
                        format!("http://localhost:8081/play?core={}&rom={}", 
                            emulator.core, 
                            urlencoding::encode(&game.file_path))
                    }
                    crate::emulators::EmulatorType::EmulatorJS | 
                    crate::emulators::EmulatorType::BrowserWASM => {
                        format!("http://localhost:8082/?rom={}&core={}", 
                            urlencoding::encode(&game.file_path),
                            emulator.core)
                    }
                    crate::emulators::EmulatorType::NativeService => {
                        format!("http://localhost:{}/launch?rom={}", 
                            emulator.service_port.unwrap_or(8080),
                            urlencoding::encode(&game.file_path))
                    }
                };
                
                GameWithEmulator {
                    game,
                    emulator,
                    launch_url,
                }
            })
        })
        .collect();
    
    Ok(Json(games_with_emulators))
}

pub async fn get_game_by_id(
    Extension(pool): Extension<Arc<PgPool>>,
    Path(id): Path<i32>,
) -> Result<Json<GameWithEmulator>, axum::http::StatusCode> {
    let game: Game = sqlx::query_as("SELECT * FROM games WHERE id = $1")
        .bind(id)
        .fetch_optional(pool.as_ref())
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;
    
    let emulator = get_emulator_by_id(&game.emulator_id)
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;
    
    let launch_url = match emulator.emulator_type {
        crate::emulators::EmulatorType::RetroArchCore => {
            format!("http://localhost:8081/play?core={}&rom={}", 
                emulator.core, 
                urlencoding::encode(&game.file_path))
        }
        crate::emulators::EmulatorType::EmulatorJS | 
        crate::emulators::EmulatorType::BrowserWASM => {
            format!("http://localhost:8082/?rom={}&core={}", 
                urlencoding::encode(&game.file_path),
                emulator.core)
        }
        crate::emulators::EmulatorType::NativeService => {
            format!("http://localhost:{}/launch?rom={}", 
                emulator.service_port.unwrap_or(8080),
                urlencoding::encode(&game.file_path))
        }
    };
    
    Ok(Json(GameWithEmulator {
        game,
        emulator,
        launch_url,
    }))
}

#[derive(Deserialize)]
pub struct AddGameRequest {
    title: String,
    system: String,
    file_path: String,
    emulator_id: String,
}

pub async fn add_game(
    Extension(pool): Extension<Arc<PgPool>>,
    Json(payload): Json<AddGameRequest>,
) -> Result<Json<Game>, axum::http::StatusCode> {
    // Validate emulator exists and get emulator type
    let emulator = get_emulator_by_id(&payload.emulator_id)
        .ok_or(axum::http::StatusCode::BAD_REQUEST)?;
    
    let emulator_type_str = match emulator.emulator_type {
        crate::emulators::EmulatorType::RetroArchCore => "RetroArchCore",
        crate::emulators::EmulatorType::EmulatorJS => "EmulatorJS",
        crate::emulators::EmulatorType::NativeService => "NativeService",
        crate::emulators::EmulatorType::BrowserWASM => "BrowserWASM",
    };
    
    let game = sqlx::query_as::<_, Game>(
        "INSERT INTO games (title, system, file_path, emulator_id, emulator_type) 
         VALUES ($1, $2, $3, $4, $5)
         RETURNING *"
    )
    .bind(&payload.title)
    .bind(&payload.system)
    .bind(&payload.file_path)
    .bind(&payload.emulator_id)
    .bind(emulator_type_str)
    .fetch_one(pool.as_ref())
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(game))
}

