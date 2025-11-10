use axum::{
    extract::Extension,
    response::Json,
    http::StatusCode,
};
use axum_extra::extract::Multipart;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;
use crate::rom_scanner::{scan_roms_directory, get_directory_for_extension, RomFile};
use crate::emulators::get_emulator_by_id;
use crate::games::Game;

#[derive(Serialize)]
pub struct ScanResult {
    pub total_found: usize,
    pub newly_added: usize,
    pub already_exists: usize,
    pub errors: Vec<String>,
}

#[derive(Serialize)]
pub struct UploadResult {
    pub success: bool,
    pub message: String,
    pub game_id: Option<i32>,
    pub file_path: Option<String>,
}

#[derive(Deserialize)]
pub struct ScanRequest {
    pub rescan: Option<bool>,
}

/// Scan ROMs directory and add discovered ROMs to database
pub async fn scan_roms(
    Extension(pool): Extension<Arc<PgPool>>,
) -> Result<Json<ScanResult>, StatusCode> {
    let roms_path = std::env::var("ROMS_PATH").unwrap_or_else(|_| "/roms".to_string());
    let base_path = Path::new(&roms_path);
    
    if !base_path.exists() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    let discovered_roms = scan_roms_directory(base_path);
    let total_found = discovered_roms.len();
    let mut newly_added = 0;
    let mut already_exists = 0;
    let mut errors = Vec::new();
    
    for rom in discovered_roms {
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
            already_exists += 1;
            continue;
        }
        
        // Get emulator info
        let emulator_id = match &rom.suggested_emulator {
            Some(id) => id,
            None => {
                errors.push(format!("No emulator found for {}", rom.file_name));
                continue;
            }
        };
        
        let emulator = match get_emulator_by_id(emulator_id) {
            Some(emu) => emu,
            None => {
                errors.push(format!("Invalid emulator ID for {}", rom.file_name));
                continue;
            }
        };
        
        let emulator_type_str = match emulator.emulator_type {
            crate::emulators::EmulatorType::RetroArchCore => "RetroArchCore",
            crate::emulators::EmulatorType::EmulatorJS => "EmulatorJS",
            crate::emulators::EmulatorType::NativeService => "NativeService",
            crate::emulators::EmulatorType::BrowserWASM => "BrowserWASM",
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
            Ok(_) => newly_added += 1,
            Err(e) => errors.push(format!("Failed to add {}: {}", rom.file_name, e)),
        }
    }
    
    Ok(Json(ScanResult {
        total_found,
        newly_added,
        already_exists,
        errors,
    }))
}

/// Upload a ROM file
pub async fn upload_rom(
    Extension(pool): Extension<Arc<PgPool>>,
    mut multipart: Multipart,
) -> Result<Json<UploadResult>, StatusCode> {
    let roms_path = std::env::var("ROMS_PATH").unwrap_or_else(|_| "/roms".to_string());
    let base_path = PathBuf::from(&roms_path);
    
    let mut file_name: Option<String> = None;
    let mut file_data: Option<Vec<u8>> = None;
    let mut console: Option<String> = None;
    let mut title: Option<String> = None;
    
    // Parse multipart form
    while let Some(field) = multipart.next_field().await.ok().flatten() {
        let name = field.name().unwrap_or("").to_string();
        
        match name.as_str() {
            "file" => {
                file_name = field.file_name().map(|s| s.to_string());
                file_data = Some(field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?.to_vec());
            }
            "console" => {
                console = Some(
                    String::from_utf8(field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?.to_vec())
                        .map_err(|_| StatusCode::BAD_REQUEST)?
                );
            }
            "title" => {
                title = Some(
                    String::from_utf8(field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?.to_vec())
                        .map_err(|_| StatusCode::BAD_REQUEST)?
                );
            }
            _ => {}
        }
    }
    
    let file_name = file_name.ok_or(StatusCode::BAD_REQUEST)?;
    let file_data = file_data.ok_or(StatusCode::BAD_REQUEST)?;
    
    // Determine console directory
    let extension = Path::new(&file_name)
        .extension()
        .and_then(|e| e.to_str())
        .ok_or(StatusCode::BAD_REQUEST)?
        .to_lowercase();
    
    let target_dir = if let Some(console_dir) = console {
        console_dir
    } else {
        get_directory_for_extension(&extension)
            .ok_or(StatusCode::BAD_REQUEST)?
            .to_string()
    };
    
    // Create target directory path
    let target_path = base_path.join(&target_dir);
    tokio::fs::create_dir_all(&target_path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Save file
    let file_path = target_path.join(&file_name);
    let mut file = tokio::fs::File::create(&file_path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    file.write_all(&file_data)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Scan and add to database
    let relative_path = format!("{}/{}", target_dir, file_name);
    
    // Get system name and emulator
    let system_map = crate::rom_scanner::get_system_mapping();
    let system_name = system_map.get(target_dir.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    // Find compatible emulator
    let emulators = crate::emulators::get_all_emulators();
    let emulator = emulators
        .iter()
        .find(|e| {
            e.system.to_lowercase().contains(&system_name.to_lowercase())
                && e.supported_formats.iter().any(|fmt| fmt == &extension)
        })
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    let emulator_type_str = match emulator.emulator_type {
        crate::emulators::EmulatorType::RetroArchCore => "RetroArchCore",
        crate::emulators::EmulatorType::EmulatorJS => "EmulatorJS",
        crate::emulators::EmulatorType::NativeService => "NativeService",
        crate::emulators::EmulatorType::BrowserWASM => "BrowserWASM",
    };
    
    // Use provided title or clean filename
    let game_title = title.unwrap_or_else(|| {
        Path::new(&file_name)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .to_string()
    });
    
    // Insert into database
    let game = sqlx::query_as::<_, Game>(
        "INSERT INTO games (title, system, file_path, emulator_id, emulator_type, file_size) 
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING *"
    )
    .bind(&game_title)
    .bind(system_name)
    .bind(&relative_path)
    .bind(&emulator.id)
    .bind(emulator_type_str)
    .bind(file_data.len() as i64)
    .fetch_one(pool.as_ref())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(UploadResult {
        success: true,
        message: format!("Successfully uploaded {}", file_name),
        game_id: Some(game.id as i32),
        file_path: Some(relative_path),
    }))
}

/// Get available console directories
#[derive(Serialize)]
pub struct ConsoleInfo {
    pub id: String,
    pub name: String,
    pub supported_formats: Vec<String>,
}

pub async fn get_consoles() -> Json<Vec<ConsoleInfo>> {
    let emulators = crate::emulators::get_all_emulators();
    let system_map = crate::rom_scanner::get_system_mapping();
    
    let mut consoles: Vec<ConsoleInfo> = system_map
        .iter()
        .map(|(id, name)| {
            let formats: Vec<String> = emulators
                .iter()
                .filter(|e| e.system.to_lowercase().contains(&name.to_lowercase()))
                .flat_map(|e| e.supported_formats.clone())
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();
            
            ConsoleInfo {
                id: id.to_string(),
                name: name.to_string(),
                supported_formats: formats,
            }
        })
        .collect();
    
    consoles.sort_by(|a, b| a.name.cmp(&b.name));
    
    Json(consoles)
}

