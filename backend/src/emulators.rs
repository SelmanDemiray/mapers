use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct EmulatorInfo {
    pub id: String,
    pub name: String,
    pub system: String,
    pub core: String,
    pub supported_formats: Vec<String>,
    pub emulator_type: EmulatorType,
    pub service_port: Option<u16>,
    pub github_url: String,
    pub license: String,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum EmulatorType {
    RetroArchCore,
    EmulatorJS,
    NativeService,
    BrowserWASM,
}

pub fn get_all_emulators() -> Vec<EmulatorInfo> {
    vec![
        // ========== NINTENDO SYSTEMS ==========
        
        // NES
        EmulatorInfo {
            id: "fceux".to_string(),
            name: "FCEUX".to_string(),
            system: "Nintendo Entertainment System".to_string(),
            core: "fceumm_libretro".to_string(),
            supported_formats: vec!["nes", "fds", "unf", "unif"].into_iter().map(String::from).collect(),
            emulator_type: EmulatorType::RetroArchCore,
            service_port: Some(8081),
            github_url: "https://github.com/TASEmulators/fceux".to_string(),
            license: "GPLv2".to_string(),
        },
        EmulatorInfo {
            id: "nestopia".to_string(),
            name: "Nestopia UE".to_string(),
            system: "Nintendo Entertainment System".to_string(),
            core: "nestopia_libretro".to_string(),
            supported_formats: vec!["nes", "fds"].into_iter().map(String::from).collect(),
            emulator_type: EmulatorType::RetroArchCore,
            service_port: Some(8081),
            github_url: "https://github.com/libretro/nestopia".to_string(),
            license: "GPLv2".to_string(),
        },
        
        // SNES
        EmulatorInfo {
            id: "snes9x".to_string(),
            name: "Snes9x".to_string(),
            system: "Super Nintendo Entertainment System".to_string(),
            core: "snes9x_libretro".to_string(),
            supported_formats: vec!["smc", "sfc", "fig", "swc", "bs"].into_iter().map(String::from).collect(),
            emulator_type: EmulatorType::RetroArchCore,
            service_port: Some(8081),
            github_url: "https://github.com/snes9xgit/snes9x".to_string(),
            license: "GPLv2+".to_string(),
        },
        EmulatorInfo {
            id: "higan-snes".to_string(),
            name: "higan (SNES)".to_string(),
            system: "Super Nintendo Entertainment System".to_string(),
            core: "bsnes_mercury_accuracy_libretro".to_string(),
            supported_formats: vec!["smc", "sfc", "bs"].into_iter().map(String::from).collect(),
            emulator_type: EmulatorType::RetroArchCore,
            service_port: Some(8081),
            github_url: "https://github.com/higan-emu/higan".to_string(),
            license: "GPLv3".to_string(),
        },
        
        // N64
        EmulatorInfo {
            id: "mupen64plus".to_string(),
            name: "Mupen64Plus".to_string(),
            system: "Nintendo 64".to_string(),
            core: "mupen64plus_next_libretro".to_string(),
            supported_formats: vec!["n64", "z64", "v64", "u64"].into_iter().map(String::from).collect(),
            emulator_type: EmulatorType::RetroArchCore,
            service_port: Some(8081),
            github_url: "https://github.com/mupen64plus".to_string(),
            license: "GPLv2".to_string(),
        },
        
        // GameCube/Wii
        EmulatorInfo {
            id: "dolphin".to_string(),
            name: "Dolphin".to_string(),
            system: "GameCube / Wii".to_string(),
            core: "dolphin_libretro".to_string(),
            supported_formats: vec!["iso", "gcm", "wbfs", "ciso", "gcz", "wad"].into_iter().map(String::from).collect(),
            emulator_type: EmulatorType::NativeService,
            service_port: Some(8083),
            github_url: "https://github.com/dolphin-emu/dolphin".to_string(),
            license: "GPLv2+".to_string(),
        },
        
        // Game Boy / Game Boy Color
        EmulatorInfo {
            id: "sameboy".to_string(),
            name: "SameBoy".to_string(),
            system: "Game Boy / Game Boy Color".to_string(),
            core: "sameboy_libretro".to_string(),
            supported_formats: vec!["gb", "gbc"].into_iter().map(String::from).collect(),
            emulator_type: EmulatorType::RetroArchCore,
            service_port: Some(8081),
            github_url: "https://github.com/LIJI32/SameBoy".to_string(),
            license: "MIT".to_string(),
        },
        
        // Game Boy Advance
        EmulatorInfo {
            id: "mgba".to_string(),
            name: "mGBA".to_string(),
            system: "Game Boy Advance".to_string(),
            core: "mgba_libretro".to_string(),
            supported_formats: vec!["gba", "gb", "gbc"].into_iter().map(String::from).collect(),
            emulator_type: EmulatorType::RetroArchCore,
            service_port: Some(8081),
            github_url: "https://github.com/mgba-emu/mgba".to_string(),
            license: "MPLv2".to_string(),
        },
        
        // Nintendo DS
        EmulatorInfo {
            id: "desmume".to_string(),
            name: "DeSmuME".to_string(),
            system: "Nintendo DS".to_string(),
            core: "desmume_libretro".to_string(),
            supported_formats: vec!["nds", "dsi"].into_iter().map(String::from).collect(),
            emulator_type: EmulatorType::RetroArchCore,
            service_port: Some(8081),
            github_url: "https://github.com/TASEmulators/desmume".to_string(),
            license: "GPLv2".to_string(),
        },
        EmulatorInfo {
            id: "melonds".to_string(),
            name: "melonDS".to_string(),
            system: "Nintendo DS / DSi".to_string(),
            core: "melonds_libretro".to_string(),
            supported_formats: vec!["nds", "dsi"].into_iter().map(String::from).collect(),
            emulator_type: EmulatorType::RetroArchCore,
            service_port: Some(8081),
            github_url: "https://github.com/melonDS-emu/melonDS".to_string(),
            license: "GPLv3".to_string(),
        },
        
        // Nintendo 3DS
        EmulatorInfo {
            id: "citra".to_string(),
            name: "Citra".to_string(),
            system: "Nintendo 3DS".to_string(),
            core: "citra_libretro".to_string(),
            supported_formats: vec!["3ds", "cci", "cxi", "app"].into_iter().map(String::from).collect(),
            emulator_type: EmulatorType::NativeService,
            service_port: Some(8087),
            github_url: "https://citra-emulator.com".to_string(),
            license: "GPLv2".to_string(),
        },
        
        // Nintendo Switch
        EmulatorInfo {
            id: "yuzu".to_string(),
            name: "yuzu".to_string(),
            system: "Nintendo Switch".to_string(),
            core: "yuzu".to_string(),
            supported_formats: vec!["nsp", "xci", "nca"].into_iter().map(String::from).collect(),
            emulator_type: EmulatorType::NativeService,
            service_port: Some(8088),
            github_url: "https://github.com/yuzu-emu/yuzu".to_string(),
            license: "GPLv2".to_string(),
        },
        EmulatorInfo {
            id: "ryujinx".to_string(),
            name: "Ryujinx".to_string(),
            system: "Nintendo Switch".to_string(),
            core: "ryujinx".to_string(),
            supported_formats: vec!["nsp", "xci", "nca"].into_iter().map(String::from).collect(),
            emulator_type: EmulatorType::NativeService,
            service_port: Some(8089),
            github_url: "https://github.com/Ryujinx/Ryujinx".to_string(),
            license: "MIT".to_string(),
        },
        
        // ========== SONY SYSTEMS ==========
        
        // PlayStation 1
        EmulatorInfo {
            id: "duckstation".to_string(),
            name: "DuckStation".to_string(),
            system: "PlayStation 1".to_string(),
            core: "duckstation_libretro".to_string(),
            supported_formats: vec!["cue", "bin", "iso", "chd", "m3u", "pbp"].into_iter().map(String::from).collect(),
            emulator_type: EmulatorType::RetroArchCore,
            service_port: Some(8081),
            github_url: "https://github.com/stenzek/duckstation".to_string(),
            license: "GPLv3".to_string(),
        },
        
        // PlayStation 2
        EmulatorInfo {
            id: "pcsx2".to_string(),
            name: "PCSX2".to_string(),
            system: "PlayStation 2".to_string(),
            core: "pcsx2".to_string(),
            supported_formats: vec!["iso", "bin", "mdf", "nrg", "gz", "cso"].into_iter().map(String::from).collect(),
            emulator_type: EmulatorType::NativeService,
            service_port: Some(8084),
            github_url: "https://github.com/PCSX2/pcsx2".to_string(),
            license: "GPLv3".to_string(),
        },
        
        // PlayStation 3
        EmulatorInfo {
            id: "rpcs3".to_string(),
            name: "RPCS3".to_string(),
            system: "PlayStation 3".to_string(),
            core: "rpcs3".to_string(),
            supported_formats: vec!["pkg", "iso", "rap", "edat", "sdat"].into_iter().map(String::from).collect(),
            emulator_type: EmulatorType::NativeService,
            service_port: Some(8085),
            github_url: "https://github.com/RPCS3/rpcs3".to_string(),
            license: "GPLv2".to_string(),
        },
        
        // PlayStation Portable
        EmulatorInfo {
            id: "ppsspp".to_string(),
            name: "PPSSPP".to_string(),
            system: "PlayStation Portable".to_string(),
            core: "ppsspp_libretro".to_string(),
            supported_formats: vec!["iso", "cso", "pbp", "elf"].into_iter().map(String::from).collect(),
            emulator_type: EmulatorType::NativeService,
            service_port: Some(8086),
            github_url: "https://github.com/hrydgard/ppsspp".to_string(),
            license: "GPLv2+".to_string(),
        },
        
        // PlayStation Vita
        EmulatorInfo {
            id: "vita3k".to_string(),
            name: "Vita3K".to_string(),
            system: "PlayStation Vita".to_string(),
            core: "vita3k".to_string(),
            supported_formats: vec!["vpk", "pkg"].into_iter().map(String::from).collect(),
            emulator_type: EmulatorType::NativeService,
            service_port: Some(8091),
            github_url: "https://github.com/Vita3K/Vita3K".to_string(),
            license: "GPLv2".to_string(),
        },
        
        // ========== SEGA SYSTEMS ==========
        
        // Dreamcast
        EmulatorInfo {
            id: "flycast".to_string(),
            name: "Flycast".to_string(),
            system: "Sega Dreamcast / Naomi / Atomiswave".to_string(),
            core: "flycast_libretro".to_string(),
            supported_formats: vec!["cdi", "gdi", "chd", "elf"].into_iter().map(String::from).collect(),
            emulator_type: EmulatorType::RetroArchCore,
            service_port: Some(8081),
            github_url: "https://github.com/flyinghead/flycast".to_string(),
            license: "GPLv2".to_string(),
        },
        EmulatorInfo {
            id: "reicast".to_string(),
            name: "Reicast".to_string(),
            system: "Sega Dreamcast".to_string(),
            core: "reicast_libretro".to_string(),
            supported_formats: vec!["cdi", "gdi"].into_iter().map(String::from).collect(),
            emulator_type: EmulatorType::RetroArchCore,
            service_port: Some(8081),
            github_url: "https://github.com/skmp/reicast-emulator".to_string(),
            license: "GPLv2".to_string(),
        },
        
        // ========== MULTI-SYSTEM ==========
        
        EmulatorInfo {
            id: "bizhawk".to_string(),
            name: "BizHawk".to_string(),
            system: "Multi-System (TAS Tool)".to_string(),
            core: "bizhawk".to_string(),
            supported_formats: vec!["nes", "snes", "n64", "gb", "gbc", "gba", "psx"].into_iter().map(String::from).collect(),
            emulator_type: EmulatorType::NativeService,
            service_port: Some(8092),
            github_url: "https://github.com/TASEmulators/BizHawk".to_string(),
            license: "GPLv2".to_string(),
        },
        
        EmulatorInfo {
            id: "higan-multi".to_string(),
            name: "higan (Multi-System)".to_string(),
            system: "Multi-System (Accuracy)".to_string(),
            core: "higan".to_string(),
            supported_formats: vec!["nes", "snes", "gb", "gbc", "gba"].into_iter().map(String::from).collect(),
            emulator_type: EmulatorType::RetroArchCore,
            service_port: Some(8081),
            github_url: "https://github.com/higan-emu/higan".to_string(),
            license: "GPLv3".to_string(),
        },
    ]
}

pub fn get_emulator_by_id(id: &str) -> Option<EmulatorInfo> {
    get_all_emulators().into_iter().find(|e| e.id == id)
}

pub fn get_emulators_by_system(system: &str) -> Vec<EmulatorInfo> {
    get_all_emulators()
        .into_iter()
        .filter(|e| e.system.to_lowercase().contains(&system.to_lowercase()))
        .collect()
}

