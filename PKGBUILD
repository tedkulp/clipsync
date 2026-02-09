# Maintainer: Your Name <ted@tedkulp.com>
pkgname=clipsync
pkgver=0.1.0
pkgrel=1
pkgdesc="Cross-platform clipboard synchronization tool"
arch=('x86_64')
url="https://github.com/tedkulp/clipsync"
license=('MIT')
depends=('webkit2gtk' 'gtk3' 'libayatana-appindicator')
makedepends=('rust' 'cargo' 'npm')
source=("$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz")
# For local testing, use: source=("clipsync::git+file://$(pwd)")
sha256sums=('SKIP') # Update with actual checksum after first release

prepare() {
  cd "$srcdir/$pkgname-$pkgver"
  # Install frontend dependencies
  cd desktop
  npm install
}

build() {
    cd "$srcdir/$pkgname-$pkgver"
    
    # Build frontend
    cd desktop
    npm run build
    cd ..
    
    # Build server
    cargo build --release -p clipsync-server
    
    # Install tauri-cli if needed
    cargo install tauri-cli --version "^2.0" --locked 2>/dev/null || true
    
    # Generate icons (creates .icns, .ico, etc from icon.png)
    cd crates/desktop
    cargo tauri icon icons/icon.png --output icons 2>/dev/null || true
    
    # Build Tauri desktop app
    # Note: We need to use 'cargo tauri build' to properly embed the frontend
    cd "$srcdir/$pkgname-$pkgver/crates/desktop"
    cargo tauri build --target x86_64-unknown-linux-gnu --bundles deb
}

package() {
    cd "$srcdir/$pkgname-$pkgver"
    
    # Install binaries
    install -Dm755 "target/release/clipsync-desktop" "$pkgdir/usr/bin/clipsync-desktop"
    install -Dm755 "target/release/clipsync-server" "$pkgdir/usr/bin/clipsync-server"
    
    # Create desktop entry with Wayland workarounds
    install -Dm644 /dev/stdin "$pkgdir/usr/share/applications/clipsync.desktop" <<EOF
[Desktop Entry]
Version=1.0
Type=Application
Name=ClipSync
Comment=Clipboard synchronization tool
Exec=env GDK_BACKEND=x11 WEBKIT_DISABLE_DMABUF_RENDERER=1 clipsync-desktop
Icon=clipsync
Terminal=false
Categories=Utility;
EOF

  # Install icons
  for size in 32x32 128x128; do
    install -Dm644 "crates/desktop/icons/${size}.png" \
      "$pkgdir/usr/share/icons/hicolor/${size}/apps/clipsync.png"
  done

    # Install systemd user service
    install -Dm644 /dev/stdin "$pkgdir/usr/lib/systemd/user/clipsync-server.service" <<EOF
[Unit]
Description=ClipSync Server
After=network.target

[Service]
Type=simple
ExecStart=/usr/bin/clipsync-server
Restart=on-failure
RestartSec=5s

[Install]
WantedBy=default.target
EOF
    
    # Install license
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
