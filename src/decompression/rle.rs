pub fn decompress_rle(data: &[u8]) -> Vec<u8> {
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

    result
}
