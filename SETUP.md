# ClipSync Setup Guide

## Initial Setup

### 1. Install mise

mise is a development environment manager that handles Rust and Node.js versions.

**macOS/Linux:**
```bash
curl https://mise.run | sh

# Or with Homebrew
brew install mise

# Add to your shell (bash/zsh)
echo 'eval "$(mise activate bash)"' >> ~/.bashrc  # for bash
echo 'eval "$(mise activate zsh)"' >> ~/.zshrc   # for zsh
```

**Windows:**
```powershell
# Using scoop
scoop install mise

# Or download from https://mise.jdx.dev
```

### 2. Clone and Setup Project

```bash
# Clone the repository
cd /path/to/clipsync

# Install Rust and Node.js via mise
mise install

# Verify installation
rustc --version
node --version
```

### 3. Install Tauri CLI

The Tauri CLI is required to build the desktop application:

```bash
# Option 1: Use the provided script
./scripts/install-tauri-cli.sh

# Option 2: Install manually
cargo install tauri-cli --version "^2.0" --locked

# Verify installation
cargo tauri --version
```

### 4. Install Frontend Dependencies

```bash
cd desktop
npm install
cd ..
```

## Building and Running

### Quick Start with Makefile

The Makefile handles everything automatically:

```bash
# Start server
make run-server

# Start desktop app (in another terminal)
make dev-desktop
```

### Manual Commands

If you prefer to run commands directly:

```bash
# Server
cargo run -p clipsync-server

# Desktop app
cd crates/desktop
cargo tauri dev
```

## Troubleshooting

### "no such command: tauri"

This means the Tauri CLI is not installed. Run:

```bash
./scripts/install-tauri-cli.sh
```

Or install manually:

```bash
cargo install tauri-cli --version "^2.0" --locked
```

### "command not found: mise"

mise is not installed or not in your PATH. See step 1 above.

### "rustc: command not found"

mise hasn't installed Rust yet. Run:

```bash
mise install
```

### Build fails with "edition2024" error

You need Rust 1.88.0 or later. Update mise:

```bash
mise use rust@latest
mise install
```

### npm install fails

Make sure Node.js is installed via mise:

```bash
mise install node
```

## Environment Variables

You can customize the server with environment variables:

```bash
# In .mise.local.toml (gitignored)
[env]
RUST_LOG = "debug"
CLIPSYNC_PORT = "8080"
CLIPSYNC_MAX_HISTORY = "100"
```

Or set them directly:

```bash
CLIPSYNC_PORT=9000 make run-server
```

## Next Steps

Once setup is complete:

1. Read [QUICKSTART.md](QUICKSTART.md) for a 5-minute tutorial
2. Read [DEVELOPMENT.md](DEVELOPMENT.md) for detailed development info
3. Read [TESTING.md](TESTING.md) for testing procedures

## Common Development Workflow

```bash
# Terminal 1: Server with debug logging
RUST_LOG=debug make run-server

# Terminal 2: Desktop app
make dev-desktop

# Terminal 3: Run tests
make test

# Build everything for production
make build
```
