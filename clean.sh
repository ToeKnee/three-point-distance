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
