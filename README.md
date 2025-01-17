# GoudCompressor

This repository provides a Rust- and WebAssembly-based compression library for text-heavy data like .txt and .json files. The library aims to significantly reduce file size to help minimize data transfer over the network. By providing a Node library, web developers can easily integrate this compression algorithm into their projects, allowing for smaller payloads and faster load times. The compression algorithm is lossless, meaning the original data can be fully reconstructed after decompression. The project is a proof-of-concept and may not be suitable for all use cases.

## WARNING

This project is a proof-of-concept and may not be suitable for all use cases. The compression algorithm is not optimized for speed and may be slow for large inputs. The compression ratio may vary depending on the input data. Please test the compression algorithm with your data to ensure it meets your requirements. The guarantee of lossless compression is based on the current implementation and may not hold for all inputs. Please use caution when integrating this library into your projects.

## Project Structure

- **/src**  
  Contains the Rust source code with the proprietary compression and decompression logic.

  - **/decompression**  
    Contains modules for different decompression algorithms:
    - `mod.rs`: Main decompression module.
    - `huff_decode.rs`: Huffman decoding logic.
    - `lz_huffman.rs`: LZ-Huffman decompression logic.
    - `rle.rs`: Run-Length Encoding decompression logic.
    - `delta.rs`: Delta decompression logic.

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

## Compression Algorithms

GoudCompressor uses a combination of compression strategies to achieve optimal results. The following algorithms are implemented:

1. **LZ-style Sliding Window**  
    Locates repeating substrings by searching within a sliding window and emits a backreference (distance + length) when a repeat is found.

2. **Run-Length Encoding (RLE)**  
    If a straightforward repetition (e.g., the same character repeated many times) is discovered, we apply RLE for efficiency.

3. **Delta Encoding**  
    Encodes the difference between consecutive bytes, which can be effective for certain types of data.

4. **Burrows-Wheeler Transform (BWT)**  
    Performs a reversible transformation that tends to group similar characters together, making the data more compressible.

5. **Huffman Coding**  
    Used in conjunction with LZ-style compression to further reduce the size of the compressed data.

The library automatically selects the best compression strategy based on the input data, but you can also specify a particular algorithm using the `algorithm` option.

## Compression Strategy

GoudCompressor now uses an advanced multi-strategy compression approach:

1. **Chunked Compression**
   - Data is split into 32KB chunks for optimal processing
   - Each chunk is analyzed and compressed independently
   - Enables parallel processing and better memory usage

2. **Adaptive Chain Compression**
   - Multiple compression algorithms can be chained for each chunk
   - Compression chains are limited to 3 algorithms to balance efficiency
   - Each chunk stores its compression method sequence

3. **Available Algorithms**
   - LZ + Huffman: For general-purpose compression
   - RLE: For repeated sequences
   - Delta: For gradually changing values
   - BWT: For text with repeating patterns

4. **Smart Algorithm Selection**
   - Each chunk is analyzed to determine the best compression strategy
   - Compression is only applied if it provides meaningful benefits
   - Compression chains are built progressively until diminishing returns

## Available Compression Algorithms

- `ALGO_UNCOMPRESSED`: Uncompressed
- `ALGO_LZ_HUFFMAN`: LZ+Huffman
- `ALGO_RLE`: RLE
- `ALGO_DELTA`: Delta
- `ALGO_BWT`: BWT (Burrows-Wheeler Transform)

## Usage Instructions

1. Run the build script:  
   ```
   ./build.sh
   ```
   This compiles the Rust code to WebAssembly and generates TypeScript/JavaScript bindings.

2. Navigate to the test directory and run tests:  
   ```
   ./test.sh [--log <level>] [--verbose] [--files <all|'filename-path'>] [--save] [--algorithm <lz|rle|delta>]
   ```
   You will see output showing input size, compressed size, and whether the compression is lossless. The optional parameters are:
   - `--log <level>`: Set the log level (none, error, info, debug).
   - `--verbose`: Enable detailed performance logging.
   - `--files <all|'filename-path'>`: Specify files to test (default: all).
   - `--save`: Save the test results to a file.
   - `--algorithm <lz|rle|delta>`: Specify the compression algorithm to use (default: best).

3. In your own Node.js or web project, import the resulting JavaScript module from /ts-wrapper (e.g., goud_compressor.js). Use the exported functions:
   - `compress(input: Uint8Array, options: { logLevel: string, algorithm: string }) => Uint8Array`
   - `decompress(input: Uint8Array, options: { logLevel: string }) => Uint8Array`

To specify the compression algorithm, use the `algorithm` option in the `options` object:

```javascript
const options = {
    algorithm: ALGO_LZ_HUFFMAN // or ALGO_RLE, ALGO_DELTA, ALGO_UNCOMPRESSED
};
const compressedData = compress(inputData, options);
const decompressedData = decompress(compressedData, options);
```

## License

This project is provided as-is. Consult Cargo.toml and associated crates for licensing details.
