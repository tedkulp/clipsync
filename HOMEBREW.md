# Homebrew Installation

ClipSync can be installed via Homebrew on macOS and Linux.

## Installation

```bash
brew tap tedkulp/tap
brew install clipsync
```

## What Gets Installed

- `clipsync-server` - WebSocket server for clipboard synchronization
- `clipsync-desktop` - Desktop application (GUI)

## Running

### Start the server

```bash
# Run manually
clipsync-server

# Or use Homebrew services (runs in background)
brew services start clipsync
```

### Start the desktop app

```bash
clipsync-desktop
```

On macOS, you can also launch it from Applications.

## Configuration

The desktop app will prompt you to configure:
- Server URL (default: `ws://localhost:8080`)
- Room name
- Shared secret

## Updating

```bash
brew update
brew upgrade clipsync
```

## Uninstalling

```bash
brew services stop clipsync  # If using services
brew uninstall clipsync
brew untap tedkulp/tap
```

---

## For Maintainers: Publishing a New Release

After creating a GitHub release:

1. Run the update script:
   ```bash
   ./scripts/update-homebrew-formula.sh v0.1.1
   ```

2. Copy the formula to your tap repo:
   ```bash
   cp clipsync.rb ../homebrew-tap/Formula/
   cd ../homebrew-tap
   ```

3. Commit and push:
   ```bash
   git add Formula/clipsync.rb
   git commit -m "Update clipsync to v0.1.1"
   git push
   ```

4. Test the installation:
   ```bash
   brew update
   brew upgrade clipsync
   # or for fresh install:
   brew install tedkulp/tap/clipsync
   ```
