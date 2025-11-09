FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    wget \
    xvfb \
    x11vnc \
    && rm -rf /var/lib/apt/lists/*

# Download and install RPCS3
RUN wget -q https://github.com/RPCS3/rpcs3-binaries-linux/releases/download/appimage/rpcs3-v0.0.30-15804-2b6c4a5e_linux64.AppImage -O /usr/local/bin/rpcs3 && \
    chmod +x /usr/local/bin/rpcs3

EXPOSE 8085 5900

COPY start.sh /start.sh
RUN chmod +x /start.sh

CMD ["/start.sh"]

