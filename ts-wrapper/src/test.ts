import { compress, decompress } from "./index";
import * as fs from "fs";

async function test() {
    const input = fs.readFileSync("../test-files/input.txt");
    console.log("Original:", input.toString());

    const compressed = await compress(input);
    console.log("Compressed:", compressed);

    const decompressed = await decompress(compressed);
    console.log("Decompressed:", decompressed.toString());

    fs.writeFileSync("../test-files/compressed.bin", compressed);
    fs.writeFileSync("../test-files/decompressed.txt", decompressed);
}

test();