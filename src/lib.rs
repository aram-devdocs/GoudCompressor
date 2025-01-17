mod compression;
mod constants;
mod decompression;
mod shared;
mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub fn compress(input: &[u8], options: &JsValue) -> Vec<u8> {
    compression::compress(input, options)
}

#[wasm_bindgen]
pub fn decompress(input: &[u8], options: &JsValue) -> Vec<u8> {
    decompression::decompress(input, options)
}
