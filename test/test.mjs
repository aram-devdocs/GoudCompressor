import fetch from "node-fetch";
import fs from "fs";
import path from "path";
import init, { compress, decompress } from "../ts-wrapper/goud_compressor.js";

// ----------------------------------------------
// Helper function: gather all .txt and .json files
// ----------------------------------------------
function gatherFiles(fileOrDirPaths) {
  const results = [];

  // You can adjust which extensions you want to handle
  const validExtensions = [".txt", ".json"];

  function processPath(entry) {
    const fullPath = path.resolve("files", entry);

    // If the file/folder does not exist, skip
    if (!fs.existsSync(fullPath)) {
      console.log(`Skipping: ${entry} (not found)`);
      return;
    }

    const stat = fs.lstatSync(fullPath);

    if (stat.isDirectory()) {
      // Read directory contents
      const directoryFiles = fs.readdirSync(fullPath);

      // If you want **recursive** scanning of sub-directories,
      // you could also check if items are directories here
      // and call processPath again. For a shallow read, do:
      directoryFiles.forEach((f) => {
        const ext = path.extname(f).toLowerCase();
        if (validExtensions.includes(ext)) {
          // Construct the relative path from "files/"
          const childRelativePath = path.join(entry, f);
          results.push(childRelativePath);
        }
      });
    } else {
      // It's a file; make sure the extension is valid
      const ext = path.extname(fullPath).toLowerCase();
      if (validExtensions.includes(ext)) {
        // Keep the path relative to "files/"
        results.push(entry);
      } else {
        console.log(`Skipping: ${entry} (invalid extension)`);
      }
    }
  }

  // Process each item in the incoming array
  fileOrDirPaths.forEach(processPath);
  return results;
}

async function runTests(
  logLevel = "none",
  verbose = false,
  files = "all",
  save = false,
  algorithm = "best"
) {
  // Initialize the WASM module
  const wasmPath = path.resolve("../ts-wrapper/goud_compressor_bg.wasm");
  const wasmBuffer = fs.readFileSync(wasmPath);
  await init(wasmBuffer);

  // Capture debug logs
  let debugLogs = [];
  const originalLog = console.log;
  if (logLevel === "debug") {
    console.log = (...args) => {
      debugLogs.push(args.join(" "));
      originalLog.apply(console, args);
    };
  }

  // Original hard-coded default list
  const allFilesToTest = [
    "test.txt",
    "big_lorem_ipsum.txt",
    "repeated_text.txt",
    "random_text.txt",
    "big_config.json",
    "repeated_data.json",
    // You can include directories here if you like:
    // "test_data_batch_0"  // For example
  ];

  // Build the final list of file(s) or directory(ies)
  const inputFileList = files === "all" ? allFilesToTest : [files];

  // Gather valid files from directories or single-file references
  const finalFilePaths = gatherFiles(inputFileList);

  let totalCompressionTime = 0;
  let totalDecompressionTime = 0;
  let totalCompressionRatio = 0;
  let numFilesTested = 0;
  let allLossless = true;
  let allSmaller = true;

  const results = [];
  const failedFiles = [];

  // Now loop over the fully resolved list of .txt/.json files
  for (const relativePath of finalFilePaths) {
    // Full path to the file
    const filePath = path.resolve("files", relativePath);

    // Safety check
    if (!fs.existsSync(filePath)) {
      console.log(`Skipping: ${relativePath} (file not found)`);
      continue;
    }

    // Read file as UTF-8
    const originalText = fs.readFileSync(filePath, "utf-8");
    const inputArray = new TextEncoder().encode(originalText);
    const fileName = path.basename(filePath);

    console.log(`\n=== Testing file: ${fileName} ===`);
    console.log("Input size:", inputArray.length);

    // Performance logging
    const startTime = performance.now();

    // Compress
    const compressed = compress(inputArray, { logLevel, algorithm });
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

      // Optional: Show diff if mismatch
      const origLines = originalText.split("\n");
      const decompLines = decompressedText.split("\n");
      for (let i = 0; i < origLines.length; i++) {
        if (origLines[i] !== (decompLines[i] || "")) {
          console.log("Line", i, "differs:");
          console.log("Input:", origLines[i]);
          console.log("Output:", decompLines[i]);
          // break; // If you only want to show the first difference
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
      isSmaller,
    });
  }

  // Calculate averages
  const avgCompressionTime = totalCompressionTime / numFilesTested || 0;
  const avgDecompressionTime = totalDecompressionTime / numFilesTested || 0;
  const avgCompressionRatio = totalCompressionRatio / numFilesTested || 0;

  // Identify outliers
  const compressionTimes = results.map((r) => parseFloat(r.compressionTime));
  const decompressionTimes = results.map((r) => parseFloat(r.decompressionTime));
  const compressionRatios = results.map((r) => parseFloat(r.compressionRatio));

  const compressionTimeOutliers = findOutliers(compressionTimes);
  const decompressionTimeOutliers = findOutliers(decompressionTimes);
  const compressionRatioOutliers = findOutliers(compressionRatios);

  // Display after action report
  console.log("\n=== After Action Report ===");
  console.log(`Files tested: ${numFilesTested}`);
  console.log(`Average compression time: ${avgCompressionTime.toFixed(2)} ms`);
  console.log(`Average decompression time: ${avgDecompressionTime.toFixed(2)} ms`);
  console.log(`Average compression ratio: ${avgCompressionRatio.toFixed(2)}%`);
  console.log(
    `Compression time outliers: ${
      compressionTimeOutliers.length > 0 ? compressionTimeOutliers : "None"
    }`
  );
  console.log(
    `Decompression time outliers: ${
      decompressionTimeOutliers.length > 0 ? decompressionTimeOutliers : "None"
    }`
  );
  console.log(
    `Compression ratio outliers: ${
      compressionRatioOutliers.length > 0 ? compressionRatioOutliers : "None"
    }`
  );
  console.log(`All compressions were lossless: ${allLossless}`);
  console.log(`All compressions were smaller than input: ${allSmaller}`);
  if (failedFiles.length > 0) {
    console.log(`Failed files: ${failedFiles.join(", ")}`);
  }

  // Optionally save results
  if (save) {
    const dateTime = new Date().toISOString().replace(/[:.]/g, "-");
    // Create results directory if it doesn't exist
    if (!fs.existsSync("results")) {
      fs.mkdirSync("results");
    }
    const outputPath = path.resolve(`results/results-${dateTime}.json`);
    fs.writeFileSync(outputPath, JSON.stringify(results, null, 2));
    console.log(`Results saved to ${outputPath}`);

    // Save debug logs if available
    if (logLevel === "debug" && debugLogs.length > 0) {
      const debugOutputPath = path.resolve(`results/debug-${dateTime}.log`);
      fs.writeFileSync(debugOutputPath, debugLogs.join("\n"));
      console.log(`Debug logs saved to ${debugOutputPath}`);
    }
  }

  // Restore original console.log
  if (logLevel === "debug") {
    console.log = originalLog;
  }
}

function findOutliers(data) {
  if (data.length < 4) return []; // Not enough data for quartiles
  const sorted = data.slice().sort((a, b) => a - b);
  const q1 = sorted[Math.floor(sorted.length / 4)];
  const q3 = sorted[Math.ceil(sorted.length * (3 / 4))];
  const iqr = q3 - q1;
  const lowerBound = q1 - iqr * 1.5;
  const upperBound = q3 + iqr * 1.5;
  return sorted.filter((x) => x < lowerBound || x > upperBound);
}

const args = process.argv.slice(2);
if (args.includes("--help")) {
  console.log(
    "Usage: node test.mjs [--log <level>] [--verbose] [--files <all|'filename-path'>] [--save] [--algorithm <lz|rle|delta>]"
  );
  console.log("  --log <level>  Set the log level (none, error, info, debug)");
  console.log("  --verbose      Enable detailed performance logging");
  console.log("  --files        Specify files to test (default: all)");
  console.log("  --save         Save the test results to a file");
  console.log("  --algorithm    Specify the compression algorithm to use (default: best)");
  process.exit(0);
}

const logLevel = args.includes("--log") ? args[args.indexOf("--log") + 1] : "none";
const verbose = args.includes("--verbose");
const filesArg = args.includes("--files") ? args[args.indexOf("--files") + 1] : "all";
const save = args.includes("--save"); // Simplified boolean flag check
const algorithm = args.includes("--algorithm")
  ? args[args.indexOf("--algorithm") + 1]
  : "best";

runTests(logLevel, verbose, filesArg, save, algorithm);