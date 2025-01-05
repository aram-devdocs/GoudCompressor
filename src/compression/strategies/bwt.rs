use std::cmp::Ordering;
use crate::constants::{LOG_LEVEL_DEBUG, LOG_LEVEL_ERROR, LOG_LEVEL_INFO, LOG_LEVEL_PERFORMANCE};
use crate::utils::log_message;

pub fn compress_bwt(input: &[u8], log_level: &str, verbose: bool) -> Vec<u8> {
    log_message(LOG_LEVEL_INFO, log_level, "Starting BWT compression", verbose);

    // Don't use BWT for small or empty inputs
    if input.len() < 64 {
        log_message(LOG_LEVEL_DEBUG, log_level, "Input too small for BWT compression", verbose);
        return input.to_vec();
    }

    let n = input.len();
    let mut rotations: Vec<usize> = (0..n).collect();
    
    // Sort rotations based on their corresponding suffixes
    rotations.sort_by(|&a, &b| {
        let mut i = 0;
        while i < n {
            let ai = (a + i) % n;
            let bi = (b + i) % n;
            match input[ai].cmp(&input[bi]) {
                Ordering::Equal => i += 1,
                other => return other,
            }
        }
        Ordering::Equal
    });

    let original_idx = rotations.iter().position(|&x| x == 0).unwrap();
    if original_idx >= n {
        log_message(LOG_LEVEL_ERROR, log_level, "Invalid original index in BWT compression", verbose);
        return input.to_vec(); // Invalid index
    }

    let mut transformed = Vec::with_capacity(n + 4);
    transformed.extend_from_slice(&(original_idx as u32).to_le_bytes());
    
    // Create BWT transform
    let mut bwt_data = Vec::with_capacity(n);
    for &rot in &rotations {
        bwt_data.push(input[(rot + n - 1) % n]);
    }

    // Apply Move-To-Front transform
    let mut mtf = (0..=255).collect::<Vec<u8>>();
    let mut mtf_data = Vec::with_capacity(n);
    
    for &byte in &bwt_data {
        if let Some(pos) = mtf.iter().position(|&x| x == byte) {
            mtf_data.push(pos as u8);
            mtf.remove(pos);
            mtf.insert(0, byte);
        } else {
            log_message(LOG_LEVEL_ERROR, log_level, "Invalid byte value in MTF transform", verbose);
            return input.to_vec(); // Invalid byte value
        }
    }

    // Apply RLE with safety checks
    let mut rle_data = Vec::new();
    if mtf_data.is_empty() {
        log_message(LOG_LEVEL_ERROR, log_level, "MTF data is empty", verbose);
        return input.to_vec();
    }

    let mut count = 1u8;
    let mut prev = mtf_data[0];

    for &curr in mtf_data.iter().skip(1) {
        if curr == prev && count < 255 {
            count += 1;
        } else {
            rle_data.push(count);
            rle_data.push(prev);
            count = 1;
            prev = curr;
        }
    }
    rle_data.push(count);
    rle_data.push(prev);

    // If the compressed size is larger than input, return original
    if rle_data.len() + 4 >= input.len() {
        log_message(LOG_LEVEL_DEBUG, log_level, "Compressed size larger than input, returning original", verbose);
        return input.to_vec();
    }

    transformed.extend(rle_data);

    log_message(LOG_LEVEL_PERFORMANCE, log_level, &format!("BWT compression complete: original_size={}, compressed_size={}", input.len(), transformed.len()), verbose);
    transformed
}
