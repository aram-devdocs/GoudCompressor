use crate::constants::{LOG_LEVEL_INFO, LOG_LEVEL_PERFORMANCE};
use crate::utils::log_message;

pub fn compress_delta(data: &[u8], log_level: &str, verbose: bool) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }

    log_message(
        LOG_LEVEL_INFO,
        log_level,
        "Starting Delta compression",
        verbose,
    );

    let mut result = Vec::with_capacity(data.len());
    result.push(data[0]); // Store first byte as-is

    // Store differences between consecutive bytes
    for window in data.windows(2) {
        let delta = window[1].wrapping_sub(window[0]);
        result.push(delta);
    }

    log_message(
        LOG_LEVEL_PERFORMANCE,
        log_level,
        &format!(
            "Delta compression complete: original_size={}, compressed_size={}",
            data.len(),
            result.len()
        ),
        verbose,
    );

    result
}
