# ClipSync

[![CI](https://github.com/tedkulp/clipsync/actions/workflows/ci.yml/badge.svg)](https://github.com/tedkulp/clipsync/actions/workflows/ci.yml)
[![Build Desktop](https://github.com/tedkulp/clipsync/actions/workflows/build-desktop.yml/badge.svg)](https://github.com/tedkulp/clipsync/actions/workflows/build-desktop.yml)
[![Build Server](https://github.com/tedkulp/clipsync/actions/workflows/build-server.yml/badge.svg)](https://github.com/tedkulp/clipsync/actions/workflows/build-server.yml)

Cross-platform clipboard synchronization application with real-time syncing across Mac, Linux, and Windows.

## Features

- 🔄 Real-time clipboard sync across multiple devices
- 📝 Text and image support
- 🔒 Shared secret authentication
- 📜 Short history buffer (last 50 items)
- 🖥️ System tray integration
- 🐳 Docker-ready server

## Architecture

ClipSync consists of two main components:

1. **Desktop App** (Tauri): Cross-platform desktop application with system tray
2. **Server** (Rust/Axum): WebSocket server that coordinates clipboard syncing

## Prerequisites

- Rust 1.88.0 or later
- Node.js 18 or later
- Docker (optional, for server deployment)

**First time setup?** See [SETUP.md](SETUP.md) for detailed installation instructions.

Quick setup:
```bash
# Install Tauri CLI (if not already installed)
cargo install tauri-cli --version "^2.0" --locked

# Install frontend dependencies
cd desktop && npm install && cd ..

# Or use the Makefile for convenience
make help
```

## Quick Start

### Running the Server

Using Docker:

```bash
cd docker
docker-compose up -d
```

Or build from source:

```bash
# Using Makefile
make run-server

# Or manually
cargo run -p clipsync-server
```

### Running the Desktop App

```bash
# Using Makefile
make dev-desktop

# Or manually
cd crates/desktop
cargo tauri dev

# Build for production
make build-desktop
# Or: cd crates/desktop && cargo tauri build
```

## Configuration

### Server

Environment variables:
- `CLIPSYNC_PORT`: Server port (default: 8080)
- `CLIPSYNC_MAX_HISTORY`: Maximum history items per room (default: 50)

### Desktop App

Configure via system tray:
- Server URL (e.g., `ws://localhost:8080`)
- Shared secret (all devices must use the same secret)

## Building

### Desktop App for All Platforms

```bash
# macOS
cargo tauri build

# Linux
cargo tauri build

# Windows
cargo tauri build
```

### Server Docker Image

```bash
docker build -f docker/Dockerfile -t clipsync-server .
```

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. **Update CHANGELOG.md** with your changes
5. Run tests and formatting (`cargo test && cargo fmt`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

See [CHANGELOG.md](CHANGELOG.md) for version history.

## License

MIT
