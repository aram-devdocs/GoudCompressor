import { compress, decompress } from '../ts-wrapper/goud_compressor.js';
import fs from 'fs';
import path from 'path';

const args = process.argv.slice(2);
let logLevel = 'none';
let verbose = false;
let files = 'all';
let save = false;
let algorithm = 'best';

for (let i = 0; i < args.length; i++) {
    switch (args[i]) {
        case '--log':
            logLevel = args[++i];
            break;
        case '--verbose':
            verbose = true;
            break;
        case '--files':
            files = args[++i];
            break;
        case '--save':
            save = true;
            break;
        case '--algorithm':
            algorithm = args[++i];
            break;
        case '--help':
            console.log("Usage: node test.mjs [--log <level>] [--verbose] [--files <all|'filename-path'>] [--save] [--algorithm <lz|rle|delta|bwt|best>]");
            console.log("  --log <level>  Set the log level (none, error, info, debug, performance)");
            console.log("  --verbose      Enable detailed performance logging");
            console.log("  --files        Specify files to test (default: all)");
            console.log("  --save         Save the test results to a file");
            console.log("  --algorithm    Specify the compression algorithm to use (default: best)");
            process.exit(0);
    }
}

const options = {
    logLevel,
    verbose,
    algorithm
};

// Function to read file content
const readFile = (filePath) => {
    return fs.readFileSync(filePath);
};

// Function to write file content
const writeFile = (filePath, data) => {
    fs.writeFileSync(filePath, data);
};

// Function to test compression and decompression
const testCompression = (filePath) => {
    const input = readFile(filePath);
    const compressed = compress(input, options);
    const decompressed = decompress(compressed, options);

    console.log(`File: ${filePath}`);
    console.log(`Original size: ${input.length}`);
    console.log(`Compressed size: ${compressed.length}`);
    console.log(`Decompressed size: ${decompressed.length}`);
    console.log(`Lossless: ${Buffer.compare(input, decompressed) === 0}`);

    if (save) {
        writeFile(`${filePath}.compressed`, compressed);
        writeFile(`${filePath}.decompressed`, decompressed);
    }
};

// Test all files or a specific file
if (files === 'all') {
    const testDir = path.resolve('test');
    fs.readdirSync(testDir).forEach(file => {
        if (file.endsWith('.txt')) {
            testCompression(path.join(testDir, file));
        }
    });
} else {
    testCompression(path.resolve(files));
}
