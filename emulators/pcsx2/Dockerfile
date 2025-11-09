FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    pcsx2 \
    xvfb \
    x11vnc \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8084 5900

COPY start.sh /start.sh
RUN chmod +x /start.sh

CMD ["/start.sh"]

