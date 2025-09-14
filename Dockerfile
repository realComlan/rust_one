# Step 1: builder
FROM rust:1.89 AS builder

WORKDIR /app
COPY . .

# Installer les dépendances systèmes
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Compile in release mode
RUN cargo build --release

# Step 2: runtime minimal
FROM debian:bookworm-slim

WORKDIR /app

# Copy the compiled binary
COPY --from=builder /app/target/release/rust_one .

# Exposer le port 8080 (Railway attend 8080)
EXPOSE 8080

# Launch Dockerfile
CMD ["./rust_one"]
