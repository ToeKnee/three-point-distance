#!/bin/bash

echo ""
echo "🔻 Testing Ruby project..."
cd ruby
rspec distance_spec.rb
cd ..

echo ""
echo "🐍 Testing Python project..."
cd python
python3 distance_test.py
cd ..

echo ""
echo "⚙️ Testing Rust project..."
cd rust
cargo test
cd ..

echo ""
echo "🔻⚙️ Testing Ruby-Magnus project..."
cd ruby-magnus
rspec spec/distance_spec.rb
cd ..


echo ""
echo "🐍 Testing Python Generator project..."
cd python_generator
python3 distance_test.py
cd ..
