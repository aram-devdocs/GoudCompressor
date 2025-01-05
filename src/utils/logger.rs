use wasm_bindgen::prelude::*;
use crate::constants::{LOG_LEVEL_NONE, LOG_LEVEL_ERROR, LOG_LEVEL_INFO, LOG_LEVEL_DEBUG, LOG_LEVEL_PERFORMANCE};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_many(a: &str, b: &str);
}

pub fn log_message(level: &str, current_level: &str, message: &str) {
    let levels = vec![LOG_LEVEL_NONE, LOG_LEVEL_ERROR, LOG_LEVEL_INFO, LOG_LEVEL_DEBUG, LOG_LEVEL_PERFORMANCE];
    let current_index = levels.iter().position(|&l| l == current_level).unwrap_or(0);
    let level_index = levels.iter().position(|&l| l == level).unwrap_or(0);

    if level_index <= current_index {
        log(&format!("[{}] {}", level.to_uppercase(), message));
    }
}
