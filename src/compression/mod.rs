mod matcher;

use crate::constants::MIN_MATCH_LEN;
use crate::utils::{get_log_level, log};
use wasm_bindgen::JsValue;

pub fn compress(input: &[u8], options: &JsValue) -> Vec<u8> {
    let log_level = get_log_level(options);
    let mut result = Vec::with_capacity(input.len() / 2);
    let mut i = 0;
    let mut steps = 0;

    while i < input.len() {
        let (best_offset, best_len) = matcher::find_longest_match(input, i);

        if best_len >= MIN_MATCH_LEN {
            result.push(1);
            result.push(best_len as u8);
            let dist = (best_offset & 0xFFFF) as u16;
            result.push(dist as u8);
            result.push((dist >> 8) as u8);

            i += best_len;
        } else {
            result.push(0);
            result.push(input[i]);
            i += 1;
        };

        steps += 1;

        if log_level == "debug" && i % 100 == 0 {
            log(&format!("Compressing byte {}: {}", i, input[i - 1]));
        }
    }

    if result.len() > input.len() {
        result.clear();
        result.push(2);
        let len = input.len() as u32;
        result.push((len & 0xFF) as u8);
        result.push(((len >> 8) & 0xFF) as u8);
        result.push(((len >> 16) & 0xFF) as u8);
        result.push(((len >> 24) & 0xFF) as u8);
        result.extend_from_slice(input);
    }

    if log_level == "info" || log_level == "debug" {
        log(&format!(
            "Compression complete. Original size: {}, Compressed size: {}",
            input.len(),
            result.len()
        ));
    }

    if log_level == "debug" {
        log(&format!("Compression steps: {}", steps));
    }

    result
}
