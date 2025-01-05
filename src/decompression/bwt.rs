pub fn decompress_bwt(input: &[u8]) -> Vec<u8> {
    // Return early if input is too small
    if input.len() <= 4 {
        return input.to_vec();
    }

    // Read original index
    let original_idx = u32::from_le_bytes(input[0..4].try_into().unwrap()) as usize;
    let compressed = &input[4..];

    // Safety check for empty compressed data
    if compressed.is_empty() {
        return input.to_vec();
    }

    // Reverse RLE
    let mut mtf_data = Vec::with_capacity(compressed.len() * 2);
    let mut i = 0;
    while i < compressed.len() - 1 {
        let count = compressed[i] as usize;
        let value = compressed[i + 1];
        
        // Prevent potential overflow
        if mtf_data.len() + count > mtf_data.capacity() {
            mtf_data.reserve(count);
        }
        
        for _ in 0..count {
            mtf_data.push(value);
        }
        i += 2;
    }

    // Safety check for empty MTF data
    if mtf_data.is_empty() {
        return input.to_vec();
    }

    // Reverse Move-To-Front transform
    let mut mtf = (0..=255).collect::<Vec<u8>>();
    let mut bwt_data = Vec::with_capacity(mtf_data.len());
    
    for &pos in &mtf_data {
        let pos_usize = pos as usize;
        if pos_usize >= mtf.len() {
            return input.to_vec(); // Invalid MTF index
        }
        let byte = mtf[pos_usize];
        bwt_data.push(byte);
        mtf.remove(pos_usize);
        mtf.insert(0, byte);
    }

    // Safety check for empty BWT data
    if bwt_data.is_empty() {
        return input.to_vec();
    }

    // Validate original index
    if original_idx >= bwt_data.len() {
        return input.to_vec();
    }

    // Reverse BWT
    let n = bwt_data.len();
    let mut table: Vec<(u8, usize)> = bwt_data.iter().copied().enumerate().map(|(i, b)| (b, i)).collect();
    table.sort_unstable();

    let mut result = Vec::with_capacity(n);
    let mut idx = original_idx;
    
    // Add safety counter to prevent infinite loops
    let mut safety_counter = 0;
    while result.len() < n && safety_counter < n {
        if idx >= table.len() {
            return input.to_vec(); // Invalid index
        }
        let (byte, next_idx) = table[idx];
        result.push(byte);
        idx = next_idx;
        safety_counter += 1;
    }

    // Check if we got all the data
    if result.len() != n {
        return input.to_vec();
    }

    result
}
