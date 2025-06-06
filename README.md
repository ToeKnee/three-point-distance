# Three Point Distance

Load a large JSON encoded fileinto memory. The JSON file contains an array of three sets of coordinates. This is to represent an a->b, b->c calculation.
The goal is to calculate the distance between each triplet of points in the array and return the result as the sum of all distances.

## Versions

- Ruby
- Python
- Rust
- JavaScript
- Magnus less efficient (Move lots of Point objects between Rust and Ruby and back again)
- Magnus more efficient (Move only arrays of floats between Rust and Ruby)
- Python generator (to stream from disk and not load the entire file into memory)

### TODO:

- Pyo3 (maturin develop/build --release)
- WASM

### Notes:

- The code aims to be similar accross languages.
- The rust code does not attempt to do anything clever. It doesn't try to use any buffered reading/decoding or SIMD or other optimizations. It is meant to be a direct translation of the python/ruby code.
- The python code uses a list comprehension where it could use a generator expression. This would allow it to stream from disk and not load the entire file into memory - it's much quicker, but not a like for like comparison.

## Usage

Run `./run.sh` to compile, test and run the code in all languages non-browser languages.

### To run the browser version:

```
cd javascript
python3 -m http.server 8000
cd ..
```

Then open `http://localhost:8000` in your browser.
