# Multi-stage build for ClipSync server

# Build stage
FROM rust:1.83-slim as builder

WORKDIR /build

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml ./
COPY crates/common ./crates/common
COPY crates/server ./crates/server

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
