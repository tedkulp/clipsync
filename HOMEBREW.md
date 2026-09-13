# Homebrew Installation

ClipSync can be installed via Homebrew on macOS.

## Installation

```bash
brew tap tedkulp/tap
brew install --cask clipsync
```

## What Gets Installed

- `ClipSync.app` - Desktop application (GUI) in `/Applications`

The cask does not include `clipsync-server`. Run the server with Docker (see below)
or build it from source with `cargo build --release -p clipsync-server`.

## Running

### Start the desktop app

Launch ClipSync from Applications or Spotlight.

### Start the server

```bash
docker run -p 8080:8080 ghcr.io/tedkulp/clipsync-server:latest
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

### 3. Homebrew tap updates automatically

Publishing the release triggers the `Update Homebrew Tap` workflow
(`.github/workflows/homebrew.yml`). It downloads the DMG, fills the version and
sha256 into `clipsync.rb`, and pushes `Casks/clipsync.rb` to
`tedkulp/homebrew-tap`. It needs a `HOMEBREW_TOKEN` repository secret with
write access to the tap.

To re-run it for a release:

```bash
gh workflow run "Update Homebrew Tap" -f tag=v0.2.0
```

### 4. Test the installation

```bash
brew update
brew upgrade --cask clipsync
# or for fresh install:
brew install --cask tedkulp/tap/clipsync
```
