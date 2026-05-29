FROM rust:1.85-slim

WORKDIR /app

# Install system dependencies & precompiled cargo-watch binary
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    curl \
    tar \
    && curl -L https://github.com/watchexec/cargo-watch/releases/download/v8.4.1/cargo-watch-v8.4.1-x86_64-unknown-linux-musl.tar.xz | tar -xJ --strip-components=1 -C /usr/local/bin \
    && apt-get purge -y curl tar \
    && apt-get autoremove -y \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests and source code
COPY Cargo.toml Cargo.lock* ./
COPY src ./src

EXPOSE 8080

CMD ["cargo", "watch", "-x", "run"]
