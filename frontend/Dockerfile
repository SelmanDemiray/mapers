FROM node:18-alpine

WORKDIR /app

# Accept build argument for API URL
ARG REACT_APP_API_URL=http://localhost:37291
ENV REACT_APP_API_URL=$REACT_APP_API_URL

# Set environment variables for React dev server
ENV HOST=0.0.0.0
ENV PORT=3000
ENV CHOKIDAR_USEPOLLING=true
ENV WDS_SOCKET_HOST=0.0.0.0
ENV WDS_SOCKET_PORT=3000
ENV FAST_REFRESH=false
ENV SKIP_PREFLIGHT_CHECK=true
ENV TSC_COMPILE_ON_ERROR=true
ENV DISABLE_ESLINT_PLUGIN=true
ENV GENERATE_SOURCEMAP=false
ENV TSC_COMPILE_ON_ERROR=true
ENV TS_NODE_TRANSPILE_ONLY=true

# Copy package files first for better caching
COPY package.json ./

# Install dependencies with clean cache and ensure correct versions
RUN npm cache clean --force && \
    rm -rf node_modules package-lock.json && \
    npm install --legacy-peer-deps

# Copy the rest of the application
COPY . .

# Make start script executable and ensure it has Unix line endings
RUN chmod +x /app/start.sh && \
    sed -i 's/\r$//' /app/start.sh || true

EXPOSE 3000

# Start the React development server
CMD ["/app/start.sh"]

