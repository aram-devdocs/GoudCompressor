use crate::constants::{LOG_LEVEL_INFO, LOG_LEVEL_PERFORMANCE};
use crate::utils::log_message;

pub fn decompress_rle(data: &[u8], log_level: &str, verbose: bool) -> Vec<u8> {
    log_message(
        LOG_LEVEL_INFO,
        log_level,
        "Starting RLE decompression",
        verbose,
    );

    let mut result = Vec::with_capacity(data.len());
    let mut i = 0;

    while i < data.len() {
        if data[i] == 0xFF && i + 2 < data.len() {
            let count = data[i + 1];
            let value = data[i + 2];
            result.extend(std::iter::repeat(value).take(count as usize));
            i += 3;
        } else {
            result.push(data[i]);
            i += 1;
        }
    }

    log_message(
        LOG_LEVEL_PERFORMANCE,
        log_level,
        &format!(
            "RLE decompression complete: original_size={}, decompressed_size={}",
            data.len(),
            result.len()
        ),
        verbose,
    );

    result
}
