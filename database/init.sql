-- Users table
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Emulators registry
CREATE TABLE IF NOT EXISTS emulators (
    id VARCHAR(50) PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    system VARCHAR(100) NOT NULL,
    core VARCHAR(100) NOT NULL,
    emulator_type VARCHAR(50) NOT NULL,
    service_port INTEGER,
    github_url TEXT,
    license VARCHAR(50),
    supported_formats TEXT[]
);

-- Games table
CREATE TABLE IF NOT EXISTS games (
    id SERIAL PRIMARY KEY,
    title VARCHAR(200) NOT NULL,
    system VARCHAR(100) NOT NULL,
    file_path VARCHAR(500) NOT NULL UNIQUE,
    emulator_id VARCHAR(50) NOT NULL REFERENCES emulators(id),
    emulator_type VARCHAR(50) NOT NULL,
    added_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    user_id INTEGER REFERENCES users(id),
    file_size BIGINT,
    metadata JSONB
);

-- Save states
CREATE TABLE IF NOT EXISTS save_states (
    id SERIAL PRIMARY KEY,
    game_id INTEGER REFERENCES games(id) ON DELETE CASCADE,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    save_data BYTEA,
    slot INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(game_id, user_id, slot)
);

-- Game play history
CREATE TABLE IF NOT EXISTS play_history (
    id SERIAL PRIMARY KEY,
    game_id INTEGER REFERENCES games(id) ON DELETE CASCADE,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    play_time_seconds INTEGER DEFAULT 0,
    last_played TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Active sessions table for tracking connected users
CREATE TABLE IF NOT EXISTS active_sessions (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    ip_address VARCHAR(45) NOT NULL,
    last_seen TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(username, ip_address)
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_games_system ON games(system);
CREATE INDEX IF NOT EXISTS idx_games_user ON games(user_id);
CREATE INDEX IF NOT EXISTS idx_games_emulator ON games(emulator_id);
CREATE INDEX IF NOT EXISTS idx_save_states_game_user ON save_states(game_id, user_id);
CREATE INDEX IF NOT EXISTS idx_play_history_user ON play_history(user_id);
CREATE INDEX IF NOT EXISTS idx_active_sessions_last_seen ON active_sessions(last_seen);

-- Insert emulator registry data
INSERT INTO emulators (id, name, system, core, emulator_type, service_port, github_url, license, supported_formats) VALUES
('fceux', 'FCEUX', 'Nintendo Entertainment System', 'fceumm_libretro', 'RetroArchCore', 8081, 'https://github.com/TASEmulators/fceux', 'GPLv2', ARRAY['nes', 'fds']),
('nestopia', 'Nestopia UE', 'Nintendo Entertainment System', 'nestopia_libretro', 'RetroArchCore', 8081, 'https://github.com/libretro/nestopia', 'GPLv2', ARRAY['nes', 'fds']),
('snes9x', 'Snes9x', 'Super Nintendo Entertainment System', 'snes9x_libretro', 'RetroArchCore', 8081, 'https://github.com/snes9xgit/snes9x', 'GPLv2+', ARRAY['smc', 'sfc']),
('mupen64plus', 'Mupen64Plus', 'Nintendo 64', 'mupen64plus_next_libretro', 'RetroArchCore', 8081, 'https://github.com/mupen64plus', 'GPLv2', ARRAY['n64', 'z64']),
('dolphin', 'Dolphin', 'GameCube / Wii', 'dolphin_libretro', 'NativeService', 8083, 'https://github.com/dolphin-emu/dolphin', 'GPLv2+', ARRAY['iso', 'gcm', 'wbfs']),
('sameboy', 'SameBoy', 'Game Boy / Game Boy Color', 'sameboy_libretro', 'RetroArchCore', 8081, 'https://github.com/LIJI32/SameBoy', 'MIT', ARRAY['gb', 'gbc']),
('mgba', 'mGBA', 'Game Boy Advance', 'mgba_libretro', 'RetroArchCore', 8081, 'https://github.com/mgba-emu/mgba', 'MPLv2', ARRAY['gba', 'gb', 'gbc']),
('desmume', 'DeSmuME', 'Nintendo DS', 'desmume_libretro', 'RetroArchCore', 8081, 'https://github.com/TASEmulators/desmume', 'GPLv2', ARRAY['nds']),
('melonds', 'melonDS', 'Nintendo DS / DSi', 'melonds_libretro', 'RetroArchCore', 8081, 'https://github.com/melonDS-emu/melonDS', 'GPLv3', ARRAY['nds', 'dsi']),
('citra', 'Citra', 'Nintendo 3DS', 'citra_libretro', 'NativeService', 8087, 'https://citra-emulator.com', 'GPLv2', ARRAY['3ds', 'cci']),
('yuzu', 'yuzu', 'Nintendo Switch', 'yuzu', 'NativeService', 8088, 'https://github.com/yuzu-emu/yuzu', 'GPLv2', ARRAY['nsp', 'xci']),
('ryujinx', 'Ryujinx', 'Nintendo Switch', 'ryujinx', 'NativeService', 8089, 'https://github.com/Ryujinx/Ryujinx', 'MIT', ARRAY['nsp', 'xci']),
('duckstation', 'DuckStation', 'PlayStation 1', 'duckstation_libretro', 'RetroArchCore', 8081, 'https://github.com/stenzek/duckstation', 'GPLv3', ARRAY['cue', 'bin', 'iso', 'chd']),
('pcsx2', 'PCSX2', 'PlayStation 2', 'pcsx2', 'NativeService', 8084, 'https://github.com/PCSX2/pcsx2', 'GPLv3', ARRAY['iso', 'bin']),
('rpcs3', 'RPCS3', 'PlayStation 3', 'rpcs3', 'NativeService', 8085, 'https://github.com/RPCS3/rpcs3', 'GPLv2', ARRAY['pkg', 'iso']),
('ppsspp', 'PPSSPP', 'PlayStation Portable', 'ppsspp_libretro', 'NativeService', 8086, 'https://github.com/hrydgard/ppsspp', 'GPLv2+', ARRAY['iso', 'cso']),
('vita3k', 'Vita3K', 'PlayStation Vita', 'vita3k', 'NativeService', 8091, 'https://github.com/Vita3K/Vita3K', 'GPLv2', ARRAY['vpk', 'pkg']),
('flycast', 'Flycast', 'Sega Dreamcast / Naomi / Atomiswave', 'flycast_libretro', 'RetroArchCore', 8081, 'https://github.com/flyinghead/flycast', 'GPLv2', ARRAY['cdi', 'gdi', 'chd']),
('bizhawk', 'BizHawk', 'Multi-System (TAS Tool)', 'bizhawk', 'NativeService', 8092, 'https://github.com/TASEmulators/BizHawk', 'GPLv2', ARRAY['nes', 'snes', 'n64', 'gb', 'gba'])
ON CONFLICT (id) DO NOTHING;

