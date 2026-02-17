# ClipSync task runner. Run `just` to list recipes.

desktop_dir := "desktop"
tauri_dir := "crates/desktop"

# List available recipes
default:
    @just --list

# Build all components
build: build-server build-desktop

# Build server binary
build-server:
    cargo build --release -p clipsync-server

# Build desktop app
build-desktop: _tauri-cli _npm-install
    npm --prefix {{ desktop_dir }} run build
    cd {{ tauri_dir }} && NO_STRIP=true cargo tauri build

# Run desktop app in development mode
dev-desktop: _tauri-cli _npm-install
    cd {{ tauri_dir }} && GDK_BACKEND=x11 WEBKIT_DISABLE_DMABUF_RENDERER=1 cargo tauri dev

# Run server in development mode
run-server:
    cargo run -p clipsync-server

# Regenerate app icons from crates/desktop/icons/icon.png
icons: _tauri-cli
    cd {{ tauri_dir }} && NO_STRIP=true cargo tauri icon icons/icon.png --output icons

# Run all tests
test:
    cargo test --workspace

# Clean build artifacts
clean:
    cargo clean
    rm -rf {{ desktop_dir }}/dist {{ desktop_dir }}/node_modules

# Build Docker image
docker-build:
    docker build -f docker/Dockerfile -t clipsync-server .

# Start server with Docker Compose
docker-up:
    cd docker && docker compose up -d

# Stop Docker Compose services
docker-down:
    cd docker && docker compose down

# Install the Tauri CLI only if it is not already on PATH.
_tauri-cli:
    #!/usr/bin/env bash
    set -euo pipefail
    if ! command -v cargo-tauri >/dev/null 2>&1; then
        echo "Installing Tauri CLI..."
        cargo install tauri-cli --version "^2.0" --locked
    fi

# Install frontend deps only when the manifests are newer than node_modules.
_npm-install:
    #!/usr/bin/env bash
    set -euo pipefail
    modules="{{ desktop_dir }}/node_modules"
    if [ ! -d "$modules" ] \
        || [ "{{ desktop_dir }}/package.json" -nt "$modules" ] \
        || [ "{{ desktop_dir }}/package-lock.json" -nt "$modules" ]; then
        npm --prefix {{ desktop_dir }} install
        touch "$modules"
    fi
