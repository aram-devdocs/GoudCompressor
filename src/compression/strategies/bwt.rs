use std::cmp::Ordering;

pub fn compress_bwt(input: &[u8]) -> Vec<u8> {
    // Don't use BWT for small or empty inputs
    if input.len() < 64 {
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
            return input.to_vec(); // Invalid byte value
        }
    }

    // Apply RLE with safety checks
    let mut rle_data = Vec::new();
    if mtf_data.is_empty() {
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
        return input.to_vec();
    }

    transformed.extend(rle_data);
    transformed
}
