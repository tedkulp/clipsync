#!/bin/bash
# Update Homebrew formula with correct sha256 after creating a GitHub release

set -e

VERSION=${1:-$(git describe --tags --abbrev=0)}
VERSION=${VERSION#v}  # Remove 'v' prefix if present

echo "Updating Homebrew formula for version $VERSION..."

# Download the release tarball
TARBALL_URL="https://github.com/tedkulp/clipsync/archive/refs/tags/v${VERSION}.tar.gz"
TARBALL="/tmp/clipsync-${VERSION}.tar.gz"

echo "Downloading $TARBALL_URL..."
curl -L -o "$TARBALL" "$TARBALL_URL"

# Calculate sha256
SHA256=$(shasum -a 256 "$TARBALL" | awk '{print $1}')
echo "SHA256: $SHA256"

# Update the formula
sed -i.bak "s|sha256 \".*\"|sha256 \"$SHA256\"|" clipsync.rb
rm -f clipsync.rb.bak

echo "Formula updated! Now:"
echo "1. Review the changes: git diff clipsync.rb"
echo "2. Copy to your tap repo: cp clipsync.rb ../homebrew-tap/Formula/"
echo "3. Commit and push to homebrew-tap"
echo ""
echo "Users can then install with:"
echo "  brew tap tedkulp/tap"
echo "  brew install clipsync"
