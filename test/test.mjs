import fetch from "node-fetch";
import fs from "fs";
import path from "path";
import init, { compress, decompress } from "../ts-wrapper/goud_compressor.js";

async function runTests() {
  const wasmPath = path.resolve("../ts-wrapper/goud_compressor_bg.wasm");
  const wasmBuffer = fs.readFileSync(wasmPath);
  await init(wasmBuffer);

  const textPath = path.resolve("./test.txt");
  const text = fs.readFileSync(textPath, "utf-8");
  const input = new TextEncoder().encode(text);
  console.log("Input size:", input.length);

  const compressed = compress(input);
  console.log("Compressed size:", compressed.length);

  const decompressed = decompress(compressed);
  console.log("Decompressed size:", decompressed.length);

  console.log(
    "Compressed is smaller than input:",
    compressed.length < input.length
  );

  console.log(
    "Decompressed is lossless:",
    input.length === decompressed.length
  );

  const output = new TextDecoder().decode(decompressed);
  console.log("Decompressed matches input:", text === output);
}

runTests();
