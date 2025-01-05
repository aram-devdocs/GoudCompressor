pub fn compress_rle(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(data.len());
    let mut i = 0;
    
    while i < data.len() {
        let mut count = 1;
        let current = data[i];
        
        while i + count < data.len() && data[i + count] == current && count < 255 {
            count += 1;
        }
        
        if count >= 4 {
            // Format: [marker byte, count, value]
            result.push(0xFF);
            result.push(count as u8);
            result.push(current);
            i += count;
        } else {
            // Store literal
            result.push(current);
            i += 1;
        }
    }
    
    result
}
