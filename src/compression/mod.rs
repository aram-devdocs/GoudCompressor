pub(crate) mod matcher;
pub(crate) mod huffman;
mod strategies;

use crate::constants::{UNCOMPRESSED_FLAG, MIN_FILE_SIZE};
use crate::utils::{get_log_level, log};
use crate::shared::compression::CompressionResult;
use strategies::try_all_strategies;
use wasm_bindgen::JsValue;

pub fn compress(input: &[u8], options: &JsValue) -> Vec<u8> {
    let log_level = get_log_level(options);
    
    if input.len() < MIN_FILE_SIZE {
        if log_level == "debug" {
            log("File too small, storing uncompressed");
        }
        let mut output = Vec::with_capacity(input.len() + 1);
        output.push(UNCOMPRESSED_FLAG);
        output.extend_from_slice(input);
        return output;
    }

    match try_all_strategies(input) {
        CompressionResult::Compressed(data, flag) => {
            if log_level == "debug" {
                log(&format!("Using compression method: {:02X}", flag));
            }
            let mut output = Vec::with_capacity(data.len() + 1);
            output.push(flag);
            output.extend(data);
            output
        }
        CompressionResult::Uncompressed(data) => {
            if log_level == "debug" {
                log("No effective compression found, storing uncompressed");
            }
            let mut output = Vec::with_capacity(data.len() + 1);
            output.push(UNCOMPRESSED_FLAG);
            output.extend(data);
            output
        }
    }
}

fn is_compressible(sample: &[u8]) -> bool {
    // Simple entropy calculation
    let mut freqs = [0u32; 256];
    for &byte in sample {
        freqs[byte as usize] += 1;
    }
    
    let mut entropy = 0.0;
    let sample_len = sample.len() as f64;
    for &freq in freqs.iter() {
        if freq > 0 {
            let p = freq as f64 / sample_len;
            entropy -= p * p.log2();
        }
    }
    
    // If entropy is high (close to 8), data is likely random
    entropy < 7.0
}
