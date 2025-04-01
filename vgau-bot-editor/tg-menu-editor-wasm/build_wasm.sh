#!/bin/bash
set -e

echo "====== Building WASM package ======"

# Ensure static directory exists
if [ ! -d "static" ]; then
    mkdir -p static
    echo "Created static directory"
fi

# Create output directory in React project
OUTPUT_DIR="../src/assets/wasm"
if [ ! -d "$OUTPUT_DIR" ]; then
    mkdir -p "$OUTPUT_DIR"
    echo "Created output directory: $OUTPUT_DIR"
fi

# Build WASM package
echo "Running wasm-pack build..."
wasm-pack build --target web --out-dir pkg

# Copy files to React project assets
echo "Copying assets to React project..."
cp pkg/*.js "$OUTPUT_DIR/"
cp pkg/*.wasm "$OUTPUT_DIR/"
cp pkg/*.d.ts "$OUTPUT_DIR/" 2>/dev/null || :

# Copy to local pkg directory for testing
echo "Copying assets to local pkg directory..."
cp ../index.html pkg/ 2>/dev/null || :
cp -r static/* pkg/ 2>/dev/null || :

echo "====== Build completed ======"
echo "Files are available in ./pkg/ and $OUTPUT_DIR"
echo "For local testing use: python -m http.server -d pkg"