pub fn compress_delta(data: &[u8]) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::with_capacity(data.len());
    result.push(data[0]); // Store first byte as-is
    
    // Store differences between consecutive bytes
    for window in data.windows(2) {
        let delta = window[1].wrapping_sub(window[0]);
        result.push(delta);
    }
    
    result
}
