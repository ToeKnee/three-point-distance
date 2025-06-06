#!/bin/bash

# Navigate to each directory and run the specified commands
# This script assumes that you have Python, Ruby, Rust, and the necessary tools installed.
echo "⚙️ Compiling Rust project..."
cd rust
cargo build --release
cd ..

echo ""
echo "🔻⚙️ Compiling Ruby Magnus project..."
cd ruby-magnus
rake compile
cd ..
