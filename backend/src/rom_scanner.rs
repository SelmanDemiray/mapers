use std::path::{Path, PathBuf};
use std::collections::HashMap;
use walkdir::WalkDir;
use regex;
use crate::emulators::{get_all_emulators, EmulatorInfo};

/// Maps directory names to standardized system names
pub fn get_system_mapping() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    
    // Nintendo
    map.insert("nes", "Nintendo Entertainment System");
    map.insert("snes", "Super Nintendo Entertainment System");
    map.insert("n64", "Nintendo 64");
    map.insert("gamecube", "GameCube / Wii");
    map.insert("wii", "GameCube / Wii");
    map.insert("gb", "Game Boy / Game Boy Color");
    map.insert("gbc", "Game Boy / Game Boy Color");
    map.insert("gba", "Game Boy Advance");
    map.insert("nds", "Nintendo DS");
    map.insert("3ds", "Nintendo 3DS");
    map.insert("switch", "Nintendo Switch");
    
    // Sony
    map.insert("ps1", "PlayStation 1");
    map.insert("ps2", "PlayStation 2");
    map.insert("ps3", "PlayStation 3");
    map.insert("psp", "PlayStation Portable");
    map.insert("psvita", "PlayStation Vita");
    
    // Sega
    map.insert("dreamcast", "Sega Dreamcast");
    map.insert("sega-genesis", "Sega Genesis");
    map.insert("sega-cd", "Sega CD");
    map.insert("sega-saturn", "Sega Saturn");
    
    // Arcade & Other
    map.insert("arcade", "Arcade");
    map.insert("mame", "Arcade");
    map.insert("neogeo", "Neo Geo");
    map.insert("atari2600", "Atari 2600");
    map.insert("atari7800", "Atari 7800");
    map.insert("turbografx16", "TurboGrafx-16");
    map.insert("turbografx-cd", "TurboGrafx-CD");
    map.insert("wonderswan", "WonderSwan");
    
    map
}

/// ROM file information
#[derive(Debug, Clone)]
pub struct RomFile {
    pub file_path: String,
    pub file_name: String,
    pub system: String,
    pub extension: String,
    pub size: u64,
    pub suggested_emulator: Option<String>,
}

/// Scan a directory for ROM files
pub fn scan_roms_directory(base_path: &Path) -> Vec<RomFile> {
    let mut roms = Vec::new();
    let system_map = get_system_mapping();
    let emulators = get_all_emulators();
    
    // Scan each subdirectory
    for entry in std::fs::read_dir(base_path).ok().into_iter().flatten() {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        
        let dir_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name,
            None => continue,
        };
        
        // Get system name from directory
        let system_name = match system_map.get(dir_name) {
            Some(&name) => name,
            None => continue, // Skip unknown directories
        };
        
        // Scan all files in this directory
        for file_entry in WalkDir::new(&path)
            .max_depth(3)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let file_path = file_entry.path();
            let extension = file_path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();
            
            // Find compatible emulator
            let suggested_emulator = find_compatible_emulator(&emulators, system_name, &extension);
            
            // Skip if no compatible emulator found
            if suggested_emulator.is_none() {
                continue;
            }
            
            let file_size = std::fs::metadata(file_path)
                .ok()
                .map(|m| m.len())
                .unwrap_or(0);
            
            let relative_path = file_path
                .strip_prefix(base_path)
                .unwrap_or(file_path)
                .to_string_lossy()
                .replace('\\', "/");
            
            let file_name = file_path
                .file_stem()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string();
            
            roms.push(RomFile {
                file_path: relative_path,
                file_name: clean_filename(&file_name),
                system: system_name.to_string(),
                extension,
                size: file_size,
                suggested_emulator,
            });
        }
    }
    
    roms
}

/// Find a compatible emulator for a given system and file extension
fn find_compatible_emulator(
    emulators: &[EmulatorInfo],
    system_name: &str,
    extension: &str,
) -> Option<String> {
    emulators
        .iter()
        .find(|e| {
            e.system.to_lowercase().contains(&system_name.to_lowercase())
                && e.supported_formats.iter().any(|fmt| fmt == extension)
        })
        .map(|e| e.id.clone())
}

/// Clean up filename for display (remove common tags and underscores)
fn clean_filename(name: &str) -> String {
    // Remove common ROM tags like (USA), [!], etc.
    let cleaned = regex::Regex::new(r"\([^)]*\)|\[[^\]]*\]|\{[^}]*\}")
        .unwrap()
        .replace_all(name, "");
    
    // Replace underscores and dots with spaces
    let cleaned = cleaned.replace('_', " ").replace('.', " ");
    
    // Remove multiple spaces
    let cleaned = regex::Regex::new(r"\s+")
        .unwrap()
        .replace_all(&cleaned, " ");
    
    cleaned.trim().to_string()
}

/// Get the appropriate directory for a given file extension
pub fn get_directory_for_extension(extension: &str) -> Option<&'static str> {
    match extension {
        // NES
        "nes" | "fds" | "unf" | "unif" => Some("nes"),
        
        // SNES
        "smc" | "sfc" | "fig" | "swc" | "bs" => Some("snes"),
        
        // N64
        "n64" | "z64" | "v64" | "u64" => Some("n64"),
        
        // GameCube/Wii
        "iso" | "gcm" | "wbfs" | "ciso" | "gcz" | "wad" => Some("gamecube"),
        
        // Game Boy
        "gb" => Some("gb"),
        "gbc" => Some("gbc"),
        
        // Game Boy Advance
        "gba" => Some("gba"),
        
        // Nintendo DS
        "nds" | "dsi" => Some("nds"),
        
        // 3DS
        "3ds" | "cci" | "cxi" | "app" => Some("3ds"),
        
        // Switch
        "nsp" | "xci" | "nca" => Some("switch"),
        
        // PlayStation 1
        "cue" | "bin" | "chd" | "m3u" | "pbp" => Some("ps1"),
        
        // PlayStation 2
        "mdf" | "nrg" | "gz" | "cso" => Some("ps2"),
        
        // PlayStation 3
        "pkg" | "rap" | "edat" | "sdat" => Some("ps3"),
        
        // PSP
        "elf" => Some("psp"),
        
        // PS Vita
        "vpk" => Some("psvita"),
        
        // Dreamcast
        "cdi" | "gdi" => Some("dreamcast"),
        
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_filename() {
        assert_eq!(
            clean_filename("Super_Mario_Bros_(USA)_[!]"),
            "Super Mario Bros"
        );
        assert_eq!(
            clean_filename("Legend.of.Zelda"),
            "Legend of Zelda"
        );
    }

    #[test]
    fn test_get_directory_for_extension() {
        assert_eq!(get_directory_for_extension("nes"), Some("nes"));
        assert_eq!(get_directory_for_extension("gba"), Some("gba"));
        assert_eq!(get_directory_for_extension("iso"), Some("gamecube"));
        assert_eq!(get_directory_for_extension("xyz"), None);
    }
}

