# ROM Management System - Complete Guide

## 🎮 Overview

The ROM management system has been completely redesigned with organized directories, automatic detection, and a modern upload interface.

## 📁 Directory Structure

All ROMs are now organized in the `roms/` directory with dedicated subdirectories for each console:

### Nintendo Systems
- **nes/** - Nintendo Entertainment System
- **snes/** - Super Nintendo Entertainment System
- **n64/** - Nintendo 64
- **gamecube/** - Nintendo GameCube
- **wii/** - Nintendo Wii
- **gb/** - Game Boy
- **gbc/** - Game Boy Color
- **gba/** - Game Boy Advance
- **nds/** - Nintendo DS
- **3ds/** - Nintendo 3DS
- **switch/** - Nintendo Switch

### Sony Systems
- **ps1/** - PlayStation 1
- **ps2/** - PlayStation 2
- **ps3/** - PlayStation 3
- **psp/** - PlayStation Portable
- **psvita/** - PlayStation Vita

### Sega Systems
- **dreamcast/** - Sega Dreamcast
- **sega-genesis/** - Sega Genesis/Mega Drive
- **sega-cd/** - Sega CD/Mega CD
- **sega-saturn/** - Sega Saturn

### Arcade & Other
- **arcade/** - General arcade ROMs
- **mame/** - MAME-specific ROMs
- **neogeo/** - Neo Geo
- **atari2600/** - Atari 2600
- **atari7800/** - Atari 7800
- **turbografx16/** - TurboGrafx-16/PC Engine
- **turbografx-cd/** - TurboGrafx-CD
- **wonderswan/** - WonderSwan/WonderSwan Color

Each directory contains a detailed README.md with:
- Console information
- Supported file formats
- Compatible emulators
- Usage instructions
- Console-specific notes

## 🚀 Features

### 1. Automatic ROM Detection
- **On Startup**: Backend automatically scans ROM directories on server start
- **Manual Scan**: Trigger scans via the web interface
- **Smart Detection**: Automatically matches ROMs to compatible emulators
- **Duplicate Prevention**: Won't add the same ROM twice

### 2. Web-Based Upload
- **Drag & Drop**: Intuitive file upload interface
- **Progress Tracking**: Real-time upload progress for each file
- **Batch Upload**: Upload multiple ROMs at once
- **Auto-Organization**: Files automatically placed in correct directories
- **Console Selection**: Manual override or auto-detect from file extension

### 3. Intelligent Organization
- **Extension Mapping**: Automatically determines console from file extension
- **Emulator Matching**: Finds the best emulator for each ROM
- **Clean Naming**: Removes common tags like `(USA)`, `[!]`, etc.
- **Metadata Storage**: Tracks file size, system, and emulator info

## 📤 How to Add ROMs

### Method 1: Manual File Placement
1. Place ROM files directly in the appropriate console directory
2. The system will auto-detect them on the next scan
3. Click "Scan Directory" in the upload interface to trigger detection

### Method 2: Web Upload
1. Click the "📤 Upload ROMs" button in the game library
2. Choose between "Upload ROMs" and "Scan Directory" tabs
3. Drag and drop files or click "Browse Files"
4. Optionally select a target console (or leave on auto-detect)
5. Click "Upload All" or upload files individually
6. ROMs are automatically added to your library

## 🔍 Auto-Detection Process

When files are added, the system:
1. **Scans** all console directories recursively
2. **Identifies** file extensions
3. **Maps** extensions to console systems
4. **Finds** compatible emulators
5. **Adds** to database with metadata
6. **Skips** duplicates automatically

## 📊 Backend Architecture

### New Modules

**rom_scanner.rs**
- Scans directory structure
- Maps extensions to consoles
- Matches ROMs with emulators
- Cleans filenames for display

**roms.rs**
- Upload API endpoint (`POST /api/roms/upload`)
- Scan API endpoint (`POST /api/roms/scan`)
- Console info endpoint (`GET /api/roms/consoles`)
- Multipart form handling

### Updated Components

**main.rs**
- Performs initial scan on startup
- Registers new API routes
- Handles ROM path configuration

**docker-compose.yml**
- Changed ROM volume from read-only to read-write
- Enables file uploads from web interface

**database/init.sql**
- Added UNIQUE constraint to file_path
- Prevents duplicate ROM entries

## 🎨 Frontend Components

### RomUpload Component
- Modern drag-and-drop interface
- Upload queue management
- Progress tracking
- Two-tab interface (Upload / Scan)
- Console selector dropdown
- Real-time status updates

### GameLibrary Integration
- New "Upload ROMs" button
- Slide-down upload modal
- Automatic refresh after upload
- Seamless integration with existing UI

## 🔧 API Endpoints

### Upload ROM
```
POST /api/roms/upload
Content-Type: multipart/form-data

Fields:
- file: ROM file (required)
- console: Target console directory (optional)
- title: Custom game title (optional)

Response:
{
  "success": true,
  "message": "Successfully uploaded filename.rom",
  "game_id": 123,
  "file_path": "console/filename.rom"
}
```

### Scan ROMs
```
POST /api/roms/scan

Response:
{
  "total_found": 50,
  "newly_added": 10,
  "already_exists": 40,
  "errors": []
}
```

### Get Consoles
```
GET /api/roms/consoles

Response:
[
  {
    "id": "nes",
    "name": "Nintendo Entertainment System",
    "supported_formats": ["nes", "fds", "unf", "unif"]
  },
  ...
]
```

## 🎯 File Format Quick Reference

| Console | Extensions |
|---------|-----------|
| NES | .nes, .fds, .unf, .unif |
| SNES | .smc, .sfc, .fig, .swc, .bs |
| N64 | .n64, .z64, .v64, .u64 |
| GameCube | .iso, .gcm, .wbfs, .ciso, .gcz |
| PS1 | .cue, .bin, .iso, .chd, .pbp |
| PS2 | .iso, .bin, .mdf, .nrg |
| GBA | .gba |
| DS | .nds, .dsi |
| Dreamcast | .cdi, .gdi, .chd |
| Arcade/MAME | .zip (don't unzip!) |

*See individual console README files for complete format lists*

## ⚖️ Legal Notice

**IMPORTANT**: Only use ROM files that you legally own. This typically means:
- You own the original game cartridge or disc
- You created the ROM backup yourself
- Or you purchased the ROM through a legitimate digital storefront

Downloading ROMs you don't own is illegal in most jurisdictions. This software is provided for backing up and playing games you legally own.

## 🆘 Troubleshooting

### ROMs Not Showing Up
1. Check file is in correct console directory
2. Verify file extension is supported
3. Click "Scan Directory" in upload interface
4. Check backend logs for errors

### Upload Fails
1. Ensure file extension is recognized
2. Check file isn't corrupted
3. Verify disk space available
4. Check backend is running

### Wrong Emulator Assigned
The system picks the best available emulator. You can manually change this in the database if needed.

## 🔄 Updating the System

When you add new ROMs manually:
1. Place files in appropriate directories
2. Open web interface
3. Click "Upload ROMs"
4. Switch to "Scan Directory" tab
5. Click "Start Scan"
6. Review results

The system will report:
- Total files found
- Newly added games
- Already existing games
- Any errors encountered

## 📝 Notes

- **MAME/Arcade ROMs**: Keep zipped! Don't extract them.
- **Multi-disc games**: Use .m3u playlists for PS1, etc.
- **.cue/.bin pairs**: Both files must be present
- **BIOS files**: Some systems require BIOS files (not included)
- **Compressed formats**: CHD recommended for disc-based systems

## 🎉 Benefits of This System

✅ **Organized** - Clear directory structure
✅ **Automatic** - No manual database entries needed
✅ **User-Friendly** - Upload via web interface
✅ **Intelligent** - Auto-detects console and emulator
✅ **Safe** - Prevents duplicates
✅ **Documented** - README in every directory
✅ **Scalable** - Easy to add more ROMs
✅ **Professional** - Production-ready design

Enjoy your organized ROM library! 🎮

