# ClipSync Setup Guide

## Initial Setup

### 1. Install Prerequisites

#### Rust (1.88.0 or later)

**macOS/Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Windows:**
Download and run the installer from [rustup.rs](https://rustup.rs/)

**Verify installation:**
```bash
rustc --version
cargo --version
```

#### Node.js (18 or later)

**macOS:**
```bash
brew install node
```

**Ubuntu/Debian:**
```bash
curl -fsSL https://deb.nodesource.com/setup_22.x | sudo -E bash -
sudo apt-get install -y nodejs
```

**Windows:**
Download from [nodejs.org](https://nodejs.org/)

**Verify installation:**
```bash
node --version
npm --version
```

### 2. Clone the Repository

```bash
git clone https://github.com/YOUR_USERNAME/clipsync.git
cd clipsync
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

### "command not found: rustc"

Rust is not installed or not in your PATH. Install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then restart your terminal or run:
```bash
source $HOME/.cargo/env
```

### Build fails with "edition2024" error

You need Rust 1.88.0 or later. Update Rust:

```bash
rustup update stable
```

### "command not found: npm"

Node.js is not installed. See step 1 above for installation instructions.

### npm install fails

Make sure Node.js is installed:

```bash
node --version
npm --version
```

If they're not found, install Node.js (see step 1).

### Platform-Specific Issues

#### Linux: "failed to load shared library"

Install required dependencies:

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
  libxcb-xfixes0-dev \
  libwayland-dev
```

#### macOS: "xcrun: error"

Install Xcode Command Line Tools:

```bash
xcode-select --install
```

#### Windows: Build fails

Make sure you have Visual Studio Build Tools installed. Download from [visualstudio.microsoft.com](https://visualstudio.microsoft.com/downloads/)

## Environment Variables

You can customize the server with environment variables:

```bash
# Set directly when running
RUST_LOG=debug CLIPSYNC_PORT=8080 cargo run -p clipsync-server

# Or with make
CLIPSYNC_PORT=9000 make run-server

# Or export them
export RUST_LOG=debug
export CLIPSYNC_PORT=8080
export CLIPSYNC_MAX_HISTORY=100
cargo run -p clipsync-server
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

## Using mise (Optional)

If you prefer to use [mise](https://mise.jdx.dev/) for version management, the project includes `.mise.toml`:

```bash
# Install mise
curl https://mise.run | sh

# Install versions specified in .mise.toml
mise install

# Run commands with mise
mise exec -- cargo run -p clipsync-server
```

This is optional - you can use system-installed Rust and Node.js instead.
