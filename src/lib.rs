use wasm_bindgen::prelude::*;

// You might tune these constants:
const WINDOW_SIZE: usize = 4096;   // max distance back
const MIN_MATCH_LEN: usize = 4;    // only encode matches >= this length
const MAX_MATCH_LEN: usize = 255;  // simplistic limit for this example

#[wasm_bindgen]
pub fn compress(input: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(input.len() / 2); // guess for capacity
    let mut i = 0;

    while i < input.len() {
        // Look for the best match in the sliding window.
        let (best_offset, best_len) = find_longest_match(input, i);

        // If we found a decent match, emit it as (token=1, length, distance).
        if best_len >= MIN_MATCH_LEN {
            result.push(1); // "1" means "backreference"
            result.push(best_len as u8);
            // Distance is how far back we have to go. E.g., offset=1 means "repeat last byte"
            let dist = (best_offset & 0xFFFF) as u16;
            // store distance as two bytes (little-endian)
            result.push(dist as u8);
            result.push((dist >> 8) as u8);

            i += best_len;
        } else {
            // Otherwise, emit a literal (token=0, then the byte).
            result.push(0);
            result.push(input[i]);
            i += 1;
        }
    }

    result
}

/// Finds the longest match within [i - WINDOW_SIZE, i) that matches forward from `i`.
/// Returns (offset, length).
fn find_longest_match(data: &[u8], i: usize) -> (usize, usize) {
    let window_start = i.saturating_sub(WINDOW_SIZE);

    let mut best_offset = 0;
    let mut best_len = 0;

    // naive search: check each possible start in [window_start..i]
    // In a real implementation, you'd do something more optimal (hash chain, etc.)
    for candidate_start in window_start..i {
        let mut length = 0;
        while i + length < data.len()
            && candidate_start + length < i
            && data[candidate_start + length] == data[i + length]
            && length < MAX_MATCH_LEN
        {
            length += 1;
        }
        if length > best_len {
            best_len = length;
            best_offset = i - candidate_start;
        }
        // an optional small optimization: break early if we find a perfect match
        if best_len == MAX_MATCH_LEN {
            break;
        }
    }

    (best_offset, best_len)
}

#[wasm_bindgen]
pub fn decompress(input: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(input.len() * 2); // rough guess
    let mut i = 0;

    while i < input.len() {
        // Check the token byte.
        if input[i] == 0 {
            // Literal
            i += 1; // consume token byte
            if i < input.len() {
                result.push(input[i]);
                i += 1;
            }
        } else {
            // Backreference
            i += 1; // consume token byte
            // Next byte = length
            let length = input[i] as usize;
            i += 1;
            // Next two bytes = distance (little endian)
            let dist_lo = input[i] as u16;
            let dist_hi = input[i + 1] as u16;
            let distance = (dist_hi << 8) | dist_lo;
            i += 2;

            // Copy `length` bytes from `distance` bytes behind the current write position
            let start = result.len().saturating_sub(distance as usize);
            for j in 0..length {
                if start + j < result.len() {
                    result.push(result[start + j]);
                } else {
                    // If distance is out of bounds, handle carefully
                    // (in well-formed data, ideally shouldn’t happen)
                    result.push(0);
                }
            }
        }
    }

    result
}