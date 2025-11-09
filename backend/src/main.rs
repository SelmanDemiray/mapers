use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
    response::Json,
};
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

mod emulators;
mod games;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://emulator_user:secure_password@localhost:5432/emulator_platform".to_string());
    
    let pool = PgPool::connect(&database_url).await?;
    let pool = Arc::new(pool);
    
    let app = Router::new()
        .route("/api/emulators", get(get_emulators))
        .route("/api/games", get(games::get_games).post(games::add_game))
        .route("/api/games/:id", get(games::get_game_by_id))
        .layer(Extension(pool))
        .layer(CorsLayer::permissive());
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server running on http://0.0.0.0:8080");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn get_emulators() -> Json<Vec<emulators::EmulatorInfo>> {
    Json(emulators::get_all_emulators())
}

