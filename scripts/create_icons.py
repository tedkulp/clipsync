#!/usr/bin/env python3
"""Create simple placeholder PNG icons for Tauri"""

import struct
import zlib

def create_png(width, height, color=(92, 158, 255, 255)):
    """Create a simple solid color RGBA PNG"""
    # PNG signature
    png = b'\x89PNG\r\n\x1a\n'
    
    # IHDR chunk - color type 6 = RGBA
    ihdr_data = struct.pack('>IIBBBBB', width, height, 8, 6, 0, 0, 0)
    ihdr = b'IHDR' + ihdr_data
    png += struct.pack('>I', len(ihdr_data)) + ihdr + struct.pack('>I', zlib.crc32(ihdr))
    
    # IDAT chunk - image data
    raw_data = b''
    for y in range(height):
        raw_data += b'\x00'  # Filter type
        for x in range(width):
            raw_data += bytes(color)  # RGBA
    
    compressed = zlib.compress(raw_data, 9)
    idat = b'IDAT' + compressed
    png += struct.pack('>I', len(compressed)) + idat + struct.pack('>I', zlib.crc32(idat))
    
    # IEND chunk
    iend = b'IEND'
    png += struct.pack('>I', 0) + iend + struct.pack('>I', zlib.crc32(iend))
    
    return png

# Create icons directory
import os
icon_dir = 'crates/desktop/icons'
os.makedirs(icon_dir, exist_ok=True)

# Create required icon sizes
sizes = [32, 128, 256]
for size in sizes:
    filename = f'{icon_dir}/{size}x{size}.png'
    with open(filename, 'wb') as f:
        f.write(create_png(size, size))
    print(f'Created {filename}')

# Create 128x128@2x (which is 256x256)
filename = f'{icon_dir}/128x128@2x.png'
with open(filename, 'wb') as f:
    f.write(create_png(256, 256))
print(f'Created {filename}')

# Create icon.png for Linux tray
filename = f'{icon_dir}/icon.png'
with open(filename, 'wb') as f:
    f.write(create_png(128, 128))
print(f'Created {filename}')

print('\nPlaceholder icons created!')
print('For production, use: cargo tauri icon path/to/your-icon.png')
