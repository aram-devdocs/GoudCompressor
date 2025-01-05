use crate::constants::{LOG_LEVEL_DEBUG, LOG_LEVEL_INFO, LOG_LEVEL_PERFORMANCE};
use crate::utils::log_message;

pub fn compress_rle(data: &[u8], log_level: &str, verbose: bool) -> Vec<u8> {
    let mut result = Vec::with_capacity(data.len());
    let mut i = 0;

    log_message(LOG_LEVEL_INFO, log_level, "Starting RLE compression", verbose);

    while i < data.len() {
        let mut count = 1;
        let current = data[i];

        while i + count < data.len() && data[i + count] == current && count < 255 {
            count += 1;
        }

        if count >= 4 {
            log_message(LOG_LEVEL_DEBUG, log_level, &format!("RLE match found: value={}, count={}", current, count), verbose);
            // Format: [marker byte, count, value]
            result.push(0xFF);
            result.push(count as u8);
            result.push(current);
            i += count;
        } else {
            // Store literals until we find a run
            let literal_start = i;
            while i < data.len() && (i + 1 >= data.len() || data[i] != data[i + 1] || count < 4) {
                i += 1;
                count = 1;
                while i + count < data.len() && data[i + count] == data[i] && count < 4 {
                    count += 1;
                }
            }
            let literal_length = i - literal_start;
            if literal_length > 0 {
                log_message(LOG_LEVEL_DEBUG, log_level, &format!("Literal sequence found: length={}", literal_length), verbose);
                result.push(0xFE); // Literal marker
                result.push(literal_length as u8); // Length of literals
                result.extend_from_slice(&data[literal_start..i]);
            }
        }
    }

    log_message(LOG_LEVEL_PERFORMANCE, log_level, &format!("RLE compression complete: original_size={}, compressed_size={}", data.len(), result.len()), verbose);
    result
}
