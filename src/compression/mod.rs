pub(crate) mod huffman;
pub(crate) mod matcher;
mod strategies;

use crate::constants::{
    ALGO_BWT, ALGO_DELTA, ALGO_LZ_HUFFMAN, ALGO_RLE, BWT_FLAG, COMPRESSED_FLAG, DELTA_FLAG,
    LOG_LEVEL_DEBUG, LOG_LEVEL_INFO, LOG_LEVEL_PERFORMANCE, MIN_FILE_SIZE, RLE_FLAG,
    UNCOMPRESSED_FLAG,
};
use crate::shared::compression::CompressionResult;
use crate::utils::{get_log_level, log_message};
use strategies::{compress_bwt, compress_chunked, compress_delta, compress_lz, compress_rle};
use wasm_bindgen::JsValue;

pub fn compress(input: &[u8], options: &JsValue) -> Vec<u8> {
    let log_level = get_log_level(options);
    let verbose = js_sys::Reflect::get(options, &JsValue::from_str("verbose"))
        .ok()
        .and_then(|val| val.as_bool())
        .unwrap_or(false);

    let algorithm: String = js_sys::Reflect::get(options, &JsValue::from_str("algorithm"))
        .ok()
        .and_then(|val| val.as_string())
        .unwrap_or_else(|| "best".to_string());

    log_message(LOG_LEVEL_INFO, &log_level, "Starting compression", verbose);

    // Early exit for small files
    if input.len() < MIN_FILE_SIZE {
        log_message(
            LOG_LEVEL_DEBUG,
            &log_level,
            "File too small, storing uncompressed",
            verbose,
        );
        let mut output = Vec::with_capacity(input.len() + 1);
        output.push(UNCOMPRESSED_FLAG);
        output.extend_from_slice(input);
        return output;
    }

    let result = match algorithm.as_str() {
        ALGO_RLE => {
            log_message(
                LOG_LEVEL_PERFORMANCE,
                &log_level,
                "Using RLE compression",
                verbose,
            );
            CompressionResult::Compressed(compress_rle(input, &log_level, verbose), RLE_FLAG)
        }
        ALGO_DELTA => {
            log_message(
                LOG_LEVEL_PERFORMANCE,
                &log_level,
                "Using Delta compression",
                verbose,
            );
            CompressionResult::Compressed(compress_delta(input, &log_level, verbose), DELTA_FLAG)
        }
        ALGO_LZ_HUFFMAN => {
            log_message(
                LOG_LEVEL_PERFORMANCE,
                &log_level,
                "Using LZ+Huffman compression",
                verbose,
            );
            CompressionResult::Compressed(compress_lz(input, &log_level, verbose), COMPRESSED_FLAG)
        }
        ALGO_BWT => {
            log_message(
                LOG_LEVEL_PERFORMANCE,
                &log_level,
                "Using BWT compression",
                verbose,
            );
            CompressionResult::Compressed(compress_bwt(input, &log_level, verbose), BWT_FLAG)
        }
        _ => {
            log_message(
                LOG_LEVEL_PERFORMANCE,
                &log_level,
                "Using chunked compression",
                verbose,
            );
            compress_chunked(input, &log_level, verbose)
        }
    };

    match result {
        CompressionResult::Compressed(data, flag) => {
            log_message(
                LOG_LEVEL_DEBUG,
                &log_level,
                &format!("Compression successful, method: {:02X}", flag),
                verbose,
            );
            let mut output = Vec::with_capacity(data.len() + 1);
            output.push(flag);
            output.extend(data);
            output
        }
        CompressionResult::Uncompressed(data) => {
            log_message(
                LOG_LEVEL_DEBUG,
                &log_level,
                "No effective compression found, storing uncompressed",
                verbose,
            );
            let mut output = Vec::with_capacity(data.len() + 1);
            output.push(UNCOMPRESSED_FLAG);
            output.extend(data);
            output
        }
    }
}

#[allow(dead_code)] // Keep this function for future use
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
