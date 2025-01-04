# GoudCompressor

This repository provides a Rust- and WebAssembly-based compression library for text-heavy data like .txt and .json files. The library aims to significantly reduce file size to help minimize data transfer over the network.


## Project Structure

- **/src**  
  Contains the Rust source code with the proprietary compression and decompression logic in lib.rs.

- **/ts-wrapper**  
  Generated TypeScript bindings and the JavaScript glue code (goud_compressor.js) for interacting with the WASM module.

- **/test**  
  Test scripts (test.mjs) and test files (test.txt).  
  Run tests with the provided test.sh script.

- **build.sh**  
  Builds the Rust project with wasm-pack, creating WASM and generating TypeScript bindings in /ts-wrapper.

- **test.sh**  
  Executes the test suite in the /test directory after the build step.

- **Cargo.toml**  
  Rust project configuration and dependencies (e.g., wasm-bindgen).

- **.gitignore**  
  Excludes build artifacts and test node_modules.

## Compression Algorithm

GoudCompressor has evolved from a simple dictionary-based RLE (Run-Length Encoding) scheme into a more powerful LZ-style algorithm (inspired by how 7-zip handles compression). While not a complete LZMA implementation, this approach typically yields higher compression ratios than basic RLE, especially for longer text files.

1. **LZ-style Sliding Window**  
    Locates repeating substrings by searching within a sliding window and emits a backreference (distance + length) when a repeat is found.

2. **Run-Length Encoding (RLE) Fallback**  
    If a straightforward repetition (e.g., the same character repeated many times) is discovered, we apply RLE for efficiency.

3. **Dictionary Construction**  
    A dictionary may still be built for particularly common substrings; these can be referenced using small tokens, reducing output size.

4. **Lossless Decompression**  
    Every step is fully reversible, ensuring the original data can be reconstructed bit-for-bit.

Note: For very large inputs or more advanced compression needs, consider adding entropy coding (e.g., Huffman or range coding) on top of these LZ tokens for further size reduction.

## Usage Instructions

1. Run the build script:  
   ```
   ./build.sh
   ```
   This compiles the Rust code to WebAssembly and generates TypeScript/JavaScript bindings.

2. Navigate to the test directory and run tests:  
   ```
   ./test.sh
   ```
   You will see output showing input size, compressed size, and whether the compression is lossless.

3. In your own Node.js or web project, import the resulting JavaScript module from /ts-wrapper (e.g., goud_compressor.js). Use the exported functions:
   - `compress(input: Uint8Array) => Uint8Array`
   - `decompress(input: Uint8Array) => Uint8Array`

## License

This project is provided as-is. Consult Cargo.toml and associated crates for licensing details.
