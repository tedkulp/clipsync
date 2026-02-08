# Testing ClipSync

## Manual Testing Guide

### Prerequisites

1. Start the server:
```bash
make run-server
# Or: cargo run -p clipsync-server
```

2. In separate terminals, start 2+ desktop clients:
```bash
make dev-desktop
```

### Test Cases

#### 1. Basic Text Sync

**Steps:**
1. Connect both clients to `ws://localhost:8080` with the same shared secret (e.g., `test-secret-123`)
2. On Client A: Copy some text (Cmd/Ctrl+C)
3. On Client B: Check the clipboard history in the app
4. On Client B: Paste (Cmd/Ctrl+V)

**Expected:** Text from Client A appears in Client B's clipboard

#### 2. Image Sync

**Steps:**
1. On Client A: Copy an image (screenshot, image from browser, etc.)
2. On Client B: Check clipboard history - should show `[Image: image/png]`
3. On Client B: Paste into an image editor

**Expected:** Image from Client A appears in Client B's clipboard

#### 3. Multiple Clients

**Steps:**
1. Connect 3+ clients with the same secret
2. Copy text on Client A
3. Verify it appears on Clients B and C

**Expected:** All clients receive the clipboard update

#### 4. History on Join

**Steps:**
1. With Client A connected, copy 5 different text items
2. Connect Client B with the same secret
3. Check Client B's history list

**Expected:** Client B shows the last 50 items (or fewer if less than 50 were copied)

#### 5. Pause/Resume

**Steps:**
1. Connect Client A
2. Click "Pause Sync"
3. Copy text on Client A
4. Verify it does NOT sync to other clients
5. Click "Resume Sync"
6. Copy new text
7. Verify it DOES sync

**Expected:** Sync respects pause state

#### 6. Reconnection

**Steps:**
1. Connect Client A
2. Stop the server
3. Observe connection status changes to "Disconnected"
4. Restart the server
5. Click "Connect" again

**Expected:** Client reconnects successfully

#### 7. Different Secrets

**Steps:**
1. Connect Client A with secret `room-1`
2. Connect Client B with secret `room-2`
3. Copy text on Client A

**Expected:** Client B does NOT receive the clipboard (different rooms)

### Automated Tests

Run unit tests:
```bash
make test
# Or: cargo test --workspace
```

### Performance Testing

Test with large clipboard items:

1. **Large Text:** Copy a 1MB text file
2. **Large Image:** Copy a high-resolution screenshot
3. **Rapid Changes:** Copy 10 items in quick succession

Monitor server logs for any errors or performance issues.

### Platform-Specific Testing

#### macOS
- Test with native screenshot tool (Cmd+Shift+4)
- Test with different apps (Safari, Chrome, TextEdit)

#### Linux
- Test with X11 and Wayland
- Test with different clipboard managers

#### Windows
- Test with Windows clipboard history
- Test with different applications

## Troubleshooting Tests

### Server not accessible
```bash
# Check if server is running
curl http://localhost:8080/health

# Check server logs
RUST_LOG=debug cargo run -p clipsync-server
```

### Clipboard not syncing
1. Check connection status in app
2. Verify same shared secret on all clients
3. Check that sync is not paused
4. Look at browser console (right-click > Inspect > Console)
5. Check Rust logs in terminal

### Build failures
```bash
# Clean and rebuild
make clean
make build

# Update dependencies
cargo update
```

## CI/CD Testing

For automated testing in CI:

```bash
# Run tests
cargo test --workspace --all-features

# Check formatting
cargo fmt --all -- --check

# Run linter
cargo clippy --workspace -- -D warnings

# Build all targets
cargo build --workspace --all-targets
```
