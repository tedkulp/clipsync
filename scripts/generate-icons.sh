#!/bin/bash

# Simple script to generate placeholder icons for development
# For production, use: cargo tauri icon path/to/your-icon.png

ICON_DIR="crates/desktop/icons"
mkdir -p "$ICON_DIR"

# Create a simple SVG icon
cat > "$ICON_DIR/icon.svg" << 'EOF'
<svg width="512" height="512" xmlns="http://www.w3.org/2000/svg">
  <rect width="512" height="512" fill="#5c9eff"/>
  <path d="M 128 128 L 384 128 L 384 256 L 128 256 Z" fill="white" opacity="0.9"/>
  <path d="M 128 288 L 384 288 L 384 384 L 128 384 Z" fill="white" opacity="0.7"/>
  <circle cx="352" cy="352" r="24" fill="#5c9eff"/>
  <circle cx="352" cy="352" r="16" fill="white"/>
</svg>
EOF

echo "Generated placeholder SVG icon at $ICON_DIR/icon.svg"
echo ""
echo "To generate proper icons for all platforms, install ImageMagick and run:"
echo "  cargo tauri icon $ICON_DIR/icon.svg"
echo ""
echo "Or manually create these files in $ICON_DIR/:"
echo "  - 32x32.png"
echo "  - 128x128.png"
echo "  - 128x128@2x.png"
echo "  - icon.icns (macOS)"
echo "  - icon.ico (Windows)"
echo "  - icon.png (Linux tray icon)"
