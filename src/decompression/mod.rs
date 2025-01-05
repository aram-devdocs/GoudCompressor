use crate::constants::{COMPRESSED_FLAG, UNCOMPRESSED_FLAG, RLE_FLAG, DELTA_FLAG};
mod huff_decode;
use crate::decompression::huff_decode::decode_huffman;
use crate::utils::{get_log_level, log};
use wasm_bindgen::JsValue;

pub fn decompress(input: &[u8], options: &JsValue) -> Vec<u8> {
    let log_level = get_log_level(options);
    if input.is_empty() {
        return Vec::new();
    }

    let flag = input[0];
    let data = &input[1..];

    match flag {
        UNCOMPRESSED_FLAG => {
            if log_level == "debug" { log("Decompressing: Uncompressed"); }
            data.to_vec()
        }
        COMPRESSED_FLAG => {
            if log_level == "debug" { log("Decompressing: LZ+Huffman"); }
            decompress_lz_huffman(data)
        }
        RLE_FLAG => {
            if log_level == "debug" { log("Decompressing: RLE"); }
            decompress_rle(data)
        }
        DELTA_FLAG => {
            if log_level == "debug" { log("Decompressing: Delta"); }
            decompress_delta(data)
        }
        _ => {
            if log_level == "debug" { log("Unknown compression flag"); }
            input.to_vec()
        }
    }
}

fn decompress_lz_huffman(data: &[u8]) -> Vec<u8> {
    // 1. Decode the Huffman tree
    //    In our placeholder logic, the tree is just 1 byte (0xFF).
    //    Then the rest is the token stream.
    let _tree_byte = data[0];
    let token_data = &data[1..];

    // 2. Decode tokens from the token stream
    let tokens = decode_huffman(token_data);

    // 3. Reconstruct original bytes from tokens
    let mut output = Vec::new();
    for t in tokens {
        match t {
            // Just push the literal byte
            crate::shared::token::Token::Literal(b) => {
                output.push(b);
            }
            // Copy data from existing output
            crate::shared::token::Token::Match(offset, length) => {
                let offset = offset as usize;
                let length = length as usize;
                let start = output.len().saturating_sub(offset);
                for j in 0..length {
                    if start + j < output.len() {
                        output.push(output[start + j]);
                    } else {
                        // Edge case (rarely reached unless offset > current size)
                        output.push(0);
                    }
                }
            }
        }
    }

    output
}

fn decompress_rle(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut i = 0;
    
    while i < data.len() {
        if data[i] == 0xFF && i + 2 < data.len() {
            let count = data[i + 1];
            let value = data[i + 2];
            result.extend(std::iter::repeat(value).take(count as usize));
            i += 3;
        } else {
            result.push(data[i]);
            i += 1;
        }
    }
    
    result
}

fn decompress_delta(data: &[u8]) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::with_capacity(data.len());
    result.push(data[0]);
    
    for &delta in &data[1..] {
        let next = result.last().unwrap().wrapping_add(delta);
        result.push(next);
    }
    
    result
}
