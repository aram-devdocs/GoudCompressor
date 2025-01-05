use wasm_bindgen::JsValue;
mod huffman;
mod lz_tokenize;
mod matcher;
use crate::compression::huffman::{build_huffman_tree, encode_tokens};
use crate::compression::lz_tokenize::lz_tokenize;
use crate::constants::{COMPRESSED_FLAG, UNCOMPRESSED_FLAG};
use crate::utils::{get_log_level, log};

pub fn compress(input: &[u8], options: &JsValue) -> Vec<u8> {
    let log_level = get_log_level(options);

    // 1. Convert input bytes into tokens via LZ logic
    let tokens = lz_tokenize(input);

    // 2. Build a Huffman tree for the tokens
    let tree = build_huffman_tree(&tokens);

    // 3. Encode the tokens using the Huffman tree
    let encoded_tokens = encode_tokens(&tokens, &tree);

    // 4. Construct final compressed buffer
    //    - We'll store the Huffman tree, then the encoded bits
    let mut compressed_data = tree.serialize(); // <-- Tree for decompression
    compressed_data.extend_from_slice(&encoded_tokens); // <-- Encoded token stream

    // 5. Check if compressed is actually smaller
    //    - +1 for the “compressed or not” flag
    if compressed_data.len() + 1 >= input.len() {
        // Fallback: store data uncompressed
        let mut result = Vec::with_capacity(input.len() + 1);
        result.push(UNCOMPRESSED_FLAG);
        result.extend_from_slice(input);
        if log_level == "info" || log_level == "debug" {
            log("Compressed result was bigger; storing uncompressed data");
        }
        result
    } else {
        // Store data compressed
        let mut result = Vec::with_capacity(compressed_data.len() + 1);
        result.push(COMPRESSED_FLAG);
        result.extend_from_slice(&compressed_data);
        if log_level == "info" || log_level == "debug" {
            log("Data successfully compressed with Huffman + LZ");
        }
        result
    }
}
