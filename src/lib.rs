use wasm_bindgen::prelude::*;

// Run-Length Encoding (RLE) compression
#[wasm_bindgen]
pub fn compress(input: &[u8]) -> Vec<u8> {
    let mut compressed = Vec::new();
    let mut i = 0;

    while i < input.len() {
        let byte = input[i];
        let mut count = 1;

        while i + 1 < input.len() && input[i + 1] == byte {
            count += 1;
            i += 1;
        }

        compressed.push(byte);
        compressed.push(count as u8);
        i += 1;
    }

    compressed
}

// Decompression
#[wasm_bindgen]
pub fn decompress(input: &[u8]) -> Vec<u8> {
    let mut decompressed = Vec::new();
    let mut i = 0;

    while i < input.len() {
        let byte = input[i];
        let count = input[i + 1] as usize;

        for _ in 0..count {
            decompressed.push(byte);
        }

        i += 2;
    }

    decompressed
}