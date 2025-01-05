mod bwt;
mod delta;
mod lz;
mod rle;

use crate::constants::{BWT_FLAG, CHUNKED_FLAG, COMPRESSED_FLAG, DELTA_FLAG, RLE_FLAG};
use crate::shared::compression::CompressionResult;
pub use bwt::compress_bwt;
pub use delta::compress_delta;
pub use lz::compress_lz;
pub use rle::compress_rle;

pub const CHUNK_SIZE: usize = 32 * 1024; // 32KB chunks
const CHAIN_THRESHOLD: f64 = 0.90; // If compression ratio > 90%, try chaining

type CompressionFn = fn(&[u8]) -> Vec<u8>;

#[derive(Debug)]
struct CompressedChunk {
    compressed_data: Vec<u8>,
    methods: Vec<u8>,
}

pub fn compress_chunked(data: &[u8]) -> CompressionResult {
    let chunks: Vec<&[u8]> = data.chunks(CHUNK_SIZE).collect();
    let total_chunks = chunks.len() as u32;
    let mut compressed_chunks = Vec::new();

    // Write total metadata
    compressed_chunks.extend_from_slice(&total_chunks.to_le_bytes());
    compressed_chunks.extend_from_slice(&(data.len() as u32).to_le_bytes());

    for chunk in chunks.iter() {
        let info = compress_chunk(chunk);
        
        // Format: [chunk_size: u32][methods_count: u8][methods...][compressed_data...]
        let chunk_header = (info.compressed_data.len() as u32).to_le_bytes();
        compressed_chunks.extend_from_slice(&chunk_header);
        compressed_chunks.push(info.methods.len() as u8);
        compressed_chunks.extend_from_slice(&info.methods);
        compressed_chunks.extend_from_slice(&info.compressed_data);
    }

    if compressed_chunks.len() < data.len() {
        CompressionResult::Compressed(compressed_chunks, CHUNKED_FLAG)
    } else {
        CompressionResult::Uncompressed(data.to_vec())
    }
}

fn compress_chunk(chunk: &[u8]) -> CompressedChunk {
    let mut methods = Vec::new();
    let mut current_data = chunk.to_vec();

    let attempts: [(CompressionFn, u8); 4] = [
        (compress_lz as CompressionFn, COMPRESSED_FLAG),
        (compress_rle as CompressionFn, RLE_FLAG),
        (compress_delta as CompressionFn, DELTA_FLAG),
        (compress_bwt as CompressionFn, BWT_FLAG),
    ];

    loop {
        let best_attempt = attempts
            .iter()
            .map(|(compress_fn, flag)| {
                let compressed = compress_fn(&current_data);
                (compressed.len(), compressed, *flag)
            })
            .min_by_key(|(size, _, _)| *size)
            .unwrap();

        let ratio = best_attempt.0 as f64 / current_data.len() as f64;

        if ratio > CHAIN_THRESHOLD {
            break;
        }

        methods.push(best_attempt.2);
        current_data = best_attempt.1;

        if methods.len() >= 3 {
            break;
        }
    }

    CompressedChunk {
        compressed_data: current_data,
        methods,
    }
}
