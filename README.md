# 🎮 Maplex Games - Universal Emulator Platform

<div align="center">

![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-blue)
![Docker](https://img.shields.io/badge/Docker-Required-2496ED?logo=docker)
![License](https://img.shields.io/badge/License-Multiple-green)
![Status](https://img.shields.io/badge/Status-Active-success)

**Play classic games from 15+ gaming systems in one unified platform**

[Quick Start](#-quick-start-5-minutes) • [Supported Systems](#-supported-gaming-systems) • [FAQ](#-frequently-asked-questions) • [Glossary](#-glossary)

</div>

---

## 📑 Table of Contents

1. [What is This?](#-what-is-this)
2. [Quick Start (5 Minutes)](#-quick-start-5-minutes)
3. [Detailed Installation Guide](#-detailed-installation-guide)
4. [Supported Gaming Systems](#-supported-gaming-systems)
5. [Adding Your Games](#-adding-your-games)
6. [Using the Platform](#-using-the-platform)
7. [Troubleshooting](#-troubleshooting)
8. [Frequently Asked Questions](#-frequently-asked-questions)
9. [For Developers](#-for-developers)
10. [Glossary](#-glossary)
11. [Appendix](#-appendix)
12. [License Information](#-license-information)

---

## 🎯 What is This?

**Maplex Games** is an all-in-one gaming platform that lets you play classic video games from multiple gaming systems through your web browser. Think of it as Netflix for retro gaming—one place to organize and play all your favorite childhood games.

### 🌟 Key Features

- 🖥️ **Web-Based Interface** - Play directly in your browser, no complex setup
- 🎮 **15+ Gaming Systems** - From NES to PlayStation 3, GameCube to Nintendo Switch
- 📚 **Game Library Management** - Organize and search your entire game collection
- 💾 **Save States** - Save your progress anytime and resume later
- 🎨 **Modern UI** - Beautiful, easy-to-use interface
- 🔒 **Private & Secure** - All games stay on your own computer
- 🐳 **Easy Setup** - One command to get everything running

### 🚀 Perfect For

- ✅ Retro gaming enthusiasts
- ✅ Game collectors who want to organize their ROMs
- ✅ People who want to play classic games on modern computers
- ✅ Anyone looking for a Netflix-like experience for classic games

---

## ⚡ Quick Start (5 Minutes)

### Prerequisites Checklist

Before you begin, make sure you have:

- [ ] A computer (Windows, macOS, or Linux)
- [ ] [Docker Desktop](https://www.docker.com/products/docker-desktop) installed
- [ ] At least 5GB of free disk space
- [ ] Game ROM files (you must own the original games legally)

### 🏁 Getting Started

**Step 1:** Download or clone this repository

```bash
git clone https://github.com/SelmanDemiray/maplex_games.git
cd maplex_games
```

**Step 2:** Add your game files to the `roms` folder

```bash
# Place your ROM files in the roms directory
# Example: roms/Super_Mario_World.snes
```

**Step 3:** Start the platform

```bash
docker-compose up -d
```

**Step 4:** Open your browser and visit:

```
http://localhost:41968
```

🎉 **That's it!** You should see your game library.

---

## 📖 Detailed Installation Guide

### For Windows Users

#### 1. Install Docker Desktop

1. Download Docker Desktop from [docker.com/products/docker-desktop](https://www.docker.com/products/docker-desktop)
2. Run the installer (requires administrator privileges)
3. Restart your computer when prompted
4. Open Docker Desktop and wait for it to start (green light in system tray)

#### 2. Download Maplex Games

**Option A: Using Git**
```powershell
# Open PowerShell
git clone https://github.com/SelmanDemiray/maplex_games.git
cd maplex_games
```

**Option B: Manual Download**
1. Go to [github.com/SelmanDemiray/maplex_games](https://github.com/SelmanDemiray/maplex_games)
2. Click the green "Code" button
3. Click "Download ZIP"
4. Extract the ZIP file to a folder (e.g., `C:\maplex_games`)
5. Open PowerShell and navigate to that folder

#### 3. Start the Platform

```powershell
docker-compose up -d
```

Wait 2-3 minutes for everything to start. You'll see messages about downloading and building containers.

#### 4. Access Your Platform

Open any web browser and go to: **http://localhost:41968**

### For macOS Users

#### 1. Install Docker Desktop

1. Download Docker Desktop for Mac from [docker.com/products/docker-desktop](https://www.docker.com/products/docker-desktop)
2. Open the `.dmg` file and drag Docker to Applications
3. Open Docker from Applications folder
4. Grant permissions when asked
5. Wait for Docker to start (whale icon in menu bar)

#### 2. Download Maplex Games

Open Terminal and run:

```bash
git clone https://github.com/SelmanDemiray/maplex_games.git
cd maplex_games
```

#### 3. Start the Platform

```bash
docker-compose up -d
```

#### 4. Access Your Platform

Open Safari/Chrome and visit: **http://localhost:41968**

### For Linux Users

#### 1. Install Docker

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install docker.io docker-compose
sudo systemctl start docker
sudo systemctl enable docker
sudo usermod -aG docker $USER
```

**Fedora:**
```bash
sudo dnf install docker docker-compose
sudo systemctl start docker
sudo systemctl enable docker
sudo usermod -aG docker $USER
```

Log out and back in for group changes to take effect.

#### 2. Download and Start

```bash
git clone https://github.com/SelmanDemiray/maplex_games.git
cd maplex_games
docker-compose up -d
```

#### 3. Access Your Platform

Visit: **http://localhost:41968**

---

## 🎮 Supported Gaming Systems

### Nintendo Systems

| System | Emulator Used | File Formats | Example Games |
|--------|--------------|--------------|---------------|
| **NES** (Nintendo Entertainment System) | FCEUX | `.nes`, `.fds` | Super Mario Bros., The Legend of Zelda |
| **SNES** (Super Nintendo) | Snes9x | `.smc`, `.sfc` | Super Mario World, Chrono Trigger |
| **N64** (Nintendo 64) | Mupen64Plus | `.n64`, `.z64` | Super Mario 64, Zelda: Ocarina of Time |
| **GameCube** | Dolphin | `.iso`, `.gcm` | Super Smash Bros. Melee |
| **Wii** | Dolphin | `.iso`, `.wbfs` | Super Mario Galaxy |
| **Game Boy** | SameBoy | `.gb` | Pokémon Red/Blue, Tetris |
| **Game Boy Color** | SameBoy | `.gbc` | Pokémon Crystal, Zelda: Oracle series |
| **Game Boy Advance** | mGBA | `.gba` | Pokémon Emerald, Fire Emblem |
| **Nintendo DS** | DeSmuME | `.nds` | Pokémon Diamond, Animal Crossing |
| **Nintendo 3DS** | Citra | `.3ds`, `.cia` | Pokémon X/Y, Fire Emblem Awakening |
| **Nintendo Switch** | Yuzu/Ryujinx | `.nsp`, `.xci` | Breath of the Wild, Mario Odyssey |

### Sony Systems

| System | Emulator Used | File Formats | Example Games |
|--------|--------------|--------------|---------------|
| **PlayStation 1** | DuckStation | `.bin`, `.cue`, `.chd` | Final Fantasy VII, Crash Bandicoot |
| **PlayStation 2** | PCSX2 | `.iso` | Kingdom Hearts, God of War |
| **PlayStation 3** | RPCS3 | `.pkg`, folder | The Last of Us, Uncharted |
| **PSP** (PlayStation Portable) | PPSSPP | `.iso`, `.cso` | God of War, Monster Hunter |
| **PS Vita** | Vita3K | `.vpk` | Persona 4 Golden, Gravity Rush |

### Sega Systems

| System | Emulator Used | File Formats | Example Games |
|--------|--------------|--------------|---------------|
| **Dreamcast** | Flycast | `.gdi`, `.cdi`, `.chd` | Sonic Adventure, Shenmue |

### Multi-System Tools

| Tool | Purpose | Systems Supported |
|------|---------|-------------------|
| **RetroArch** | All-in-one emulator frontend | 50+ systems |
| **BizHawk** | Tool-assisted speedrun (TAS) tool | NES, SNES, GB, GBA, N64, PS1 |

---

## 📂 Adding Your Games

### Step-by-Step Guide

#### 1. Locate Your ROM Files

ROM files are digital copies of game cartridges/discs. You must legally own the original game to use ROMs.

**Common file extensions:**
- `.nes`, `.smc`, `.n64` - Nintendo cartridge games
- `.iso`, `.bin`, `.cue` - Disc-based games
- `.gba`, `.nds` - Handheld cartridge games

#### 2. Organize Your Files

Create subfolders in the `roms` directory for better organization:

```
roms/
├── nes/
│   ├── Super_Mario_Bros.nes
│   └── Legend_of_Zelda.nes
├── snes/
│   ├── Super_Mario_World.smc
│   └── Chrono_Trigger.smc
├── ps1/
│   ├── Final_Fantasy_VII.bin
│   └── Final_Fantasy_VII.cue
└── gba/
    ├── Pokemon_Emerald.gba
    └── Fire_Emblem.gba
```

#### 3. Copy Files to the ROMs Directory

**Windows:**
```powershell
# Navigate to your maplex_games folder
cd C:\path\to\maplex_games

# Copy your ROMs (example)
copy "C:\Users\YourName\Downloads\*.nes" roms\nes\
```

**macOS/Linux:**
```bash
# Navigate to your maplex_games folder
cd ~/maplex_games

# Copy your ROMs (example)
cp ~/Downloads/*.nes roms/nes/
```

#### 4. Restart the Platform (if running)

```bash
docker-compose restart backend
```

#### 5. Refresh Your Browser

Your new games should appear in the library automatically!

### 🎨 Game Art & Metadata

The platform can automatically fetch game artwork and information. Just make sure your ROM files have recognizable names (e.g., "Super Mario World.smc" instead of "game1.smc").

---

## 🖱️ Using the Platform

### Main Interface

When you open **http://localhost:41968**, you'll see:

1. **Game Library** - Grid view of all your games with cover art
2. **Search Bar** - Quickly find specific games
3. **Filters** - Filter by system (NES, SNES, etc.) or emulator
4. **Game Cards** - Click any game to launch it

### Launching a Game

1. Click on any game card
2. The emulator will load automatically
3. The game will start playing in your browser

### Controls

Each emulator has different default controls. Generally:

**Keyboard Controls:**
- Arrow Keys = D-Pad/Movement
- Z/X = A/B Buttons
- A/S = X/Y Buttons
- Enter = Start
- Shift = Select
- Esc = Exit/Menu

**Gamepad Support:**
Most USB/Bluetooth controllers work automatically. Connect your controller before starting a game.

### Saving Your Progress

**Save States** (Quick Save):
- Press F5 to quick save
- Press F7 to quick load
- You can save at any point in the game

**In-Game Saves:**
- Use the game's built-in save feature (like in Pokémon)
- These saves persist between sessions

---

## 🔧 Troubleshooting

### Platform Won't Start

**Problem:** `docker-compose up -d` fails

**Solutions:**
1. Make sure Docker Desktop is running (look for the whale icon)
2. Try: `docker-compose down` then `docker-compose up -d`
3. Check Docker Desktop settings → Resources → ensure enough RAM allocated (4GB minimum)

### Can't Access http://localhost:41968

**Problem:** Browser shows "Can't connect" or "Connection refused"

**Solutions:**
1. Wait 2-3 minutes after starting - containers take time to initialize
2. Check if services are running: `docker-compose ps`
3. Check the logs: `docker-compose logs frontend`
4. Make sure port 41968 isn't being used by another application

### Games Don't Appear in Library

**Problem:** Added ROMs but they don't show up

**Solutions:**
1. Make sure ROM files are in the `roms` directory
2. Check file extensions match supported formats (see [Supported Systems](#-supported-gaming-systems))
3. Restart the backend: `docker-compose restart backend`
4. Check backend logs: `docker-compose logs backend`

### Emulator Won't Load Game

**Problem:** Click game but nothing happens or error appears

**Solutions:**
1. Verify ROM file isn't corrupted - try opening with a standalone emulator first
2. Check browser console for errors (F12 → Console tab)
3. Some systems require BIOS files (see [BIOS Files](#bios-files-required-for-some-systems))

### Slow Performance / Laggy Games

**Problem:** Games run slow or choppy

**Solutions:**
1. Close other applications to free up RAM
2. In Docker Desktop → Settings → Resources, increase CPU and RAM allocation
3. Some systems (PS3, Switch) require powerful hardware - check [System Requirements](#system-requirements)
4. Try using a lighter browser (Edge/Chrome instead of Firefox)

### Port Already in Use Error

**Problem:** Error says port 41968, 37291, or 28473 is already in use

**Solutions:**
1. Find what's using the port: 
   - Windows: `netstat -ano | findstr :41968`
   - macOS/Linux: `lsof -i :41968`
2. Stop that application or change ports in `docker-compose.yml`

---

## ❓ Frequently Asked Questions

### General Questions

**Q: Is this legal?**

A: The platform itself is legal. However, you must own the original game to legally use ROM files. Downloading copyrighted games you don't own is illegal in most countries.

**Q: Do I need to own the original games?**

A: Yes, legally you should own a physical or digital copy of any game you emulate.

**Q: Can I play online multiplayer?**

A: Not currently. This platform is designed for single-player local gaming.

**Q: Is my data private?**

A: Yes! Everything runs locally on your computer. No data is sent to external servers.

### Technical Questions

**Q: Can I run this without Docker?**

A: Yes, but it's much more complex. Docker handles all dependencies automatically. See [Manual Installation](#manual-installation-without-docker) for advanced users.

**Q: How much disk space do I need?**

A: Minimum 5GB. More if you have many games:
- Platform: ~3GB
- Each NES/SNES game: 1-4MB
- Each N64 game: 8-64MB
- Each PS1 game: 300-700MB
- Each PS2 game: 2-8GB
- Each PS3 game: 5-50GB

**Q: Can I access this from other devices on my network?**

A: Yes! Replace `localhost` with your computer's IP address. Example: `http://192.168.1.100:41968`

**Q: Can I share this with friends so they can access it from anywhere?**

A: Yes! Use Cloudflare Tunnel (100% free). See [CLOUDFLARE_SETUP.md](CLOUDFLARE_SETUP.md) for setup instructions.

**Quick start:**
```bash
# Option 1: Using Docker
docker-compose --profile cloudflare up cloudflared

# Option 2: Standalone (if cloudflared is installed)
cloudflared tunnel --url http://localhost:41968
```

This gives you a shareable URL like `https://yourname.trycloudflare.com` that works from anywhere!

**Q: Which systems require a powerful computer?**

A: 
- **Low requirements:** NES, SNES, GB, GBA, PS1, N64
- **Medium requirements:** GameCube, PS2, Wii, DS, 3DS, PSP
- **High requirements:** PS3, Wii U, Switch, PS Vita

**Q: Can I customize the appearance?**

A: Yes! Edit the CSS files in `frontend/src/` directory. See [Customization Guide](#customization-guide).

### Troubleshooting Questions

**Q: Why is Docker so large?**

A: Docker downloads base images and all dependencies. This ensures everything works consistently across different computers.

**Q: Can I add more than one ROM at a time?**

A: Yes! Just copy all your ROMs to the `roms` directory at once. The platform will scan and add them all.

**Q: What if a game doesn't work?**

A: Some games have issues with certain emulators. Try:
1. A different ROM file (different version/region)
2. Enabling a different emulator (edit `docker-compose.yml`)
3. Checking the [Compatibility Database](#compatibility-database)

---

## 💻 For Developers

### Architecture Overview

```
┌─────────────────────────────────────────────────┐
│                    Browser                       │
│              (React Frontend)                    │
│            Port: 41968                          │
└─────────────┬───────────────────────────────────┘
              │
              │ HTTP/REST API
              │
┌─────────────▼───────────────────────────────────┐
│              Rust Backend                        │
│              (Axum Framework)                    │
│            Port: 37291                          │
└─────────────┬───────────────────────────────────┘
              │
              │ SQL Queries
              │
┌─────────────▼───────────────────────────────────┐
│           PostgreSQL Database                    │
│            Port: 28473                          │
│  (Game Library, Users, Save States)             │
└──────────────────────────────────────────────────┘
```

### Technology Stack

**Frontend:**
- React 18.2.0
- TypeScript 4.9.0
- CSS3 (custom styling)

**Backend:**
- Rust (latest stable)
- Axum web framework
- SQLx for database access
- Tower for middleware

**Database:**
- PostgreSQL 15
- Stores game metadata, users, save states, play history

**Emulators:**
- RetroArch (libretro cores)
- Standalone emulators (Dolphin, PCSX2, etc.)

### Project Structure

```
maplex_games/
├── backend/               # Rust backend API
│   ├── src/
│   │   ├── main.rs       # Entry point
│   │   ├── games.rs      # Game management
│   │   └── emulators.rs  # Emulator registry
│   ├── Cargo.toml        # Rust dependencies
│   └── Dockerfile        # Backend container
├── frontend/              # React frontend
│   ├── src/
│   │   ├── App.tsx       # Main app component
│   │   ├── components/   # React components
│   │   └── services/     # API client
│   ├── package.json      # Node dependencies
│   └── Dockerfile        # Frontend container
├── database/              # PostgreSQL setup
│   ├── init.sql          # Database schema
│   └── Dockerfile        # Database container
├── emulators/             # Emulator configurations
│   ├── retroarch/
│   ├── dolphin/
│   ├── pcsx2/
│   └── ... (more emulators)
├── roms/                  # Your game files go here
├── docker-compose.yml     # Orchestrates all services
└── README.md              # This file
```

### API Endpoints

#### Games

```
GET    /api/games                  # List all games
GET    /api/games/:id              # Get specific game
POST   /api/games                  # Add new game
PUT    /api/games/:id              # Update game
DELETE /api/games/:id              # Remove game
```

#### Emulators

```
GET    /api/emulators              # List all emulators
GET    /api/emulators/:id          # Get emulator details
```

#### Query Parameters

```
GET /api/games?system=nes          # Filter by system
GET /api/games?emulator=fceux      # Filter by emulator
GET /api/games?search=mario        # Search by title
```

### Running in Development Mode

#### Backend (Rust)

```bash
cd backend
cargo run
# Runs on http://localhost:8080
```

#### Frontend (React)

```bash
cd frontend
npm install
npm start
# Runs on http://localhost:3000
```

#### Database

```bash
docker run -d \
  -e POSTGRES_PASSWORD=your_password \
  -e POSTGRES_DB=emulator_platform \
  -p 5432:5432 \
  postgres:15
```

### Adding a New Emulator

1. **Create emulator directory:**
   ```bash
   mkdir emulators/new-emulator
   ```

2. **Create Dockerfile:**
   ```dockerfile
   FROM ubuntu:22.04
   RUN apt-get update && apt-get install -y your-emulator
   # Add configuration
   EXPOSE 8093
   CMD ["your-emulator-command"]
   ```

3. **Add to docker-compose.yml:**
   ```yaml
   new-emulator:
     build:
       context: ./emulators/new-emulator
     volumes:
       - ./roms:/roms:ro
     ports:
       - "8093:8093"
   ```

4. **Register in backend** (`backend/src/emulators.rs`):
   ```rust
   EmulatorInfo {
       id: "new-emulator".to_string(),
       name: "New Emulator".to_string(),
       system: "System Name".to_string(),
       // ... more fields
   }
   ```

### Contributing

We welcome contributions! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Running Tests

```bash
# Backend tests
cd backend
cargo test

# Frontend tests
cd frontend
npm test
```

### Building for Production

```bash
# Build all containers
docker-compose build

# Start in production mode
docker-compose up -d

# View logs
docker-compose logs -f
```

---

## 📚 Glossary

### A

**API (Application Programming Interface)**
- A way for different software programs to communicate with each other. In this project, the frontend talks to the backend through an API.

**Axum**
- A web framework for the Rust programming language, used to build our backend server.

### B

**Backend**
- The "behind-the-scenes" part of the application that handles data storage, business logic, and talks to the database.

**BIOS (Basic Input/Output System)**
- System files required by some emulators to function correctly (e.g., PlayStation emulators need PS1 BIOS files).

### C

**Container**
- A packaged software unit that includes everything needed to run an application (like a lightweight virtual machine).

**CORS (Cross-Origin Resource Sharing)**
- A security feature that allows the frontend to communicate with the backend even though they're on different ports.

**Core**
- In RetroArch, a "core" is an emulator module for a specific system (e.g., Snes9x core for SNES games).

### D

**Docker**
- Software that packages applications and their dependencies into containers, making them easy to run anywhere.

**Docker Compose**
- A tool for defining and running multiple Docker containers together (like our frontend, backend, and database).

**Dockerfile**
- A text file with instructions on how to build a Docker container.

### E

**Emulator**
- Software that mimics the behavior of a gaming console, allowing you to play games designed for that console on your computer.

**Environment Variable**
- A configuration value stored outside the code (like passwords or settings) that can be changed without modifying the program.

### F

**Frontend**
- The user interface part of the application that you see and interact with in your web browser.

### G

**Git**
- A version control system that tracks changes to code over time.

**GitHub**
- A website that hosts Git repositories, allowing developers to share and collaborate on code.

### H

**HTTP (Hypertext Transfer Protocol)**
- The protocol used by web browsers to communicate with web servers.

### I

**ISO (Disc Image)**
- A file that contains an exact copy of a game disc (like PlayStation, GameCube, or PS2 discs).

### J

**JSON (JavaScript Object Notation)**
- A format for structuring data, commonly used in APIs to send information between frontend and backend.

**JWT (JSON Web Token)**
- A secure way to transmit information between parties, often used for user authentication.

### L

**libretro**
- A universal emulator API that allows different emulators to work with a common frontend (RetroArch).

**localhost**
- A term referring to "this computer." When you visit `http://localhost:41968`, you're accessing services on your own machine.

### M

**Middleware**
- Software that sits between different parts of an application, often handling things like authentication or logging.

### P

**Port**
- A virtual endpoint for network communications. Different services listen on different ports (e.g., 41968 for frontend, 37291 for backend).

**PostgreSQL**
- An open-source database system used to store game data, user information, and save states.

### R

**React**
- A popular JavaScript library for building user interfaces, used for our frontend.

**REST (Representational State Transfer)**
- An architectural style for APIs that uses standard HTTP methods (GET, POST, PUT, DELETE).

**RetroArch**
- A powerful, all-in-one emulator frontend that supports dozens of gaming systems through "cores."

**ROM (Read-Only Memory)**
- In gaming context, a digital copy of a game cartridge or disc. ROM files let you play games through emulators.

**Rust**
- A modern programming language known for speed and safety, used for our backend server.

### S

**Save State**
- A snapshot of a game at a specific moment that can be saved and loaded at any time, independent of the game's built-in save system.

**SQL (Structured Query Language)**
- A language used to interact with databases, for storing and retrieving data.

**SQLx**
- A Rust library that lets our backend communicate with the PostgreSQL database.

### T

**TAS (Tool-Assisted Speedrun)**
- Using special tools (like BizHawk) to create frame-perfect gameplay recordings.

**TypeScript**
- A programming language that adds type safety to JavaScript, used in our frontend code.

### W

**WASM (WebAssembly)**
- A technology that allows running high-performance code in web browsers, used by browser-based emulators.

---

## 📎 Appendix

### A. System Requirements

#### Minimum Requirements

- **OS:** Windows 10, macOS 10.15+, or Ubuntu 20.04+
- **CPU:** Dual-core processor (2.0 GHz)
- **RAM:** 4GB
- **Storage:** 10GB free space
- **Docker:** Docker Desktop 4.0 or later

#### Recommended Requirements

- **OS:** Windows 11, macOS 12+, or Ubuntu 22.04+
- **CPU:** Quad-core processor (3.0 GHz)
- **RAM:** 8GB or more
- **Storage:** 50GB+ free space (for game collection)
- **GPU:** Dedicated graphics card (for PS2, PS3, Wii U, Switch)
- **Docker:** Latest version of Docker Desktop

### B. BIOS Files (Required for Some Systems)

Some emulators require BIOS files to function:

| System | BIOS Required | Where to Place |
|--------|---------------|----------------|
| PlayStation 1 | ✅ Yes | `roms/bios/ps1/` |
| PlayStation 2 | ✅ Yes | `roms/bios/ps2/` |
| PlayStation 3 | ✅ Yes | `roms/bios/ps3/` |
| Nintendo DS | ✅ Yes | `roms/bios/nds/` |
| Nintendo 3DS | ✅ Yes | `roms/bios/3ds/` |
| GameCube | ❌ No | - |
| Wii | ❌ No | - |

**Note:** You must obtain BIOS files from consoles you own. We cannot provide these files.

### C. Port Reference

| Service | Internal Port | External Port | Purpose |
|---------|--------------|---------------|---------|
| Frontend | 3000 | 41968 | Web interface |
| Backend | 8080 | 37291 | API server |
| Database | 5432 | 28473 | PostgreSQL |
| RetroArch | 8081 | 8081 | Multi-system emulator |
| EmulatorJS | 80 | 8082 | Browser emulator |
| Dolphin | 8083 | 8083 | GameCube/Wii |
| PCSX2 | 8084 | 8084 | PlayStation 2 |
| RPCS3 | 8085 | 8085 | PlayStation 3 |
| PPSSPP | 8086 | 8086 | PSP |
| Citra | 8087 | 8087 | Nintendo 3DS |
| Yuzu | 8088 | 8088 | Nintendo Switch |
| Ryujinx | 8089 | 8089 | Nintendo Switch |
| Flycast | 8090 | 8090 | Dreamcast |
| Vita3K | 8091 | 8091 | PS Vita |
| BizHawk | 8092 | 8092 | Multi-system TAS |

### D. File Format Support

#### Nintendo Formats
```
.nes      - NES/Famicom
.sfc .smc - Super Nintendo
.n64 .z64 - Nintendo 64
.gb       - Game Boy
.gbc      - Game Boy Color
.gba      - Game Boy Advance
.nds      - Nintendo DS
.3ds .cia - Nintendo 3DS
.nsp .xci - Nintendo Switch
.iso      - GameCube/Wii discs
.wbfs     - Wii backup format
```

#### Sony Formats
```
.bin .cue - PlayStation 1 (must have both files)
.chd      - Compressed disc image
.iso      - PlayStation 2/PSP
.cso      - Compressed PSO
.pkg      - PlayStation 3 packages
.vpk      - PlayStation Vita
```

#### Sega Formats
```
.gdi      - Dreamcast (uncompressed)
.cdi      - Dreamcast (compressed)
.chd      - Compressed Dreamcast
```

### E. Keyboard Controls Reference

#### Standard Controls (Most Emulators)

| Action | Keyboard | Description |
|--------|----------|-------------|
| D-Pad Up | ↑ | Move up |
| D-Pad Down | ↓ | Move down |
| D-Pad Left | ← | Move left |
| D-Pad Right | → | Move right |
| A Button | Z | Primary action |
| B Button | X | Secondary action |
| X Button | A | Tertiary action |
| Y Button | S | Fourth action |
| Start | Enter | Start/Pause |
| Select | Shift | Select/Back |
| L Trigger | Q | Left shoulder |
| R Trigger | W | Right shoulder |
| Quick Save | F5 | Save state |
| Quick Load | F7 | Load state |
| Fast Forward | Tab | Speed up game |
| Rewind | Backspace | Rewind gameplay |
| Screenshot | F12 | Capture screen |
| Full Screen | F11 | Toggle fullscreen |
| Exit | Esc | Close emulator |

### F. Common Error Messages

| Error | Meaning | Solution |
|-------|---------|----------|
| "Cannot connect to Docker daemon" | Docker isn't running | Start Docker Desktop |
| "Port already in use" | Another program is using the same port | Stop other program or change port in docker-compose.yml |
| "Failed to fetch games" | Backend isn't responding | Check backend logs: `docker-compose logs backend` |
| "BIOS not found" | Required BIOS file missing | Add BIOS file to appropriate directory |
| "Unsupported file format" | ROM format not recognized | Convert ROM to supported format |
| "Out of memory" | Not enough RAM | Close other applications or increase Docker RAM limit |

### G. Docker Commands Cheat Sheet

```bash
# Start all services
docker-compose up -d

# Stop all services
docker-compose down

# View running containers
docker-compose ps

# View logs
docker-compose logs -f

# View specific service logs
docker-compose logs frontend
docker-compose logs backend

# Restart a service
docker-compose restart backend

# Rebuild a service
docker-compose build frontend

# Remove all containers and data
docker-compose down -v

# Update and restart
docker-compose pull
docker-compose up -d

# Enter a container shell
docker-compose exec backend sh
docker-compose exec frontend sh

# View resource usage
docker stats
```

### H. Environment Variables

You can customize these in a `.env` file:

```bash
# Database
DB_PASSWORD=your_secure_password

# Backend
JWT_SECRET=your_jwt_secret_key
ROMS_PATH=/roms
APP_PASSWORD=your_app_password_here

# Frontend
REACT_APP_API_URL=http://localhost:37291

# Optional: Change default ports
FRONTEND_PORT=41968
BACKEND_PORT=37291
DATABASE_PORT=28473
```

**Note:** The `APP_PASSWORD` is the password that all users must enter when logging into the application. Set this to a secure password that you'll share with authorized users.

### I. Compatibility Database

Systems with excellent compatibility (95%+ games work):
- ✅ NES/Famicom
- ✅ SNES/Super Famicom
- ✅ Game Boy / Game Boy Color
- ✅ Game Boy Advance
- ✅ Nintendo DS
- ✅ PlayStation 1
- ✅ Sega Genesis/Mega Drive

Systems with good compatibility (80-95% games work):
- ⚠️ Nintendo 64
- ⚠️ PlayStation 2
- ⚠️ GameCube
- ⚠️ Wii
- ⚠️ PSP
- ⚠️ Dreamcast

Systems with moderate compatibility (50-80% games work):
- ⚠️ Nintendo 3DS
- ⚠️ PlayStation 3
- ⚠️ PS Vita

Systems with early/experimental support:
- 🧪 Nintendo Switch (Requires powerful PC)
- 🧪 Wii U (Limited game support)

### J. Manual Installation (Without Docker)

For advanced users who don't want to use Docker:

#### Backend Setup

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install PostgreSQL
# Ubuntu: sudo apt install postgresql
# macOS: brew install postgresql

# Setup database
psql -U postgres -c "CREATE DATABASE emulator_platform;"
psql -U postgres -d emulator_platform -f database/init.sql

# Build and run backend
cd backend
export DATABASE_URL="postgres://postgres:password@localhost:5432/emulator_platform"
cargo run --release
```

#### Frontend Setup

```bash
# Install Node.js (v18+)
# Download from nodejs.org

# Install dependencies and run
cd frontend
npm install
REACT_APP_API_URL=http://localhost:8080 npm start
```

### K. Performance Optimization Tips

1. **Allocate More Resources to Docker**
   - Docker Desktop → Settings → Resources
   - Increase CPUs: 4+ cores
   - Increase Memory: 8GB+

2. **Enable Hardware Acceleration**
   - Docker Desktop → Settings → General
   - Enable "Use the WSL 2 based engine" (Windows)

3. **Use SSD for ROM Storage**
   - Store roms directory on an SSD for faster loading

4. **Close Background Applications**
   - Free up RAM and CPU for emulation

5. **Use Optimal Emulator Settings**
   - Disable unnecessary visual enhancements
   - Use "Fast" rendering mode for older systems

### L. Security Best Practices

1. **Change Default Passwords**
   - Set strong `DB_PASSWORD` in `.env`
   - Set strong `JWT_SECRET` in `.env`

2. **Don't Expose Publicly**
   - Only access from local network
   - Don't port forward to the internet

3. **Keep Docker Updated**
   - Update Docker Desktop regularly
   - Run `docker-compose pull` to update images

4. **Legal ROM Usage**
   - Only use ROMs of games you own
   - Don't share ROM files online

5. **Backup Your Data**
   - Regularly backup the `roms` directory
   - Export save states periodically

### M. Useful Resources & Links

**Official Documentation:**
- Docker: [docs.docker.com](https://docs.docker.com)
- Rust: [rust-lang.org](https://www.rust-lang.org)
- React: [react.dev](https://react.dev)
- PostgreSQL: [postgresql.org](https://www.postgresql.org)

**Emulator Documentation:**
- RetroArch: [docs.libretro.com](https://docs.libretro.com)
- Dolphin: [dolphin-emu.org/docs](https://dolphin-emu.org/docs)
- PCSX2: [pcsx2.net/docs](https://pcsx2.net/docs)
- RPCS3: [rpcs3.net/quickstart](https://rpcs3.net/quickstart)
- Yuzu: [yuzu-emu.org/wiki](https://yuzu-emu.org/wiki)

**Community:**
- r/emulation (Reddit)
- EmuGen Wiki (emulation.gametechwiki.com)

---

## 📜 License Information

### Project License

This project (Maplex Games platform) is open source and released under the MIT License.

### Third-Party Licenses

This platform integrates multiple emulators, each with their own licenses:

| Emulator | License | Link |
|----------|---------|------|
| RetroArch | GPLv3 | [libretro.com](https://www.libretro.com) |
| Dolphin | GPLv2+ | [dolphin-emu.org](https://dolphin-emu.org) |
| PCSX2 | GPLv3 | [pcsx2.net](https://pcsx2.net) |
| RPCS3 | GPLv2 | [rpcs3.net](https://rpcs3.net) |
| PPSSPP | GPLv2+ | [ppsspp.org](https://ppsspp.org) |
| Citra | GPLv2+ | [citra-emu.org](https://citra-emu.org) |
| Yuzu | GPLv2+ | [yuzu-emu.org](https://yuzu-emu.org) |
| Ryujinx | MIT | [ryujinx.org](https://ryujinx.org) |
| mGBA | MPLv2 | [mgba.io](https://mgba.io) |
| DeSmuME | GPLv2+ | [desmume.org](https://desmume.org) |

**Important Legal Notes:**

1. ✅ **Emulators are legal** - All emulators included are legal to use
2. ⚠️ **ROM legality** - You must own the original game to legally use ROM files
3. ❌ **BIOS distribution** - We cannot distribute BIOS files; you must obtain them from consoles you own
4. ✅ **Personal use** - This platform is intended for personal, non-commercial use
5. ❌ **No piracy** - Do not use this platform to play games you don't legally own

### Respecting Licenses

By using this platform, you agree to respect the licenses of all included emulators and to use game ROMs legally.

---

## 🤝 Credits & Acknowledgments

**Developed by:** Selman Demiray

**Special Thanks To:**
- The RetroArch/libretro team for the universal emulation framework
- All emulator developers who make game preservation possible
- The Docker community for simplifying deployment
- The Rust and React communities for excellent tools and documentation

**Emulator Credits:**
This project would not be possible without the incredible work of emulator developers worldwide. Please support these projects directly if you find them useful.

---

## 📞 Support & Contact

### Need Help?

1. **Check the [Troubleshooting](#-troubleshooting) section** - Most issues are covered there
2. **Check the [FAQ](#-frequently-asked-questions)** - Common questions answered
3. **Read the [Glossary](#-glossary)** - Understand the terminology
4. **Open an Issue** - [GitHub Issues](https://github.com/SelmanDemiray/maplex_games/issues)

### Found a Bug?

Please report bugs on our [GitHub Issues](https://github.com/SelmanDemiray/maplex_games/issues) page with:
- Your operating system
- Docker version
- Steps to reproduce the bug
- Any error messages

### Want to Contribute?

Contributions are welcome! See the [For Developers](#-for-developers) section for details on the codebase and how to contribute.

---

## 🌟 Star This Project!

If you find this project useful, please consider giving it a ⭐ on GitHub! It helps others discover the project.

---

<div align="center">

**Made with ❤️ for retro gaming enthusiasts**

[⬆ Back to Top](#-maplex-games---universal-emulator-platform)

</div>
