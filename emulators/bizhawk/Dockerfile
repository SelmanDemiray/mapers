FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    wget \
    xvfb \
    x11vnc \
    mono-complete \
    && rm -rf /var/lib/apt/lists/*

# Download and install BizHawk
RUN wget -q https://github.com/TASEmulators/BizHawk/releases/download/2.9.1/BizHawk-2.9.1-linux-x64.tar.gz -O /tmp/bizhawk.tar.gz && \
    tar -xzf /tmp/bizhawk.tar.gz -C /usr/local/ && \
    rm /tmp/bizhawk.tar.gz

EXPOSE 8092 5900

COPY start.sh /start.sh
RUN chmod +x /start.sh

CMD ["/start.sh"]

