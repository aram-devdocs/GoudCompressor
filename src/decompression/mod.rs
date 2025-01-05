use crate::constants::{
    ALGO_DELTA, ALGO_LZ_HUFFMAN, ALGO_RLE, ALGO_UNCOMPRESSED, COMPRESSED_FLAG, DELTA_FLAG,
    RLE_FLAG, UNCOMPRESSED_FLAG,
};
mod huff_decode;
mod lz_huffman;
mod rle;
mod delta;
use crate::decompression::lz_huffman::decompress_lz_huffman;
use crate::decompression::rle::decompress_rle;
use crate::decompression::delta::decompress_delta;
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
        _ => {
            if log_level == "debug" {
                log("Unknown compression flag");
            }
            input.to_vec()
        }
    }
}
