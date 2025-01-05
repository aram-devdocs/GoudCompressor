#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Function to display usage instructions
usage() {
  echo "Usage: $0 [--log <level>]"
  echo "  --log <level>  Set the log level (none, error, info, debug)"
  exit 1
}

# Check for help flag
if [[ "$1" == "--help" ]]; then
  usage
fi

# Check for log level argument
LOG_LEVEL="none"
if [[ "$1" == "--log" ]]; then
  if [[ -n "$2" ]]; then
    LOG_LEVEL="$2"
  else
    usage
  fi
fi

# Run the build script
./build.sh

# Navigate to the test directory
cd test

# Run the test script with Node.js
node test.mjs --log "$LOG_LEVEL"
