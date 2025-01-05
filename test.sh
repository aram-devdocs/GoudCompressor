#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Function to display usage instructions
usage() {
  echo "Usage: $0 [--log <level>] [--verbose]"
  echo "  --log <level>  Set the log level (none, error, info, debug)"
  echo "  --verbose      Enable detailed performance logging"
  exit 1
}

# Check for help flag
if [[ "$1" == "--help" ]]; then
  usage
fi

# Check for log level argument
LOG_LEVEL="none"
VERBOSE="false"
while [[ "$#" -gt 0 ]]; do
  case $1 in
    --log)
      LOG_LEVEL="$2"
      shift
      ;;
    --verbose)
      VERBOSE="true"
      ;;
    *)
      usage
      ;;
  esac
  shift
done

# Run the build script
./build.sh

# Navigate to the test directory
cd test

# Run the test script with Node.js
node test.mjs --log "$LOG_LEVEL" --verbose "$VERBOSE"
