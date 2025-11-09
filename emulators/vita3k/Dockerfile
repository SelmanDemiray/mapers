FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    wget \
    xvfb \
    x11vnc \
    && rm -rf /var/lib/apt/lists/*

# Download and install Vita3K
RUN wget -q https://github.com/Vita3K/Vita3K/releases/download/continuous/Vita3K-linux-x86_64.AppImage -O /usr/local/bin/vita3k && \
    chmod +x /usr/local/bin/vita3k

EXPOSE 8091 5900

COPY start.sh /start.sh
RUN chmod +x /start.sh

CMD ["/start.sh"]

