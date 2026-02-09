# Release Process

## Current Status

The GitHub Actions workflows are set up, but some builds are still failing. Here's the status:

### Working ✅
- CI checks (format, clippy, tests)
- Server Docker build
- Server binary build

### In Progress ⚠️
- macOS desktop build (needs both targets for universal binary)
- Windows desktop build (icon generation issue)
- Linux desktop build (should work once others are fixed)

## Known Issues to Fix

### 1. Icon Generation
The `cargo tauri icon` command needs to be run successfully in CI. Current issue:
- Windows requires a proper `.ico` file
- macOS requires a proper `.icns` file

**Solution**: The workflow now runs `cargo tauri icon icons/icon.png --output icons` which should generate proper icons.

### 2. macOS Universal Binary
Requires both `aarch64-apple-darwin` and `x86_64-apple-darwin` targets.

**Solution**: The workflow now installs both targets for macOS.

## Making a Release (Once Builds Pass)

### 1. Ensure All Builds Pass

Check that the latest commit on `main` has all green checkmarks:
```
https://github.com/YOUR_USERNAME/clipsync/actions
```

### 2. Create and Push a Tag

```bash
# Update version in Cargo.toml if needed
# Then create tag
git tag v0.1.0
git push origin v0.1.0
```

### 3. Monitor the Build

Go to Actions tab and watch the "Build Desktop Apps" workflow:
```
https://github.com/YOUR_USERNAME/clipsync/actions
```

### 4. Publish the Release

Once builds complete:
1. Go to Releases page
2. Find the draft release for v0.1.0
3. Edit release notes if needed
4. Click "Publish release"

## Release Artifacts

When working, the release will include:

### Desktop Apps
- `ClipSync_VERSION_universal.dmg` - macOS (Intel + Apple Silicon)
- `clipsync_VERSION_amd64.deb` - Linux Debian/Ubuntu
- `clipsync_VERSION_amd64.AppImage` - Linux AppImage
- `ClipSync_VERSION_x64-setup.exe` - Windows NSIS installer
- `ClipSync_VERSION_x64_en-US.msi` - Windows MSI installer

### Docker Image
- `ghcr.io/YOUR_USERNAME/clipsync-server:VERSION`
- `ghcr.io/YOUR_USERNAME/clipsync-server:latest`

## Debugging Failed Builds

### Check Build Logs
1. Go to Actions tab
2. Click on the failed workflow run
3. Click on the failed job
4. Expand the failed step to see error details

### Common Issues

**"Icon not found"**
- The icon generation step failed
- Check that `icons/icon.png` exists and is valid

**"Target not installed"**
- Rust target not installed
- Check the "Setup Rust" step includes the correct targets

**"No artifacts found"**
- The build step failed before creating artifacts
- Check earlier steps in the workflow

### Testing Locally

Before pushing a tag, test the build locally:

```bash
# macOS
cd crates/desktop
cargo tauri icon icons/icon.png --output icons
cargo tauri build --target universal-apple-darwin

# Linux
cargo tauri build --target x86_64-unknown-linux-gnu

# Windows
cargo tauri build --target x86_64-pc-windows-msvc
```

## Manual Release (If CI Fails)

If you need to create a release manually:

1. Build locally on each platform
2. Collect the artifacts from `crates/desktop/target/*/release/bundle/`
3. Create a GitHub release manually
4. Upload the files

## Docker Image

The Docker image builds separately and should work even if desktop builds fail.

Pull and run:
```bash
docker pull ghcr.io/YOUR_USERNAME/clipsync-server:latest
docker run -p 8080:8080 ghcr.io/YOUR_USERNAME/clipsync-server:latest
```

## Version Numbering

Use semantic versioning:
- `v0.1.0` - Initial release
- `v0.2.0` - Minor updates, new features
- `v1.0.0` - Stable release
- `v1.0.1` - Bug fixes

## Pre-release

For testing, create a pre-release:
```bash
git tag v0.1.0-beta.1
git push origin v0.1.0-beta.1
```

This will build but mark the release as "Pre-release" on GitHub.
