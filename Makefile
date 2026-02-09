.PHONY: help build build-server build-desktop run-server dev-desktop test clean docker-build docker-up docker-down

help:
	@echo "ClipSync - Makefile"
	@echo ""
	@echo "Available targets:"
	@echo "  build          - Build all components"
	@echo "  build-server   - Build server binary"
	@echo "  build-desktop  - Build desktop app"
	@echo "  run-server     - Run server in development mode"
	@echo "  dev-desktop    - Run desktop app in development mode"
	@echo "  test           - Run all tests"
	@echo "  clean          - Clean build artifacts"
	@echo "  docker-build   - Build Docker image"
	@echo "  docker-up      - Start server with Docker Compose"
	@echo "  docker-down    - Stop Docker Compose services"

build: build-server build-desktop

build-server:
	cargo build --release -p clipsync-server

build-desktop:
	@echo "Installing frontend dependencies..."
	cd desktop && npm install
	@echo "Building frontend..."
	cd desktop && npm run build
	@echo "Installing Tauri CLI if needed..."
	@cargo install tauri-cli --version "^2.0" --locked 2>/dev/null || true
	@echo "Generating icons..."
	cd crates/desktop && cargo tauri icon icons/icon.png --output icons
	@echo "Building desktop app..."
	cd crates/desktop && cargo tauri build

run-server:
	cargo run -p clipsync-server

dev-desktop:
	@echo "Installing Tauri CLI if needed..."
	@cargo install tauri-cli --version "^2.0" --locked 2>/dev/null || true
	cd crates/desktop && GDK_BACKEND=x11 WEBKIT_DISABLE_DMABUF_RENDERER=1 cargo tauri dev

test:
	cargo test --workspace

clean:
	cargo clean
	rm -rf desktop/dist desktop/node_modules

docker-build:
	docker build -f docker/Dockerfile -t clipsync-server .

docker-up:
	cd docker && docker-compose up -d

docker-down:
	cd docker && docker-compose down
