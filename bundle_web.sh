#!/bin/bash

set -e

echo "Building WebAssembly version for browser..."

# Clean previous builds
rm -rf pkg/
rm -rf dist/web/

# Build with wasm-pack, setting environment variables for web compatibility
export RUSTFLAGS='--cfg getrandom_backend="wasm_js"'
wasm-pack build --target web --out-dir dist/web/pkg --features web

echo "Copying assets to web distribution..."
mkdir -p dist/web/pkg

# Copy the generated WASM files
cp -r pkg/* dist/web/pkg/

# Copy assets
cp -r assets dist/web/

# Copy HTML file
cp index.html dist/web/

# Create a .htaccess file for proper MIME types (helpful for some web servers)
cat > dist/web/.htaccess << EOF
AddType application/wasm .wasm
AddType application/octet-stream .wasm
EOF

echo "Creating ZIP file for itch.io..."
cd dist/web
zip -r ../juice-zero-bugs-given-web.zip .

echo "Web build completed!"
echo "Files created in: dist/web/"
echo "ZIP for itch.io: dist/juice-zero-bugs-given-web.zip"
echo ""
echo "To upload to itch.io:"
echo "1. Go to your game's dashboard"
echo "2. Click 'Edit game'"
echo "3. Go to 'Uploads'"
echo "4. Upload the ZIP file as a new HTML5 build"
echo "5. Set the HTML5 build as the default for browser play"
ls -la
cd ../..
