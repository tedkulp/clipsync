# Contributing to ClipSync

Thank you for your interest in contributing to ClipSync!

## 📝 IMPORTANT: Update CHANGELOG.md

**Before committing any changes, you MUST update CHANGELOG.md!**

Add your changes to the `[Unreleased]` section under the appropriate category:
- **Added** - New features
- **Changed** - Changes to existing functionality
- **Deprecated** - Soon-to-be removed features
- **Removed** - Removed features
- **Fixed** - Bug fixes
- **Security** - Security improvements

Example:
```markdown
## [Unreleased]

### Added
- Support for copying files in addition to text and images

### Fixed
- Connection timeout on slow networks
```

## Development Setup

1. **Install Prerequisites**
   - Rust 1.88.0 or later
   - Node.js 18 or later
   - Tauri CLI: `cargo install tauri-cli --version "^2.0" --locked`

2. **Clone and Setup**
   ```bash
   git clone https://github.com/YOUR_USERNAME/clipsync.git
   cd clipsync
   cd desktop && npm install && cd ..
   ```

3. **Run Tests**
   ```bash
   cargo test --workspace
   cargo fmt --all
   cargo clippy --workspace -- -D warnings
   ```

## Making Changes

1. **Create a Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make Your Changes**
   - Write code
   - Add tests if applicable
   - Run `cargo fmt --all`
   - Run `cargo clippy --workspace -- -D warnings`

3. **Update CHANGELOG.md**
   - Add your changes to the `[Unreleased]` section
   - Use clear, descriptive language
   - Group related changes together

4. **Test Your Changes**
   ```bash
   # Test server
   cargo test -p clipsync-server
   
   # Test desktop (if applicable)
   make dev-desktop
   
   # Test the full flow
   make run-server  # Terminal 1
   make dev-desktop # Terminal 2
   ```

5. **Commit**
   ```bash
   git add .
   git commit -m "feat: your feature description"
   ```

6. **Push and Create PR**
   ```bash
   git push origin feature/your-feature-name
   ```
   Then create a Pull Request on GitHub.

## Commit Message Format

Use conventional commits:
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting, etc.)
- `refactor:` - Code refactoring
- `test:` - Adding or updating tests
- `chore:` - Maintenance tasks

Examples:
- `feat: add support for file clipboard sync`
- `fix: resolve connection timeout on Windows`
- `docs: update installation instructions`

## Code Style

- Run `cargo fmt --all` before committing
- Run `cargo clippy --workspace -- -D warnings` and fix all warnings
- Follow Rust naming conventions
- Add comments for complex logic
- Keep functions focused and small

## Testing

- Add unit tests for new functionality
- Test on your platform (macOS, Linux, or Windows)
- Test clipboard sync with multiple devices
- Check that the UI updates correctly

## Pull Request Process

1. Ensure all tests pass
2. Update CHANGELOG.md
3. Update documentation if needed
4. Fill out the PR template
5. Wait for CI to pass
6. Address any review comments

## Questions?

- Open an issue for questions
- Check existing issues and PRs
- Read the documentation in `/docs`

## Thank You!

Your contributions make ClipSync better for everyone!
