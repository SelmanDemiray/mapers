FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    flycast \
    xvfb \
    x11vnc \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8090 5900

COPY start.sh /start.sh
RUN chmod +x /start.sh

CMD ["/start.sh"]

