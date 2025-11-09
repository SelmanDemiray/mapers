FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    wget \
    xvfb \
    x11vnc \
    && rm -rf /var/lib/apt/lists/*

# Download and install Citra
RUN wget -q https://github.com/citra-emu/citra-nightly/releases/download/nightly-1/citra-linux.tar.xz -O /tmp/citra.tar.xz && \
    tar -xf /tmp/citra.tar.xz -C /usr/local/ && \
    rm /tmp/citra.tar.xz

EXPOSE 8087 5900

COPY start.sh /start.sh
RUN chmod +x /start.sh

CMD ["/start.sh"]

