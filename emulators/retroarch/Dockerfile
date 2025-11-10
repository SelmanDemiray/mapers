FROM ubuntu:22.04

# Install build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    git \
    cmake \
    libsdl2-dev \
    libsdl2-image-dev \
    libsdl2-ttf-dev \
    libfreetype6-dev \
    libx11-dev \
    libx11-xcb-dev \
    libxext-dev \
    libxrandr-dev \
    libvulkan-dev \
    libgl1-mesa-dev \
    libglu1-mesa-dev \
    libopenal-dev \
    libasound2-dev \
    libpulse-dev \
    libudev-dev \
    libdrm-dev \
    libgbm-dev \
    && rm -rf /var/lib/apt/lists/*

# Clone and build RetroArch
WORKDIR /tmp
RUN git clone https://github.com/libretro/RetroArch.git retroarch && \
    cd retroarch && \
    ./configure --enable-networking && \
    make -j$(nproc) && \
    make install && \
    cd / && \
    rm -rf /tmp/retroarch

# Create directories
RUN mkdir -p /saves /states /config /roms

EXPOSE 8081

# Run RetroArch with network command interface enabled
# The network_cmd_port is set in the config file
CMD ["retroarch", "--config", "/config/retroarch.cfg"]
