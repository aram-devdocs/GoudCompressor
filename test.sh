#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Function to display usage instructions
usage() {
    echo "Usage: $0 [--log <level>] [--verbose] [--files <all|'filename-path'>] [--save] [--algorithm <lz|rle|delta>]"
    echo "  --log <level>  Set the log level (none, error, info, debug)"
    echo "  --verbose      Enable detailed performance logging"
    echo "  --files        Specify files to test (default: all)"
    echo "  --save         Save the test results to a file"
    echo "  --algorithm    Specify the compression algorithm to use (default: best)"
    exit 1
}

# Check for help flag
if [[ "$1" == "--help" ]]; then
    usage
fi

# Initialize parameters
LOG_LEVEL="none"
VERBOSE="false"
FILES="all"
SAVE="false"
ALGORITHM="best"

# Parse arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
    --log)
        LOG_LEVEL="$2"
        shift
        ;;
    --verbose)
        VERBOSE="true"
        ;;
    --files)
        FILES="$2"
        shift
        ;;
    --save)
        SAVE="true"
        ;;
    --algorithm)
        ALGORITHM="$2"
        shift
        ;;
    *)
        usage
        ;;
    esac
    shift
done

# Run the build script
echo "Building the project..."
./build.sh
echo "Build complete."

# Navigate to the test directory
cd test

# Run the test script with Node.js
echo "Starting the test suite..."
VERBOSE_FLAG=""
SAVE_FLAG=""
if [ "$VERBOSE" = "true" ]; then
    VERBOSE_FLAG="--verbose"
fi
if [ "$SAVE" = "true" ]; then
    SAVE_FLAG="--save"
fi
node test.mjs --log "$LOG_LEVEL" $VERBOSE_FLAG --files "$FILES" $SAVE_FLAG --algorithm "$ALGORITHM"
