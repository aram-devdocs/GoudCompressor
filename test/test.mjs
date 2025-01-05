import fetch from "node-fetch";
import fs from "fs";
import path from "path";
import init, { compress, decompress } from "../ts-wrapper/goud_compressor.js";

async function runTests(logLevel = "none", verbose = false, files = "all", save = false) {
  // Initialize the WASM module
  const wasmPath = path.resolve("../ts-wrapper/goud_compressor_bg.wasm");
  const wasmBuffer = fs.readFileSync(wasmPath);
  await init(wasmBuffer);

  // List of test files we want to run compression on
  const allFilesToTest = [
    "test.txt",
    "big_lorem_ipsum.txt",
    "repeated_text.txt",
    "random_text.txt",
    "big_config.json",
    "repeated_data.json"
  ];

  const filesToTest = files === "all" ? allFilesToTest : [files];

  let totalCompressionTime = 0;
  let totalDecompressionTime = 0;
  let totalCompressionRatio = 0;
  let numFilesTested = 0;
  let allLossless = true;
  let allSmaller = true;

  const results = [];
  const failedFiles = [];

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

    // Performance logging
    const startTime = performance.now();

    // Compress
    const compressed = compress(inputArray, { logLevel });
    const compressTime = performance.now();
    console.log("Compressed size:", compressed.length);

    // Decompress
    const decompressed = decompress(compressed, { logLevel });
    const decompressTime = performance.now();
    console.log("Decompressed size:", decompressed.length);

    // Check if compressed is smaller
    const isSmaller = compressed.length < inputArray.length;
    console.log("Compressed is smaller than input:", isSmaller);
    if (!isSmaller) {
      allSmaller = false;
      failedFiles.push(fileName);
    }

    // Check for lossless
    const decompressedText = new TextDecoder().decode(decompressed);
    const isLossless = decompressedText === originalText;
    console.log("Decompressed is lossless:", isLossless);
    if (!isLossless) {
      allLossless = false;
      failedFiles.push(fileName);
    }

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

    // Performance logging
    const compressionTime = compressTime - startTime;
    const decompressionTime = decompressTime - compressTime;
    totalCompressionTime += compressionTime;
    totalDecompressionTime += decompressionTime;
    totalCompressionRatio += compressionRatio;
    numFilesTested++;

    if (verbose) {
      console.log(`Compression time: ${compressionTime.toFixed(2)} ms`);
      console.log(`Decompression time: ${decompressionTime.toFixed(2)} ms`);
    }

    results.push({
      fileName,
      inputSize: inputArray.length,
      compressedSize: compressed.length,
      decompressedSize: decompressed.length,
      compressionRatio: compressionRatio.toFixed(2),
      compressionTime: compressionTime.toFixed(2),
      decompressionTime: decompressionTime.toFixed(2),
      isLossless,
      isSmaller
    });
  }

  // Calculate averages
  const avgCompressionTime = totalCompressionTime / numFilesTested;
  const avgDecompressionTime = totalDecompressionTime / numFilesTested;
  const avgCompressionRatio = totalCompressionRatio / numFilesTested;

  // Identify outliers
  const compressionTimes = results.map(r => parseFloat(r.compressionTime));
  const decompressionTimes = results.map(r => parseFloat(r.decompressionTime));
  const compressionRatios = results.map(r => parseFloat(r.compressionRatio));

  const compressionTimeOutliers = findOutliers(compressionTimes);
  const decompressionTimeOutliers = findOutliers(decompressionTimes);
  const compressionRatioOutliers = findOutliers(compressionRatios);

  // Display after action report
  console.log("\n=== After Action Report ===");
  console.log(`Files tested: ${numFilesTested}`);
  console.log(`Average compression time: ${avgCompressionTime.toFixed(2)} ms`);
  console.log(`Average decompression time: ${avgDecompressionTime.toFixed(2)} ms`);
  console.log(`Average compression ratio: ${avgCompressionRatio.toFixed(2)}%`);
  console.log(`Compression time outliers: ${compressionTimeOutliers.length > 0 ? compressionTimeOutliers : "None"}`);
  console.log(`Decompression time outliers: ${decompressionTimeOutliers.length > 0 ? decompressionTimeOutliers : "None"}`);
  console.log(`Compression ratio outliers: ${compressionRatioOutliers.length > 0 ? compressionRatioOutliers : "None"}`);
  console.log(`All compressions were lossless: ${allLossless}`);
  console.log(`All compressions were smaller than input: ${allSmaller}`);
  if (failedFiles.length > 0) {
    console.log(`Failed files: ${failedFiles.join(", ")}`);
  }

  if (save) {
    const dateTime = new Date().toISOString().replace(/[:.]/g, "-");
    // create results directory if it doesn't exist
    if (!fs.existsSync("results")) {
      fs.mkdirSync("results");
    }
    const outputPath = path.resolve(`results/results-${dateTime}.json`);
    fs.writeFileSync(outputPath, JSON.stringify(results, null, 2));
    console.log(`Results saved to ${outputPath}`);
  }
}

function findOutliers(data) {
  const sorted = data.slice().sort((a, b) => a - b);
  const q1 = sorted[Math.floor((sorted.length / 4))];
  const q3 = sorted[Math.ceil((sorted.length * (3 / 4)))];
  const iqr = q3 - q1;
  const lowerBound = q1 - (iqr * 1.5);
  const upperBound = q3 + (iqr * 1.5);
  return sorted.filter(x => x < lowerBound || x > upperBound);
}
const args = process.argv.slice(2);
if (args.includes("--help")) {
  console.log("Usage: node test.mjs [--log <level>] [--verbose] [--files <all|'filename-path'>] [--save]");
  console.log("  --log <level>  Set the log level (none, error, info, debug)");
  console.log("  --verbose      Enable detailed performance logging");
  console.log("  --files        Specify files to test (default: all)");
  console.log("  --save         Save the test results to a file");
  process.exit(0);
}

const logLevel = args.includes("--log") ? args[args.indexOf("--log") + 1] : "none";
const verbose = args.includes("--verbose");
const files = args.includes("--files") ? args[args.indexOf("--files") + 1] : "all";
const save = args.includes("--save");  // Simplified boolean flag check

runTests(logLevel, verbose, files, save);
