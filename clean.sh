#!/bin/bash

# Navigate to each directory and run the specified commands
# This script assumes that you have Python, Ruby, Rust, and the necessary tools installed.
echo "⚙️ Cleaning Rust project..."
cd rust
cargo clean
cd ..

echo ""
echo "🔻⚙️ Cleaning Ruby Magnus project..."
cd ruby-magnus
rake clobber
cd ..

echo ""
echo "🐍⚙️ Cleaning Py03 project..."
cd py03
cargo clean
rm -rf target
cd ..


echo ""
echo "🌐⚙️ Cleaning WASM project..."
cd javascript_and_wasm
cargo clean
rm -rf pkg
cd ..
