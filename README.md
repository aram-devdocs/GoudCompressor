# GoudCompressor

This repository provides a custom dictionary-based RLE (Run-Length Encoding) compression algorithm implemented in Rust and compiled to WebAssembly (WASM) for use in both web and Node.js environments. The goal is to minimize the size of .txt and .json files, potentially reducing the amount of data transferred over the network.

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

1. Combines dictionary-based compression with run-length encoding.  
2. Special control bytes identify repeated characters and dictionary entries.  
3. Designed to potentially reduce size for text-based inputs (e.g., JSON, plain text).  
4. Ensures that decompression fully reverses the process (lossless).

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
