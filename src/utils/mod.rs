mod logger;

use js_sys::Reflect;
pub use logger::log_message;
use wasm_bindgen::JsValue;

pub fn get_log_level(options: &JsValue) -> String {
    Reflect::get(options, &JsValue::from_str("logLevel"))
        .unwrap_or(JsValue::from_str("none"))
        .as_string()
        .unwrap_or("none".to_string())
}
