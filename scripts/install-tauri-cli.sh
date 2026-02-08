#!/bin/bash
# Install Tauri CLI if not already installed

set -e

echo "Checking for Tauri CLI..."

if cargo tauri --version &>/dev/null; then
    echo "✓ Tauri CLI already installed"
    cargo tauri --version
else
    echo "Installing Tauri CLI..."
    cargo install tauri-cli --version "^2.0" --locked
    echo "✓ Tauri CLI installed successfully"
fi
