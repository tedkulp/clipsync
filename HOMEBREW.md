# Homebrew Installation

ClipSync can be installed via Homebrew on macOS.

## Installation

```bash
brew tap tedkulp/tap
brew install --cask clipsync
```

## What Gets Installed

- `ClipSync.app` - Desktop application (GUI) in `/Applications`
- `clipsync-server` - WebSocket server binary

## Running

### Start the desktop app

Launch ClipSync from Applications or Spotlight.

### Start the server

```bash
clipsync-server
```

The server runs on `localhost:8080` by default.

## Configuration

The desktop app will prompt you to configure:
- Server URL (default: `ws://localhost:8080`)
- Room name
- Shared secret

## Updating

```bash
brew update
brew upgrade --cask clipsync
```

## Uninstalling

```bash
brew uninstall --cask clipsync
brew untap tedkulp/tap
```

---

## For Maintainers: Publishing a New Release

### 1. Push tag and let GitHub Actions build

```bash
git tag v0.1.1
git push && git push --tags
```

GitHub Actions will automatically build the DMG and create a draft release.

### 2. Publish the GitHub release

1. Go to https://github.com/tedkulp/clipsync/releases
2. Edit the draft release
3. Review the release notes
4. Publish the release

### 3. Update Homebrew cask

After the release is published:

```bash
./scripts/update-homebrew-formula.sh v0.1.1
```

This downloads the DMG and updates the cask with the sha256.

### 4. Push to tap repo

```bash
cp clipsync.rb ../homebrew-tap/Casks/
cd ../homebrew-tap
git add Casks/clipsync.rb
git commit -m "Update clipsync to v0.1.1"
git push
```

### 5. Test the installation

```bash
brew update
brew upgrade --cask clipsync
# or for fresh install:
brew install --cask tedkulp/tap/clipsync
```
