use crate::constants::{
    ALGO_BWT, ALGO_DELTA, ALGO_LZ_HUFFMAN, ALGO_RLE, ALGO_UNCOMPRESSED, BWT_FLAG, CHUNKED_FLAG, COMPRESSED_FLAG, DELTA_FLAG, LOG_LEVEL_DEBUG, LOG_LEVEL_PERFORMANCE, RLE_FLAG, UNCOMPRESSED_FLAG
};
mod bwt;
mod delta;
mod huff_decode;
mod lz_huffman;
mod rle;
use crate::decompression::bwt::decompress_bwt;
use crate::decompression::delta::decompress_delta;
use crate::decompression::lz_huffman::decompress_lz_huffman;
use crate::decompression::rle::decompress_rle;
use crate::utils::{get_log_level, log_message};
use wasm_bindgen::JsValue;

pub fn decompress(input: &[u8], options: &JsValue) -> Vec<u8> {
    let log_level = get_log_level(options);
    let verbose = js_sys::Reflect::get(options, &JsValue::from_str("verbose"))
        .ok()
        .and_then(|val| val.as_bool())
        .unwrap_or(false);

    if input.is_empty() {
        return Vec::new();
    }

    let flag = input[0];
    let data = &input[1..];

    match flag {
        UNCOMPRESSED_FLAG => {
            log_message(
                LOG_LEVEL_DEBUG,
                &log_level,
                &format!("Decompressing: {}", ALGO_UNCOMPRESSED),
                verbose,
            );
            data.to_vec()
        }
        CHUNKED_FLAG => {
            log_message(LOG_LEVEL_DEBUG, &log_level, "Decompressing: Chunked", verbose);
            decompress_chunked(data, &log_level, verbose)
        }
        COMPRESSED_FLAG => {
            log_message(
                LOG_LEVEL_DEBUG,
                &log_level,
                &format!("Decompressing: {}", ALGO_LZ_HUFFMAN),
                verbose,
            );
            decompress_lz_huffman(data, &log_level, verbose)
        }
        RLE_FLAG => {
            log_message(
                LOG_LEVEL_DEBUG,
                &log_level,
                &format!("Decompressing: {}", ALGO_RLE),
                verbose,
            );
            decompress_rle(data, &log_level, verbose)
        }
        DELTA_FLAG => {
            log_message(
                LOG_LEVEL_DEBUG,
                &log_level,
                &format!("Decompressing: {}", ALGO_DELTA),
                verbose,
            );
            decompress_delta(data, &log_level, verbose)
        }
        BWT_FLAG => {
            log_message(
                LOG_LEVEL_DEBUG,
                &log_level,
                &format!("Decompressing: {}", ALGO_BWT),
                verbose,
            );
            decompress_bwt(data, &log_level, verbose)
        }
        _ => {
            log_message(LOG_LEVEL_DEBUG, &log_level, "Unknown compression flag", verbose);
            input.to_vec()
        }
    }
}

pub fn decompress_chunked(data: &[u8], log_level: &str, verbose: bool) -> Vec<u8> {
    if data.len() < 8 {
        return Vec::new();
    }

    // Read metadata
    let total_chunks = u32::from_le_bytes(data[0..4].try_into().unwrap()) as usize;
    let total_size = u32::from_le_bytes(data[4..8].try_into().unwrap()) as usize;

    let mut result = Vec::with_capacity(total_size);
    let mut pos = 8;

    for _ in 0..total_chunks {
        if pos + 4 >= data.len() {
            break;
        }

        // Read chunk header
        let chunk_size = u32::from_le_bytes(data[pos..pos + 4].try_into().unwrap()) as usize;
        pos += 4;

        if pos >= data.len() {
            break;
        }
        let methods_count = data[pos] as usize;
        pos += 1;

        if pos + methods_count >= data.len() {
            break;
        }
        let methods = &data[pos..pos + methods_count];
        pos += methods_count;

        if pos + chunk_size > data.len() {
            break;
        }
        let mut chunk_data = data[pos..pos + chunk_size].to_vec();
        pos += chunk_size;

        // Apply decompression methods in reverse order
        for &method in methods.iter().rev() {
            chunk_data = match method {
                COMPRESSED_FLAG => decompress_lz_huffman(&chunk_data, log_level, verbose),
                RLE_FLAG => decompress_rle(&chunk_data, log_level, verbose),
                DELTA_FLAG => decompress_delta(&chunk_data, log_level, verbose),
                BWT_FLAG => decompress_bwt(&chunk_data, log_level, verbose),
                _ => chunk_data,
            };
        }

        result.extend(chunk_data);
    }

    log_message(
        LOG_LEVEL_PERFORMANCE,
        log_level,
        &format!(
            "Chunked decompression complete: original_size={}, decompressed_size={}",
            data.len(),
            result.len()
        ),
        verbose,
    );

    result
}
