#!/bin/bash

set -e

echo "Building release binary..."
cargo build --release --no-default-features --features bevy/dynamic_linking

echo "Creating distribution directory..."
mkdir -p dist/macOS

echo "Copying binary..."
cp target/release/fighter_game dist/macOS/

echo "Copying assets..."
cp -r assets dist/macOS/

echo "Checking dynamic library dependencies..."
otool -L dist/macOS/fighter_game

echo "Finding and copying required dylibs..."
# Find the bevy dylib
if [ -f "target/release/deps/libbevy_dylib-*.dylib" ]; then
    BEVY_DYLIB=$(ls target/release/deps/libbevy_dylib-*.dylib | head -1)
    echo "Copying Bevy dylib: $BEVY_DYLIB"
    cp "$BEVY_DYLIB" dist/macOS/
    BEVY_DYLIB_NAME=$(basename "$BEVY_DYLIB")
fi

# Find the std dylib (this is the problematic one)
if [ -f "target/release/deps/libstd-*.dylib" ]; then
    STD_DYLIB=$(ls target/release/deps/libstd-*.dylib | head -1)
    echo "Copying std dylib: $STD_DYLIB"
    cp "$STD_DYLIB" dist/macOS/
    STD_DYLIB_NAME=$(basename "$STD_DYLIB")
fi

echo "Fixing library paths..."
if [ -n "$BEVY_DYLIB_NAME" ]; then
    install_name_tool -change "$BEVY_DYLIB" "@executable_path/$BEVY_DYLIB_NAME" dist/macOS/fighter_game
fi

if [ -n "$STD_DYLIB_NAME" ]; then
    install_name_tool -change "@rpath/$STD_DYLIB_NAME" "@executable_path/$STD_DYLIB_NAME" dist/macOS/fighter_game
fi

echo "Verifying fixed dependencies..."
otool -L dist/macOS/fighter_game

echo "Testing the binary..."
cd dist/macOS
./fighter_game --version || echo "Version check failed, but continuing..."
cd ../..

echo "Creating ZIP file..."
cd dist
zip -r ../Juice-Zero-Bugs-Given.zip .

echo "Bundle created successfully!"
echo "ZIP file: Juice-Zero-Bugs-Given.zip"
echo "Contents:"
unzip -l ../Juice-Zero-Bugs-Given.zip | tail -10
