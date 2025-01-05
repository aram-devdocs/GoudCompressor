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
            // Store literals until we find a run
            let literal_start = i;
            while i < data.len() && (i + 1 >= data.len() || data[i] != data[i + 1] || count < 4) {
                i += 1;
                count = 1;
                while i + count < data.len() && data[i + count] == data[i] && count < 255 {
                    count += 1;
                }
            }
            result.push(0xFE); // Literal marker
            result.push((i - literal_start) as u8); // Length of literals
            result.extend_from_slice(&data[literal_start..i]);
        }
    }

    result
}
