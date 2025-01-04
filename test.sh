#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# run ./build.sh
./build.sh

# Navigate to the test directory
cd test

# Run the test script with Node.js
node test.mjs
