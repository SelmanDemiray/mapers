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
mod rom_scanner;
mod roms;
mod sessions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://emulator_user:secure_password@localhost:5432/emulator_platform".to_string());
    
    let pool = PgPool::connect(&database_url).await?;
    let pool = Arc::new(pool);
    
    // Perform initial ROM scan on startup and add to database
    println!("Performing initial ROM scan...");
    let roms_path = std::env::var("ROMS_PATH").unwrap_or_else(|_| "/roms".to_string());
    if std::path::Path::new(&roms_path).exists() {
        let discovered = rom_scanner::scan_roms_directory(std::path::Path::new(&roms_path));
        println!("Found {} ROM files", discovered.len());
        
        // Add discovered ROMs to database
        let mut added = 0;
        let mut skipped = 0;
        for rom in discovered {
            // Check if already exists
            let exists: Option<(i64,)> = sqlx::query_as(
                "SELECT id FROM games WHERE file_path = $1"
            )
            .bind(&rom.file_path)
            .fetch_optional(pool.as_ref())
            .await
            .ok()
            .flatten();
            
            if exists.is_some() {
                skipped += 1;
                continue;
            }
            
            // Get emulator info
            let emulator_id = match &rom.suggested_emulator {
                Some(id) => id,
                None => {
                    println!("Warning: No emulator found for {}", rom.file_name);
                    continue;
                }
            };
            
            let emulator = match emulators::get_emulator_by_id(emulator_id) {
                Some(emu) => emu,
                None => {
                    println!("Warning: Invalid emulator ID for {}", rom.file_name);
                    continue;
                }
            };
            
            let emulator_type_str = match emulator.emulator_type {
                emulators::EmulatorType::RetroArchCore => "RetroArchCore",
                emulators::EmulatorType::EmulatorJS => "EmulatorJS",
                emulators::EmulatorType::NativeService => "NativeService",
                emulators::EmulatorType::BrowserWASM => "BrowserWASM",
            };
            
            // Insert into database
            let result = sqlx::query(
                "INSERT INTO games (title, system, file_path, emulator_id, emulator_type, file_size) 
                 VALUES ($1, $2, $3, $4, $5, $6)"
            )
            .bind(&rom.file_name)
            .bind(&rom.system)
            .bind(&rom.file_path)
            .bind(emulator_id)
            .bind(emulator_type_str)
            .bind(rom.size as i64)
            .execute(pool.as_ref())
            .await;
            
            match result {
                Ok(_) => {
                    added += 1;
                    println!("Added: {} ({})", rom.file_name, rom.system);
                }
                Err(e) => {
                    println!("Error adding {}: {}", rom.file_name, e);
                }
            }
        }
        println!("Initial scan complete: {} added, {} already in database", added, skipped);
    }
    
    // Start background task to clean up old sessions
    let pool_cleanup = pool.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(300)); // Every 5 minutes
        loop {
            interval.tick().await;
            sessions::cleanup_old_sessions(pool_cleanup.clone()).await;
        }
    });
    
    let app = Router::new()
        .route("/api/emulators", get(get_emulators))
        .route("/api/games", get(games::get_games).post(games::add_game))
        .route("/api/games/:id", get(games::get_game_by_id))
        .route("/api/roms/scan", post(roms::scan_roms))
        .route("/api/roms/upload", post(roms::upload_rom))
        .route("/api/roms/consoles", get(roms::get_consoles))
        .route("/api/auth/login", post(sessions::login))
        .route("/api/auth/previous-usernames", get(sessions::get_previous_usernames))
        .route("/api/auth/delete-account", post(sessions::delete_account))
        .route("/api/sessions/register", post(sessions::register_session))
        .route("/api/sessions/connected", get(sessions::get_connected_users))
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

