FROM rust:1.85-slim

WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Install cargo-watch for hot reload
RUN cargo install cargo-watch --version 8.4.1 --locked

# Copy manifests first for dependency caching
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build 2>/dev/null || true
RUN rm -rf src

# Copy source code
COPY . .

EXPOSE 8080

CMD ["cargo", "watch", "-x", "run"]
