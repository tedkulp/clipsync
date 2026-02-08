# ClipSync Quick Start Guide

Get up and running with ClipSync in 5 minutes!

## Step 1: Start the Server

### Option A: Using Docker (Recommended)

```bash
cd docker
docker-compose up -d
```

The server will be available at `ws://localhost:8080`

### Option B: Build from Source

```bash
cargo build --release -p clipsync-server
./target/release/clipsync-server
```

## Step 2: Install the Desktop App

### First Time Setup

Install dependencies:

```bash
# Install frontend dependencies
cd desktop
npm install
cd ..

# Install Tauri CLI (required for building desktop app)
cargo install tauri-cli --version "^2.0" --locked
```

### Run the App

```bash
cd crates/desktop
cargo tauri dev
```

For production build:

```bash
cargo tauri build
```

## Step 3: Configure and Connect

1. Open the ClipSync app
2. Enter your server URL: `ws://localhost:8080` (or your server's address)
3. Enter a shared secret (e.g., `my-secret-key-123`)
4. Click "Connect"

## Step 4: Connect Additional Devices

On each device:

1. Install and run ClipSync
2. Use the **same server URL** and **same shared secret**
3. Click "Connect"

That's it! Copy text or images on any device and watch them appear on all connected devices.

## Testing It Works

1. Connect at least 2 devices
2. Copy some text on Device A
3. The text should appear in the clipboard history on Device B
4. Paste (Cmd/Ctrl+V) on Device B - you should see the text from Device A

## Troubleshooting

### Can't connect to server

- Make sure the server is running: `docker ps` or check the terminal
- Verify the URL format: `ws://hostname:port` (not `http://`)
- Check firewall settings

### Clipboard not syncing

- Ensure all devices use the **exact same shared secret**
- Check that sync is not paused (button should say "Pause Sync", not "Resume Sync")
- Look for error messages in the app

### Server not accessible from other machines

If running locally, replace `localhost` with your machine's IP address:

```bash
# Find your IP
# macOS/Linux:
ifconfig | grep "inet "
# Windows:
ipconfig
```

Then use: `ws://192.168.1.100:8080` (replace with your actual IP)

## Next Steps

- Read [DEVELOPMENT.md](DEVELOPMENT.md) for detailed build instructions
- Check [README.md](README.md) for more features and configuration options
- Deploy the server to a cloud provider for remote syncing

## Security Note

⚠️ This setup uses unencrypted WebSockets (ws://) and no end-to-end encryption. The shared secret is hashed, but clipboard data is transmitted in plaintext.

For production use:
- Use WSS (WebSocket Secure) with TLS
- Deploy behind a reverse proxy (nginx, Caddy)
- Use a strong, unique shared secret
- Consider network-level security (VPN, firewall rules)
