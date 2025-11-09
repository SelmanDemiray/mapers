FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    wget \
    xvfb \
    x11vnc \
    && rm -rf /var/lib/apt/lists/*

# Download and install Ryujinx
RUN wget -q https://github.com/Ryujinx/release-channel-master/releases/download/1.1.0/ryujinx-1.1.0-linux_x64.tar.gz -O /tmp/ryujinx.tar.gz && \
    tar -xzf /tmp/ryujinx.tar.gz -C /usr/local/ && \
    rm /tmp/ryujinx.tar.gz

EXPOSE 8089 5900

COPY start.sh /start.sh
RUN chmod +x /start.sh

CMD ["/start.sh"]

