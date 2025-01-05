use crate::constants::{COMPRESSED_FLAG, UNCOMPRESSED_FLAG};
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
    if flag == UNCOMPRESSED_FLAG {
        // Data stored raw, so just return the remainder
        if log_level == "info" || log_level == "debug" {
            log("Detected uncompressed data");
        }
        return input[1..].to_vec();
    }

    if flag == COMPRESSED_FLAG {
        // Huffman + LZ data
        if log_level == "info" || log_level == "debug" {
            log("Detected compressed data, proceeding with Huffman + LZ decompression");
        }

        // 1. Decode the Huffman tree
        //    In our placeholder logic, the tree is just 1 byte (0xFF).
        //    Then the rest is the token stream.
        let _tree_byte = input[1];
        let token_data = &input[2..];

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

        if log_level == "info" || log_level == "debug" {
            log(&format!(
                "Decompression complete. Compressed size: {}, Decompressed size: {}",
                input.len(),
                output.len()
            ));
        }
        return output;
    }

    // If the flag is something else, we can’t decode
    if log_level == "debug" {
        log("Unknown compression flag found; returning input directly");
    }
    input.to_vec()
}
