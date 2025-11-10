# ROM Organization Guide

This directory contains organized subdirectories for all supported gaming console ROMs.

## Directory Structure

### Nintendo Systems
- `nes/` - Nintendo Entertainment System (NES)
- `snes/` - Super Nintendo Entertainment System (SNES)
- `n64/` - Nintendo 64
- `gamecube/` - Nintendo GameCube
- `wii/` - Nintendo Wii
- `gb/` - Game Boy
- `gbc/` - Game Boy Color
- `gba/` - Game Boy Advance
- `nds/` - Nintendo DS
- `3ds/` - Nintendo 3DS
- `switch/` - Nintendo Switch

### Sony Systems
- `ps1/` - PlayStation 1
- `ps2/` - PlayStation 2
- `ps3/` - PlayStation 3
- `psp/` - PlayStation Portable
- `psvita/` - PlayStation Vita

### Sega Systems
- `dreamcast/` - Sega Dreamcast
- `sega-genesis/` - Sega Genesis / Mega Drive
- `sega-cd/` - Sega CD / Mega CD
- `sega-saturn/` - Sega Saturn

### Arcade & Other Systems
- `arcade/` - Arcade ROMs (MAME)
- `mame/` - MAME ROMs
- `neogeo/` - Neo Geo
- `atari2600/` - Atari 2600
- `atari7800/` - Atari 7800
- `turbografx16/` - TurboGrafx-16 / PC Engine
- `turbografx-cd/` - TurboGrafx-CD
- `wonderswan/` - WonderSwan / WonderSwan Color

## Usage

### Method 1: Manual Organization
Simply place your ROM files in the appropriate directory based on the console. The system will automatically detect and organize them.

Example:
```
roms/
├── nes/
│   ├── Super_Mario_Bros.nes
│   └── Legend_of_Zelda.nes
├── snes/
│   ├── Super_Mario_World.smc
│   └── Chrono_Trigger.smc
└── ps1/
    ├── Final_Fantasy_VII.bin
    └── Final_Fantasy_VII.cue
```

### Method 2: Web Upload
Use the web interface to upload ROMs directly. The system will:
1. Detect the ROM format
2. Match it with compatible emulators
3. Place it in the correct directory
4. Add it to your game library automatically

## Supported File Formats

The system supports various file formats for each console. Common formats include:
- **NES**: .nes, .fds, .unf, .unif
- **SNES**: .smc, .sfc, .fig, .swc, .bs
- **N64**: .n64, .z64, .v64, .u64
- **PS1**: .cue, .bin, .iso, .chd, .m3u, .pbp
- **GBA**: .gba
- **And many more...**

## Auto-Detection

The backend service automatically scans these directories on startup and periodically checks for new ROMs. You can also trigger a manual scan from the web interface.

## Legal Notice

Only use ROM files that you legally own. This means you should own the original game cartridge or disc. Downloading ROMs of games you don't own is illegal in most jurisdictions.

