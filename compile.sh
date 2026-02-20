#!/bin/bash

set -e # Exit on error

# Navigate to each directory and run the specified commands
# This script assumes that you have Python, Ruby, Rust, and the necessary tools installed.

cd data
python3 generate.py
cd ..

echo "⚙️ Compiling Rust project..."
cd rust
cargo build --release
cd ..

echo ""
echo "🔻⚙️ Compiling Ruby Magnus project..."
cd ruby-magnus
rake compile
cd ..

echo ""
echo "🐍⚙️ Compiling Py03 project..."
cd py03
pipenv run pipenv install --dev
pipenv run maturin develop --release
cd ..

echo ""
echo "🌐⚙️ Compiling WASM project..."
cd javascript_and_wasm
cargo install wasm-bindgen-cli --locked
echo "🛠️ Building the project..."
cargo build --release  --target wasm32-unknown-unknown
echo "📦 Generating bindings with wasm-bindgen..."
wasm-bindgen ./target/wasm32-unknown-unknown/release/wasm.wasm --out-dir ./pkg  --target web
echo "✅ WASM build complete."
cd ..
