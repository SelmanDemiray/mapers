FROM node:18-alpine

# Install dependencies
RUN apk add --no-cache \
    git \
    python3 \
    make \
    g++

# Clone EmulatorJS repository
WORKDIR /app
RUN git clone https://github.com/ethanaobrien/emulatorjs.git . && \
    npm install && \
    npm run build

# Serve the built files
EXPOSE 80

CMD ["npx", "http-server", "dist", "-p", "80", "--cors"]
