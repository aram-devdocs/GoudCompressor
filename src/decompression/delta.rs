use crate::constants::{LOG_LEVEL_INFO, LOG_LEVEL_PERFORMANCE};
use crate::utils::log_message;

pub fn decompress_delta(data: &[u8], log_level: &str, verbose: bool) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }

    log_message(
        LOG_LEVEL_INFO,
        log_level,
        "Starting Delta decompression",
        verbose,
    );

    let mut result = Vec::with_capacity(data.len());
    result.push(data[0]);

    for &delta in &data[1..] {
        let next = result.last().unwrap().wrapping_add(delta);
        result.push(next);
    }

    log_message(
        LOG_LEVEL_PERFORMANCE,
        log_level,
        &format!(
            "Delta decompression complete: original_size={}, decompressed_size={}",
            data.len(),
            result.len()
        ),
        verbose,
    );

    result
}
