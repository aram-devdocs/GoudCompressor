// ...existing code...
if (args.includes("--help")) {
  console.log("Usage: node test.mjs [--log <level>] [--verbose] [--files <all|'filename-path'>] [--save] [--algorithm <lz|rle|delta|bwt>]");
  console.log("  --log <level>  Set the log level (none, error, info, debug)");
  console.log("  --verbose      Enable detailed performance logging");
  console.log("  --files        Specify files to test (default: all)");
  console.log("  --save         Save the test results to a file");
  console.log("  --algorithm    Specify the compression algorithm to use (default: best)");
  process.exit(0);
}
// ...existing code...
