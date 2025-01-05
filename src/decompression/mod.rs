use crate::constants::{
    ALGO_BWT, ALGO_DELTA, ALGO_LZ_HUFFMAN, ALGO_RLE, ALGO_UNCOMPRESSED, COMPRESSED_FLAG, DELTA_FLAG, RLE_FLAG, UNCOMPRESSED_FLAG, BWT_FLAG, CHUNKED_FLAG
};
mod huff_decode;
mod lz_huffman;
mod rle;
mod delta;
mod bwt;
use crate::decompression::lz_huffman::decompress_lz_huffman;
use crate::decompression::rle::decompress_rle;
use crate::decompression::delta::decompress_delta;
use crate::decompression::bwt::decompress_bwt;
use crate::utils::{get_log_level, log};
use wasm_bindgen::JsValue;

pub fn decompress(input: &[u8], options: &JsValue) -> Vec<u8> {
    let log_level = get_log_level(options);
    if input.is_empty() {
        return Vec::new();
    }

    let flag = input[0];
    let data = &input[1..];

    match flag {
        UNCOMPRESSED_FLAG => {
            if log_level == "debug" {
                log(&format!("Decompressing: {}", ALGO_UNCOMPRESSED));
            }
            data.to_vec()
        }
        CHUNKED_FLAG => {
            if log_level == "debug" {
                log("Decompressing: Chunked");
            }
            decompress_chunked(data)
        }
        COMPRESSED_FLAG => {
            if log_level == "debug" {
                log(&format!("Decompressing: {}", ALGO_LZ_HUFFMAN));
            }
            decompress_lz_huffman(data)
        }
        RLE_FLAG => {
            if log_level == "debug" {
                log(&format!("Decompressing: {}", ALGO_RLE));
            }
            decompress_rle(data)
        }
        DELTA_FLAG => {
            if log_level == "debug" {
                log(&format!("Decompressing: {}", ALGO_DELTA));
            }
            decompress_delta(data)
        }
        BWT_FLAG => {
            if log_level == "debug" {
                log(&format!("Decompressing: {}", ALGO_BWT));
            }
            decompress_bwt(data)
        }
        _ => {
            if log_level == "debug" {
                log("Unknown compression flag");
            }
            input.to_vec()
        }
    }
}

pub fn decompress_chunked(data: &[u8]) -> Vec<u8> {
    if data.len() < 8 {
        return Vec::new();
    }

    // Read metadata
    let total_chunks = u32::from_le_bytes(data[0..4].try_into().unwrap()) as usize;
    let total_size = u32::from_le_bytes(data[4..8].try_into().unwrap()) as usize;
    
    let mut result = Vec::with_capacity(total_size);
    let mut pos = 8;

    for _ in 0..total_chunks {
        if pos + 4 >= data.len() { break; }
        
        // Read chunk header
        let chunk_size = u32::from_le_bytes(data[pos..pos+4].try_into().unwrap()) as usize;
        pos += 4;
        
        if pos >= data.len() { break; }
        let methods_count = data[pos] as usize;
        pos += 1;

        if pos + methods_count >= data.len() { break; }
        let methods = &data[pos..pos+methods_count];
        pos += methods_count;

        if pos + chunk_size > data.len() { break; }
        let mut chunk_data = data[pos..pos+chunk_size].to_vec();
        pos += chunk_size;

        // Apply decompression methods in reverse order
        for &method in methods.iter().rev() {
            chunk_data = match method {
                COMPRESSED_FLAG => decompress_lz_huffman(&chunk_data),
                RLE_FLAG => decompress_rle(&chunk_data),
                DELTA_FLAG => decompress_delta(&chunk_data),
                BWT_FLAG => decompress_bwt(&chunk_data),
                _ => chunk_data,
            };
        }

        result.extend(chunk_data);
    }

    result
}
