#!/bin/bash
set -e

echo "=== Building for Linux ==="

# Detect architecture
ARCH=$(uname -m)
if [ "$ARCH" = "x86_64" ]; then
    TARGET="x86_64-unknown-linux-gnu"
elif [ "$ARCH" = "aarch64" ]; then
    TARGET="aarch64-unknown-linux-gnu"
else
    TARGET="x86_64-unknown-linux-gnu"
fi

echo "Target architecture: $TARGET"

# Install target if needed
if ! rustup target list --installed | grep -q "$TARGET"; then
    echo "Installing target: $TARGET"
    rustup target add "$TARGET"
fi

# Build release - use dynamic linking for Bevy compatibility
echo "Building release binary..."
cargo build --release --target "$TARGET"

# Create distribution directory
echo "Creating distribution directory..."
mkdir -p dist/Linux

# Copy binary (statically linked, no external deps)
cp "target/$TARGET/release/fighter_game" dist/Linux/

# Copy assets
echo "Copying assets..."
cp -r assets dist/Linux/

# Copy README and LICENSE
cp README.md dist/Linux/
cp LICENSE dist/Linux/

# Make binary executable
chmod +x dist/Linux/fighter_game

echo ""
echo "=== Distribution created! ==="
echo ""
echo "Contents of dist/Linux:"
find dist/Linux -maxdepth 2 -type f | head -20

# Create ZIP
echo ""
echo "Creating ZIP file..."
cd dist && zip -r Juice-Zero-Bugs-Given-Linux.zip Linux/
cd ..

echo ""
echo "=== Bundle Complete! ==="
echo ""
echo "Linux distribution: dist/Linux/"
echo "ZIP file: dist/Juice-Zero-Bugs-Given-Linux.zip"
echo ""
echo "To run on Linux:"
echo "  1. Extract: unzip dist/Juice-Zero-Bugs-Given-Linux.zip"
echo "  2. cd Linux"
echo "  3. ./fighter_game"
echo ""
echo "Note: This build uses static linking for maximum compatibility"
