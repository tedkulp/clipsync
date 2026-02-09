#!/bin/bash
# Update Homebrew cask with correct sha256 after creating a GitHub release

set -e

VERSION=${1:-$(git describe --tags --abbrev=0)}
VERSION=${VERSION#v}  # Remove 'v' prefix if present

echo "Updating Homebrew cask for version $VERSION..."

# Download the DMG and calculate sha256
DMG_URL="https://github.com/tedkulp/clipsync/releases/download/v${VERSION}/ClipSync_${VERSION}_universal.dmg"
DMG_FILE="/tmp/ClipSync_${VERSION}_universal.dmg"

echo "Downloading ${DMG_URL}..."
if curl -L -f -o "$DMG_FILE" "$DMG_URL"; then
  SHA256=$(shasum -a 256 "$DMG_FILE" | awk '{print $1}')
  echo "SHA256: ${SHA256}"
  rm -f "$DMG_FILE"
else
  echo "Error: Could not download DMG file"
  exit 1
fi

# Update the cask
sed -i.bak "s/version \".*\"/version \"${VERSION}\"/" clipsync.rb
sed -i.bak "s/sha256 \".*\"/sha256 \"${SHA256}\"/" clipsync.rb
rm -f clipsync.rb.bak

echo ""
echo "✓ Cask updated!"
echo ""
echo "Next steps:"
echo "1. Review the changes: git diff clipsync.rb"
echo "2. Copy to your tap repo: cp clipsync.rb ../homebrew-tap/Casks/"
echo "3. Commit and push to homebrew-tap"
echo ""
echo "Users can then install with:"
echo "  brew tap tedkulp/tap"
echo "  brew install --cask clipsync"
