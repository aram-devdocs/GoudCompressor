use crate::compression::huffman;
use crate::compression::matcher;
use crate::constants::{LOG_LEVEL_INFO, LOG_LEVEL_PERFORMANCE};
use crate::utils::log_message;

pub fn compress_lz(data: &[u8], log_level: &str, verbose: bool) -> Vec<u8> {
    log_message(
        LOG_LEVEL_INFO,
        log_level,
        "Starting LZ compression",
        verbose,
    );

    // 1. Generate LZ77 tokens
    let tokens = matcher::find_matches(data, log_level, verbose);

    // 2. Build Huffman tree
    let tree = huffman::build_huffman_tree(&tokens);

    // 3. Encode tokens with Huffman
    let mut result = tree.serialize();
    result.extend(huffman::encode_tokens(&tokens, &tree));

    log_message(
        LOG_LEVEL_PERFORMANCE,
        log_level,
        &format!(
            "LZ compression complete: original_size={}, compressed_size={}",
            data.len(),
            result.len()
        ),
        verbose,
    );

    result
}
