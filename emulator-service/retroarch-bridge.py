#!/usr/bin/env python3
"""
HTTP Bridge for RetroArch Network Command Interface
Translates HTTP requests to RetroArch's TCP command protocol
"""
import http.server
import socketserver
import urllib.parse
import socket
import json
from typing import Optional

RETROARCH_HOST = "localhost"
RETROARCH_PORT = 8081

class RetroArchBridge(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        parsed_path = urllib.parse.urlparse(self.path)
        query_params = urllib.parse.parse_qs(parsed_path.query)
        
        if parsed_path.path == "/play":
            core = query_params.get('core', [None])[0]
            rom = query_params.get('rom', [None])[0]
            
            if not core or not rom:
                self.send_error(400, "Missing core or rom parameter")
                return
            
            # Decode the ROM path
            rom_path = urllib.parse.unquote(rom)
            
            # Convert ROM path to EmulatorJS format
            # Backend provides paths like /roms/system/game.rom
            # EmulatorJS expects paths relative to /data/roms (the mounted volume)
            # The volume is mounted at /data/roms, so we need the path relative to that
            if rom_path.startswith('/roms/'):
                # Remove /roms/ prefix to get relative path
                emulatorjs_rom_path = rom_path.replace('/roms/', '')
            elif rom_path.startswith('roms/'):
                emulatorjs_rom_path = rom_path.replace('roms/', '')
            else:
                emulatorjs_rom_path = rom_path.lstrip('/')
            
            # Map RetroArch cores to EmulatorJS cores
            core_mapping = {
                'fceumm_libretro': 'nes',
                'nestopia_libretro': 'nes',
                'snes9x_libretro': 'snes',
                'gambatte_libretro': 'gb',
                'mgba_libretro': 'gba',
                'genesis_plus_gx_libretro': 'genesis',
            }
            
            emulatorjs_core = core_mapping.get(core, 'nes')  # Default to NES
            
            # EmulatorJS URL format: /?rom=path/to/rom&core=corename
            # The ROM path should be relative to the /data/roms mount point
            emulatorjs_url = f"http://localhost:8082/?rom={urllib.parse.quote(emulatorjs_rom_path)}&core={emulatorjs_core}"
            
            # Create HTML page that can launch via EmulatorJS
            html = f"""<!DOCTYPE html>
<html>
<head>
    <title>RetroArch Player</title>
    <style>
        body {{
            font-family: Arial, sans-serif;
            text-align: center;
            padding: 50px;
            background: #1a1a1a;
            color: #fff;
        }}
        .container {{
            max-width: 800px;
            margin: 0 auto;
            background: #2a2a2a;
            padding: 30px;
            border-radius: 10px;
        }}
        .status {{
            margin: 20px 0;
            padding: 15px;
            border-radius: 5px;
            background: #333;
        }}
        .error {{
            background: #5a1a1a;
            color: #ff6b6b;
        }}
        .success {{
            background: #1a5a1a;
            color: #6bff6b;
        }}
        button {{
            padding: 15px 30px;
            font-size: 16px;
            background: #4a9eff;
            color: white;
            border: none;
            border-radius: 5px;
            cursor: pointer;
            margin: 10px;
        }}
        button:hover {{
            background: #5aaeff;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>🎮 RetroArch Player</h1>
        <div id="status" class="status">Connecting to RetroArch...</div>
        <button onclick="loadGame()">Load Game</button>
        <button onclick="reset()">Reset</button>
        <button onclick="closeWindow()">Close</button>
    </div>
    
    <script>
        const core = {json.dumps(core)};
        const rom = {json.dumps(rom)};
        let socket = null;
        
        function connect() {{
            const status = document.getElementById('status');
            status.textContent = 'Ready to launch game...';
            status.className = 'status success';
        }}
        
        function loadGame() {{
            const status = document.getElementById('status');
            status.textContent = 'Launching game in EmulatorJS...';
            status.className = 'status';
            
            // Launch via EmulatorJS
            window.location.href = '{emulatorjs_url}';
        }}
        
        function reset() {{
            if (window.location.href.includes('emulatorjs')) {{
                // If we're in EmulatorJS, reload the page
                window.location.reload();
            }} else {{
                loadGame();
            }}
        }}
        
        function closeWindow() {{
            window.close();
        }}
        
        // Auto-launch game on load
        window.onload = function() {{
            connect();
            // Auto-launch after a short delay
            setTimeout(loadGame, 1000);
        }};
    </script>
</body>
</html>"""
            
            self.send_response(200)
            self.send_header('Content-type', 'text/html')
            self.end_headers()
            self.wfile.write(html.encode())
        
        elif parsed_path.path == "/health" or parsed_path.path == "/":
            # Health check endpoint - always return OK since bridge is running
            self.send_response(200)
            self.send_header('Content-type', 'application/json')
            self.end_headers()
            self.wfile.write(json.dumps({"status": "ok", "service": "retroarch-bridge"}).encode())
        
        else:
            self.send_error(404, "Not Found")
    
    def log_message(self, format, *args):
        # Suppress default logging
        pass

def run_server(port=8081):
    with socketserver.TCPServer(("", port), RetroArchBridge) as httpd:
        print(f"RetroArch HTTP Bridge running on port {port}")
        httpd.serve_forever()

if __name__ == "__main__":
    run_server()

