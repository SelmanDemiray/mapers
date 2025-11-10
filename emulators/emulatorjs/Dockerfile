FROM node:18-alpine

# Install dependencies
RUN apk add --no-cache \
    git

# Install http-server globally
RUN npm install -g http-server

# Clone EmulatorJS repository
WORKDIR /app
RUN git clone --depth 1 https://github.com/ethanaobrien/emulatorjs.git . && \
    mkdir -p dist && \
    cp -r data dist/ && \
    cp -r emulators dist/ && \
    cp index.html dist/ 2>/dev/null || true

# If index.html doesn't exist, create a basic one
RUN if [ ! -f dist/index.html ]; then \
        echo '<!DOCTYPE html><html><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><title>EmulatorJS</title><style>body{margin:0;padding:20px;font-family:Arial,sans-serif;background:#1a1a1a;color:#fff}#emulator{width:100%;height:80vh;border:2px solid #333}</style></head><body><h1>EmulatorJS</h1><div id="emulator"></div><script src="data/loader.js"></script><script>EJS_player="#emulator";EJS_gameUrl=new URLSearchParams(window.location.search).get("rom")||"";EJS_core=new URLSearchParams(window.location.search).get("core")||"nes";EJS_startOnLoaded=true;</script></body></html>' > dist/index.html; \
    fi

# Create startup script that sets up symlinks and serves files
RUN printf '#!/bin/sh\ncd /app/dist\nif [ -d /data/roms ]; then\n  for dir in /data/roms/*/; do\n    if [ -d "$dir" ]; then\n      sysname=$(basename "$dir")\n      ln -sf "$dir" "/app/dist/$sysname" 2>/dev/null || true\n    fi\n  done\nfi\nmkdir -p /app/dist/data && ln -sf /data/roms /app/dist/data/roms 2>/dev/null || true\nexec http-server . -p 80 --cors -a 0.0.0.0\n' > /start.sh && \
    chmod +x /start.sh

EXPOSE 80

WORKDIR /app/dist

# Use startup script
CMD ["/start.sh"]
