use crate::constants::{LOG_LEVEL_INFO, LOG_LEVEL_PERFORMANCE};
use crate::shared::token::Token;
use crate::utils::log_message;
use std::convert::TryInto;

pub fn decode_huffman(data: &[u8], log_level: &str, verbose: bool) -> Vec<Token> {
    log_message(
        LOG_LEVEL_INFO,
        log_level,
        "Starting Huffman decoding",
        verbose,
    );

    let mut tokens = Vec::new();
    let mut i = 0;
    while i < data.len() {
        let tag = data[i];
        i += 1;
        match tag {
            0 => {
                // Literal
                if i < data.len() {
                    tokens.push(Token::Literal(data[i]));
                    i += 1;
                }
            }
            1 => {
                // Match(offset, length)
                if i + 3 < data.len() {
                    let offset = u16::from_le_bytes(data[i..i + 2].try_into().unwrap());
                    let length = u16::from_le_bytes(data[i + 2..i + 4].try_into().unwrap());
                    tokens.push(Token::Match(offset, length));
                    i += 4;
                } else {
                    break;
                }
            }
            _ => {
                // Unknown tag, break or skip
                break;
            }
        }
    }

    log_message(
        LOG_LEVEL_PERFORMANCE,
        log_level,
        &format!(
            "Huffman decoding complete: original_size={}, decoded_size={}",
            data.len(),
            tokens.len()
        ),
        verbose,
    );

    tokens
}
