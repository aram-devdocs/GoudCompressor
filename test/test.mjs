import fetch from "node-fetch";
import fs from "fs";
import path from "path";
import init, { compress, decompress } from "../ts-wrapper/goud_compressor.js";

async function runTests() {
  // Initialize the WASM module
  const wasmPath = path.resolve("../ts-wrapper/goud_compressor_bg.wasm");
  const wasmBuffer = fs.readFileSync(wasmPath);
  await init(wasmBuffer);

  // List of test files we want to run compression on
  const filesToTest = [
    "test.txt",
    "big_lorem_ipsum.txt",
    "repeated_text.txt",
    "random_text.txt",
    "big_config.json",
    "repeated_data.json"
  ];

  for (const fileName of filesToTest) {
    const filePath = path.resolve("files/" + fileName);
    if (!fs.existsSync(filePath)) {
      console.log(`Skipping: ${fileName} (file not found)`);
      continue;
    }

    // Read file as UTF-8
    const originalText = fs.readFileSync(filePath, "utf-8");
    const inputArray = new TextEncoder().encode(originalText);

    console.log(`\n=== Testing file: ${fileName} ===`);
    console.log("Input size:", inputArray.length);

    // Compress
    const compressed = compress(inputArray);
    console.log("Compressed size:", compressed.length);

    // Decompress
    const decompressed = decompress(compressed);
    console.log("Decompressed size:", decompressed.length);

    // Check if compressed is smaller
    console.log(
      "Compressed is smaller than input:",
      compressed.length < inputArray.length
    );

    // Check for lossless
    const decompressedText = new TextDecoder().decode(decompressed);
    const isLossless = decompressedText === originalText;
    console.log("Decompressed is lossless:", isLossless);

    // Optional: Show diff if mismatch
    if (!isLossless) {
      const origLines = originalText.split("\n");
      const decompLines = decompressedText.split("\n");
      for (let i = 0; i < origLines.length; i++) {
        if (origLines[i] !== (decompLines[i] || "")) {
          console.log("Line", i, "differs:");
          console.log("Input:", origLines[i]);
          console.log("Output:", decompLines[i]);
          // Break early or keep showing differences
          // break;
        }
      }
    }

    // Calculate compression ratio
    const compressionRatio =
      ((inputArray.length - compressed.length) / inputArray.length) * 100;
    console.log(
      `Compression ratio: ${compressionRatio.toFixed(
        2
      )}% smaller than the original.`
    );
  }
}

runTests();