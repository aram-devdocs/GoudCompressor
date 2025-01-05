pub fn decompress_delta(data: &[u8]) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::with_capacity(data.len());
    result.push(data[0]);

    for &delta in &data[1..] {
        let next = result.last().unwrap().wrapping_add(delta);
        result.push(next);
    }

    result
}
