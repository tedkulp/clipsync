#!/bin/sh
# Wrapper script to set environment variables for Wayland compatibility

# Use X11 backend for GTK to avoid Wayland protocol issues
export GDK_BACKEND=x11

# Disable DMA-BUF renderer to fix GBM buffer errors
export WEBKIT_DISABLE_DMABUF_RENDERER=1

# Execute the actual binary
exec "$(dirname "$0")/clipsync-desktop" "$@"
