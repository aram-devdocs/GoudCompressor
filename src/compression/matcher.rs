use crate::constants::{LOG_LEVEL_DEBUG, LOG_LEVEL_INFO, LOG_LEVEL_PERFORMANCE, MAX_MATCH_LEN, MIN_MATCH_LEN, WINDOW_SIZE};
use crate::shared::token::Token;
use crate::utils::log_message;
use std::collections::HashMap;

pub fn find_matches(data: &[u8], log_level: &str, verbose: bool) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut i = 0;
    let mut hash_table: HashMap<u32, Vec<usize>> = HashMap::new();

    log_message(LOG_LEVEL_INFO, log_level, "Finding matches in data", verbose);

    while i < data.len() {
        let (offset, length) = find_match(data, i, &mut hash_table, log_level, verbose);
        
        if length >= MIN_MATCH_LEN {
            tokens.push(Token::Match(offset as u16, length as u16));
            log_message(LOG_LEVEL_DEBUG, log_level, &format!("Match found: offset={}, length={}", offset, length), verbose);
            // Skip the matched sequence
            for j in 0..length {
                if i + j < data.len() {
                    update_hash_table(data, i + j, &mut hash_table);
                }
            }
            i += length;
        } else {
            tokens.push(Token::Literal(data[i]));
            log_message(LOG_LEVEL_DEBUG, log_level, &format!("Literal found: {}", data[i]), verbose);
            update_hash_table(data, i, &mut hash_table);
            i += 1;
        }
    }
    tokens
}

fn find_match(data: &[u8], pos: usize, hash_table: &mut HashMap<u32, Vec<usize>>, log_level: &str, verbose: bool) -> (usize, usize) {
    if pos + MIN_MATCH_LEN > data.len() {
        return (0, 0);
    }

    let hash = calc_hash(&data[pos..pos + MIN_MATCH_LEN]);
    let window_start = pos.saturating_sub(WINDOW_SIZE);
    
    if let Some(positions) = hash_table.get(&hash) {
        let mut best_len = 0;
        let mut best_pos = 0;

        for &start in positions.iter().rev() {
            if start < window_start {
                break;
            }

            let mut len = 0;
            while pos + len < data.len() 
                && start + len < pos 
                && data[start + len] == data[pos + len]
                && len < MAX_MATCH_LEN 
            {
                len += 1;
            }

            if len > best_len {
                best_len = len;
                best_pos = start;
                if best_len == MAX_MATCH_LEN {
                    break;
                }
            }
        }

        if best_len >= MIN_MATCH_LEN {
            log_message(LOG_LEVEL_PERFORMANCE, log_level, &format!("Best match found: pos={}, len={}", best_pos, best_len), verbose);
            return (pos - best_pos, best_len);
        }
    }

    (0, 0)
}

#[inline]
fn calc_hash(bytes: &[u8]) -> u32 {
    (bytes[0] as u32) << 16 | (bytes[1] as u32) << 8 | (bytes[2] as u32)
}

fn update_hash_table(data: &[u8], pos: usize, hash_table: &mut HashMap<u32, Vec<usize>>) {
    if pos + MIN_MATCH_LEN <= data.len() {
        let hash = calc_hash(&data[pos..pos + MIN_MATCH_LEN]);
        hash_table.entry(hash).or_default().push(pos);
    }
}
