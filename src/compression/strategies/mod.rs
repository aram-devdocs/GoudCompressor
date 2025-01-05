mod bwt;
mod delta;
mod lz;
mod rle;

use crate::constants::{
    BWT_FLAG, CHUNKED_FLAG, COMPRESSED_FLAG, DELTA_FLAG, LOG_LEVEL_DEBUG, LOG_LEVEL_INFO,
    LOG_LEVEL_PERFORMANCE, RLE_FLAG,
};
use crate::shared::compression::CompressionResult;
use crate::utils::log_message;
pub use bwt::compress_bwt;
pub use delta::compress_delta;
pub use lz::compress_lz;
pub use rle::compress_rle;

pub const CHUNK_SIZE: usize = 32 * 1024; // 32KB chunks
const CHAIN_THRESHOLD: f64 = 0.90; // If compression ratio > 90%, try chaining
const MAX_METHODS: usize = 3; // Maximum number of compression methods to apply

type CompressionFn = fn(&[u8], &str, bool) -> Vec<u8>;

#[derive(Debug)]
struct CompressedChunk {
    compressed_data: Vec<u8>,
    methods: Vec<u8>,
}

pub fn compress_chunked(data: &[u8], log_level: &str, verbose: bool) -> CompressionResult {
    let chunks: Vec<&[u8]> = data.chunks(CHUNK_SIZE).collect();
    let total_chunks = chunks.len() as u32;
    let mut compressed_chunks = Vec::new();

    log_message(
        LOG_LEVEL_INFO,
        log_level,
        "Starting chunked compression",
        verbose,
    );

    // Write total metadata
    compressed_chunks.extend_from_slice(&total_chunks.to_le_bytes());
    compressed_chunks.extend_from_slice(&(data.len() as u32).to_le_bytes());

    for chunk in chunks.iter() {
        let info = compress_chunk(chunk, log_level, verbose);

        // Format: [chunk_size: u32][methods_count: u8][methods...][compressed_data...]
        let chunk_header = (info.compressed_data.len() as u32).to_le_bytes();
        compressed_chunks.extend_from_slice(&chunk_header);
        compressed_chunks.push(info.methods.len() as u8);
        compressed_chunks.extend_from_slice(&info.methods);
        compressed_chunks.extend_from_slice(&info.compressed_data);
    }

    log_message(
        LOG_LEVEL_PERFORMANCE,
        log_level,
        &format!(
            "Chunked compression complete: original_size={}, compressed_size={}",
            data.len(),
            compressed_chunks.len()
        ),
        verbose,
    );

    if compressed_chunks.len() < data.len() {
        CompressionResult::Compressed(compressed_chunks, CHUNKED_FLAG)
    } else {
        CompressionResult::Uncompressed(data.to_vec())
    }
}

fn compress_chunk(chunk: &[u8], log_level: &str, verbose: bool) -> CompressedChunk {
    let mut methods = Vec::new();
    let mut current_data = chunk.to_vec();

    let attempts: [(CompressionFn, u8); 4] = [
        (compress_lz as CompressionFn, COMPRESSED_FLAG),
        (compress_rle as CompressionFn, RLE_FLAG),
        (compress_delta as CompressionFn, DELTA_FLAG),
        (compress_bwt as CompressionFn, BWT_FLAG),
    ];

    while methods.len() < MAX_METHODS {
        let best_attempt = attempts
            .iter()
            .filter(|(_, flag)| !methods.contains(flag))
            .map(|(compress_fn, flag)| {
                let compressed = compress_fn(&current_data, log_level, verbose);
                (compressed.len(), compressed, *flag)
            })
            .min_by_key(|(size, _, _)| *size)
            .unwrap();

        let ratio = best_attempt.0 as f64 / current_data.len() as f64;

        log_message(
            LOG_LEVEL_DEBUG,
            log_level,
            &format!(
                "Compression attempt: method={:02X}, ratio={:.2}",
                best_attempt.2, ratio
            ),
            verbose,
        );

        if ratio > CHAIN_THRESHOLD {
            break;
        }

        methods.push(best_attempt.2);
        current_data = best_attempt.1;
    }

    CompressedChunk {
        compressed_data: current_data,
        methods,
    }
}
