# Multi-stage build for ClipSync server

# Build stage
FROM rust:1.83-slim as builder

WORKDIR /build

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy only server-related crates
COPY crates/common ./crates/common
COPY crates/server ./crates/server

# Create a minimal workspace Cargo.toml for server-only build
RUN cat > Cargo.toml <<'EOF'
[workspace]
members = [
    "crates/common",
    "crates/server",
]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["ClipSync Contributors"]
license = "MIT"

[workspace.dependencies]
tokio = { version = "1.41", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
thiserror = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
tokio-tungstenite = "0.24"
futures-util = "0.3"
sha2 = "0.10"
axum = { version = "0.7", features = ["ws"] }
tower = "0.5"
tower-http = { version = "0.6", features = ["trace", "cors"] }
EOF

# Build the server
RUN cargo build --release -p clipsync-server

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 clipsync

# Copy binary from builder
COPY --from=builder /build/target/release/clipsync-server /usr/local/bin/clipsync-server

# Set ownership
RUN chown clipsync:clipsync /usr/local/bin/clipsync-server

# Switch to non-root user
USER clipsync

# Expose port
EXPOSE 8080

# Set environment variables
ENV CLIPSYNC_PORT=8080
ENV CLIPSYNC_MAX_HISTORY=50
ENV RUST_LOG=clipsync_server=info

# Run the server
CMD ["clipsync-server"]
