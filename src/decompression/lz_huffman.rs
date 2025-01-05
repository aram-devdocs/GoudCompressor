use crate::constants::{LOG_LEVEL_INFO, LOG_LEVEL_PERFORMANCE, WINDOW_SIZE};
use crate::decompression::huff_decode::decode_huffman;
use crate::utils::log_message;

pub fn decompress_lz_huffman(data: &[u8], log_level: &str, verbose: bool) -> Vec<u8> {
    log_message(
        LOG_LEVEL_INFO,
        log_level,
        "Starting LZ+Huffman decompression",
        verbose,
    );

    // 1. Decode the Huffman tree
    //    In our placeholder logic, the tree is just 1 byte (0xFF).
    //    Then the rest is the token stream.
    let _tree_byte = data[0];
    let token_data = &data[1..];

    // 2. Decode tokens from the token stream
    let tokens = decode_huffman(token_data, log_level, verbose);

    // 3. Reconstruct original bytes from tokens
    let mut output = Vec::with_capacity(WINDOW_SIZE);
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

    log_message(
        LOG_LEVEL_PERFORMANCE,
        log_level,
        &format!(
            "LZ+Huffman decompression complete: original_size={}, decompressed_size={}",
            data.len(),
            output.len()
        ),
        verbose,
    );

    output
}
