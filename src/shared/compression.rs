#[derive(Debug)]
pub enum CompressionResult {
    Compressed(Vec<u8>, u8),  // (data, flag)
    Uncompressed(Vec<u8>),
}
