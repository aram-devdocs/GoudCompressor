use wasm_bindgen::JsValue;
use crate::utils::{log, get_log_level};

pub fn decompress(input: &[u8], options: &JsValue) -> Vec<u8> {
    let log_level = get_log_level(options);
    let mut result = Vec::with_capacity(input.len() * 2);
    let mut i = 0;
    let mut steps = 0;

    while i < input.len() {
        if input[i] == 2 {
            i += 1;
            let l1 = input[i] as u32;
            let l2 = input[i + 1] as u32;
            let l3 = input[i + 2] as u32;
            let l4 = input[i + 3] as u32;
            i += 4;
            let length = l1 | (l2 << 8) | (l3 << 16) | (l4 << 24);
            let end = i + length as usize;
            result.extend_from_slice(&input[i..end]);
            i = end;
        } else if input[i] == 0 {
            i += 1;
            if i < input.len() {
                result.push(input[i]);
                i += 1;
            }
        } else {
            i += 1;
            let length = input[i] as usize;
            i += 1;
            let dist_lo = input[i] as u16;
            let dist_hi = input[i + 1] as u16;
            let distance = (dist_hi << 8) | dist_lo;
            i += 2;

            let start = result.len().saturating_sub(distance as usize);
            for j in 0..length {
                if start + j < result.len() {
                    result.push(result[start + j]);
                } else {
                    result.push(0);
                }
            }
        }

        steps += 1;

        if log_level == "debug" && i % 100 == 0 {
            log(&format!("Decompressing byte {}: {}", i, input[i - 1]));
        }
    }

    if log_level == "info" || log_level == "debug" {
        log(&format!("Decompression complete. Compressed size: {}, Decompressed size: {}", input.len(), result.len()));
    }

    if log_level == "debug" {
        log(&format!("Decompression steps: {}", steps));
    }

    result
}
