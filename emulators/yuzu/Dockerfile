FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    wget \
    xvfb \
    x11vnc \
    && rm -rf /var/lib/apt/lists/*

# Download and install yuzu
RUN wget -q https://github.com/yuzu-emu/yuzu-mainline/releases/download/mainline-0/yuzu-mainline-0.AppImage -O /usr/local/bin/yuzu && \
    chmod +x /usr/local/bin/yuzu

EXPOSE 8088 5900

COPY start.sh /start.sh
RUN chmod +x /start.sh

CMD ["/start.sh"]

