# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

<!-- Add your changes here before committing! -->

## [0.1.0] - 2026-02-09

### Added
- Initial implementation of ClipSync
- Cross-platform desktop application (macOS, Linux, Windows) using Tauri
- Rust-based WebSocket server for clipboard synchronization
- Text and image clipboard support
- System tray integration with show/hide functionality
- Auto-connect on startup when configured
- Minimize to tray instead of closing
- Start on boot option (preference saved)
- Start minimized option
- Shared secret authentication with SHA-256 hashing
- Room-based architecture for multi-device sync
- Short history buffer (50 items per room, configurable)
- Docker deployment support with multi-arch images (amd64, arm64)
- GitHub Actions CI/CD for all platforms
- Comprehensive documentation (README, QUICKSTART, DEVELOPMENT, TESTING, SETUP)

### Technical Details
- Uses `arboard` for cross-platform clipboard access
- Uses `clipboard-master` for clipboard change detection
- WebSocket protocol with JSON messages
- Axum web framework for server
- In-memory history with automatic cleanup
- Tauri v2 for desktop application

---

## How to Update This Changelog

**IMPORTANT**: Update this file whenever you make changes to the project!

### When to Update
- Before committing any feature, fix, or change
- When creating a pull request
- Before creating a release

### Categories
- **Added** - New features
- **Changed** - Changes to existing functionality
- **Deprecated** - Soon-to-be removed features
- **Removed** - Removed features
- **Fixed** - Bug fixes
- **Security** - Security improvements

### Format
```markdown
## [Unreleased]

### Added
- Your new feature description

### Fixed
- Bug fix description
```

### Before Release
1. Change `[Unreleased]` to `[X.Y.Z] - YYYY-MM-DD`
2. Add a new `[Unreleased]` section at the top
3. Update the version date
4. Commit with message: "Release vX.Y.Z"
5. Create git tag: `git tag vX.Y.Z`
6. Push: `git push && git push --tags`

### Example
```markdown
## [Unreleased]

### Added
- New clipboard format support

## [0.2.0] - 2026-02-15

### Added
- Image clipboard sync
- System tray integration

### Fixed
- Connection timeout issues

## [0.1.0] - 2026-02-09

### Added
- Initial release
```
