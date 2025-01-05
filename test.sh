#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Function to display usage instructions
usage() {
    echo "Usage: $0 [--log <level>] [--verbose] [--files <all|'filename-path'>] [--save]"
    echo "  --log <level>  Set the log level (none, error, info, debug)"
    echo "  --verbose      Enable detailed performance logging"
    echo "  --files        Specify files to test (default: all)"
    echo "  --save         Save the test results to a file"
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
node test.mjs --log "$LOG_LEVEL" --verbose "$VERBOSE" --files "$FILES" ${SAVE:+--save}
