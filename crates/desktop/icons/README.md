# Icons

Placeholder PNG icons have been created. Tauri will automatically generate platform-specific icons from `icon.png`.

## For Production

Generate proper icons for all platforms:

```bash
# Install Tauri CLI if not already installed
cargo install tauri-cli --version "^2.0"

# Generate icons from a 512x512 or 1024x1024 PNG
cargo tauri icon path/to/your-icon.png
```

This will create:
- **macOS**: icon.icns
- **Windows**: icon.ico (proper multi-resolution format)
- **Linux**: Various PNG sizes

## Current Setup

The current `icon.png` is a placeholder. For production:

1. Create a high-quality 1024x1024 PNG icon
2. Run `cargo tauri icon icon.png` from the `crates/desktop` directory
3. Commit the generated icons

## Manual Icon Creation

If you can't use the Tauri icon generator:

### macOS (.icns)
```bash
mkdir icon.iconset
sips -z 16 16 icon.png --out icon.iconset/icon_16x16.png
sips -z 32 32 icon.png --out icon.iconset/icon_16x16@2x.png
sips -z 32 32 icon.png --out icon.iconset/icon_32x32.png
sips -z 64 64 icon.png --out icon.iconset/icon_32x32@2x.png
sips -z 128 128 icon.png --out icon.iconset/icon_128x128.png
sips -z 256 256 icon.png --out icon.iconset/icon_128x128@2x.png
sips -z 256 256 icon.png --out icon.iconset/icon_256x256.png
sips -z 512 512 icon.png --out icon.iconset/icon_256x256@2x.png
sips -z 512 512 icon.png --out icon.iconset/icon_512x512.png
cp icon.png icon.iconset/icon_512x512@2x.png
iconutil -c icns icon.iconset
```

### Windows (.ico)
Use ImageMagick or an online converter to create a proper multi-resolution .ico file.

### Linux
Just use the PNG files directly.
