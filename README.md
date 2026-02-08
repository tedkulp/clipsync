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

- [mise](https://mise.jdx.dev/) - Development environment manager
- Docker (optional, for server deployment)

**First time setup?** See [SETUP.md](SETUP.md) for detailed installation instructions.

Quick setup:
```bash
# Install mise
curl https://mise.run | sh  # or: brew install mise

# Install project dependencies
mise install

# Install Tauri CLI
./scripts/install-tauri-cli.sh
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
make build-server
./target/release/clipsync-server

# Or run directly
make run-server
```

### Running the Desktop App

```bash
make dev-desktop

# Or build for production
make build-desktop
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

## License

MIT
