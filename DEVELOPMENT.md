# Development Guide

## Prerequisites

- [mise](https://mise.jdx.dev/) - Development environment manager
- Docker (optional, for server deployment)

**Note**: mise will automatically install the correct versions of Rust and Node.js for you.

### Platform-specific requirements

#### macOS
```bash
xcode-select --install
```

#### Linux
```bash
# Debian/Ubuntu
sudo apt-get install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libxcb-shape0-dev \
  libxcb-xfixes0-dev

# For Wayland support
sudo apt-get install libwayland-dev
```

#### Windows
- Install Visual Studio Build Tools
- WebView2 runtime (usually pre-installed on Windows 11)

## Project Structure

```
clipsync/
├── crates/
│   ├── common/          # Shared types and protocol
│   ├── server/          # WebSocket server
│   └── desktop/         # Tauri desktop app (Rust backend)
├── desktop/             # Tauri frontend (HTML/JS)
├── docker/              # Docker configuration
└── Cargo.toml           # Workspace root
```

## Building

### Server Only

```bash
cargo build --release -p clipsync-server
./target/release/clipsync-server
```

### Desktop App

The Makefile handles all dependencies automatically:

```bash
# Development mode (installs Tauri CLI if needed)
make dev-desktop

# Production build
make build-desktop
```

Or manually:

```bash
# Install dependencies
cd desktop && npm install && cd ..
./scripts/install-tauri-cli.sh

# Run in development
cd crates/desktop
cargo tauri dev

# Build for production
cd crates/desktop
cargo tauri build
```

The built application will be in `crates/desktop/target/release/bundle/`.

## Running

### Server

```bash
# Using cargo
cargo run --release -p clipsync-server

# Or with environment variables
CLIPSYNC_PORT=8080 CLIPSYNC_MAX_HISTORY=100 cargo run --release -p clipsync-server

# Using Docker
cd docker
docker-compose up -d
```

### Desktop App

```bash
cd crates/desktop
cargo tauri dev
```

## Testing

```bash
# Test all crates
cargo test --workspace

# Test specific crate
cargo test -p clipsync-common
cargo test -p clipsync-server
```

## Debugging

### Server

Enable debug logging:

```bash
RUST_LOG=clipsync_server=debug,tower_http=debug cargo run -p clipsync-server
```

### Desktop App

The Tauri app will show console logs in the developer tools (right-click > Inspect Element).

For Rust backend logs:

```bash
RUST_LOG=clipsync_desktop=debug cargo tauri dev
```

## Cross-Platform Building

### macOS

```bash
cargo tauri build
```

Outputs:
- `.app` bundle in `target/release/bundle/macos/`
- `.dmg` installer in `target/release/bundle/dmg/`

### Linux

```bash
cargo tauri build
```

Outputs:
- `.deb` package in `target/release/bundle/deb/`
- `.AppImage` in `target/release/bundle/appimage/`

### Windows

```bash
cargo tauri build
```

Outputs:
- `.exe` installer in `target/release/bundle/nsis/`
- `.msi` installer in `target/release/bundle/msi/`

## Code Style

Format code before committing:

```bash
cargo fmt --all
```

Run linter:

```bash
cargo clippy --workspace -- -D warnings
```

## Architecture Notes

### Clipboard Monitoring

- **macOS**: Uses polling via `NSPasteboard::changeCount`
- **Windows**: Native window messages via clipboard-master
- **Linux**: X11 clipboard events (Wayland via wayland-data-control feature)

### WebSocket Protocol

All messages are JSON-encoded. See `crates/common/src/protocol.rs` for message definitions.

Client connects → sends `Join` with hashed secret → receives history → bidirectional sync begins.

### Room Management

Rooms are identified by SHA-256 hash of the shared secret. Multiple devices with the same secret join the same room. Empty rooms are cleaned up automatically.

## Troubleshooting

### "Failed to connect to display" on Linux

Make sure you have X11 or Wayland running. For Wayland, ensure the `wayland-data-control` feature is enabled in arboard.

### WebSocket connection fails

- Check that the server is running and accessible
- Verify the URL format (should be `ws://host:port` or `wss://host:port`)
- Check firewall settings

### Clipboard not syncing

- Ensure both devices are connected to the same server
- Verify they're using the same shared secret
- Check that sync is not paused
- Look at debug logs for errors
