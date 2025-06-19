# Three Point Distance

Load a large JSON encoded fileinto memory. The JSON file contains an array of three sets of coordinates. This is to represent an a->b, b->c calculation.
The goal is to calculate the distance between each triplet of points in the array and return the result as the sum of all distances.

## Versions

- Ruby
- Python
- Rust
- JavaScript
- Magnus less efficient
  This has the minimal changes to the Ruby code to make it work with the Magnus library.
  However, it moves lots of Point objects between Rust and Ruby and back again.
- Magnus more efficient
  This version moves the object creation to Rust and only moves arrays of floats between Rust and Ruby.
- Py03 less efficient
  This has the minimal changes to the Python code to make it work with the Py03 library.
  However, it moves lots of Point objects between Rust and Python and back again.
- Py03 more efficient
  This version moves the object creation to Rust and only moves arrays of floats between Rust and Python.
- Python generator
  Uses a generator expression to stream the Points instead of creating the entire list in memory.
- WASM (browser)
  This is a JavaScript version that uses WebAssembly to run the Rust code in the browser.

### Notes:

- The code aims to be similar accross languages.
- The rust code does not attempt to do anything clever. It doesn't try to use any buffered reading/decoding or SIMD or other optimizations. It is meant to be a direct translation of the python/ruby code.
- The python code uses a list comprehension where it could use a generator expression. This would allow it to stream from disk and not load the entire file into memory - it's much quicker, but not a like for like comparison.

## Usage

Run `./run.sh` to compile, test and run the code in all languages non-browser languages.

### To run the browser version:

```
cd javascript_and_wasm
wasm-pack build --target web --release
python3 -m http.server 8000
cd ..
```

Then open `http://localhost:8000` in your browser.

## Results

| Language    | Notes               | JSON Parse    | Calculate    | Total         | Memory |
| ----------- | ------------------- | ------------- | ------------ | ------------- | ------ |
| Ruby        | Plain old Ruby      | 22.362383462s | 6.184880194s | 28.547263656s | 4.5g   |
| Python      | Plain old Python    | 33.335266s    | 5.621269s    | 38.956535s    | 9.7g   |
| Rust        | Plain old Rust      | 2.471924s     | 0.758905s    | 3.230829s     | 0.994g |
| Ruby Magnus | Less efficient      | 50.737945359s | 3.328673311s | 54.06661867s  | 5.9g   |
| Ruby Magnus | More efficient      | 16.80033739s  | 2.389948383s | 19.190285773s | 4.8g   |
| Python Py03 | Less efficient      | 19.80117s     | 1.44656s     | 21.24773s     | 9.9g   |
| Python Py03 | More efficient      | 14.726s       | 1.793427s    | 16.519427s    | 7.6g   |
| Python      | Generator           | 11.970741s    | 9.584922s    | 21.555663s    | 6.7g   |
| JavaScript  | Plain old JS 10x    | 9.021s        | 1.401s       | 10.422s       | N/A    |
| WASM        | WASM in browser 10x | 7.018000s     | 1.153000s    | 8.171s        | N/A    |

_Note: The JavaScript and WASM versions are not directly comparable to the others as they run in a browser environment and do not have the same memory usage metrics. They are also limited on the amount of data they can process due to browser memory limits. As they load 10% of the data of the other tests, we run each section 10 times to get us in the ballpark of one run for the full data set._
_Note: The Ruby Magnus version uses a RefCel to enable mutability of the Point struct as a Ruby object. This has an overhead, but is required._

## Thoughts

- The Rust code is significantly faster than the Ruby and Python versions, both in parsing and calculating distances.
- Doing a naive translation of the Ruby code to Rust is not the most efficient way to write Rust code, but it is a good exercise in understanding how to translate concepts between languages.
- Passing less data between languages (like in the more efficient Magnus and Py03 versions) results in better performance.
- It was easier to develop the Py03 version than the Magnus version.
- The WASM version was easy to implement. The changes only really involved adding a few annotations and changing the Error return type.

## Links

- [VIDEO](https://www.youtube.com/watch?app=desktop&v=Zs6Uer3VAyQ) Shipping Rust to Python, Typescript, and Ruby — by Sam Lijin — Seattle Rust User Group, April 2025
  I found this video after I had already written the code, but it has some good insights about how to ship Rust with Python, Ruby and JavaScript.
