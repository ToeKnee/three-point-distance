# Three Point Distance

Load a large JSON encoded fileinto memory. The JSON file contains an array of three sets of coordinates. This is to represent an a->b, b->c calculation.
The goal is to calculate the distance between each triplet of points in the array and return the result as the sum of all distances.

## Versions

- Ruby
- Python
- Rust
- JavaScript

### TODO:

- Magnus
- Pyo3 (maturin develop/build --release)
- WASM

### Notes:

- The code aims to be similar accross languages.
- The rust code does not attempt to do anything clever. It doesn't try to use any buffered reading/decoding or SIMD or other optimizations. It is meant to be a direct translation of the python/ruby code.
- The python code uses a list comprehension where it could use a generator expression. This would allow it to stream from disk and not load the entire file into memory - it's much quicker, but not a like for like comparison.

## Usage

```bash
# Generate the data
cd data
python3 generate_data.py
cd ..

cd rust
cargo run --release
cd ..

cd ruby
ruby main.rb
cd ..

cd python
python3 main.py
cd ..

cd javascript
python3 -m http.server
open http://localhost:8000
cd ..
```
