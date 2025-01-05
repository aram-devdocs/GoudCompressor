mod rle;
mod delta;
mod lz;
mod bwt;

use crate::shared::compression::CompressionResult;
use crate::constants::{BWT_FLAG, COMPRESSED_FLAG, DELTA_FLAG, RLE_FLAG};
pub use rle::compress_rle;
pub use delta::compress_delta;
pub use lz::compress_lz;
pub use bwt::compress_bwt;

pub fn try_all_strategies(data: &[u8]) -> CompressionResult {
    // Try each compression strategy
    let lz_result = compress_lz(data);
    let rle_result = compress_rle(data);
    let delta_result = compress_delta(data);
    let bwt_result = compress_bwt(data);

    // Compare sizes and pick the best one
    let results = [
        (lz_result.len(), lz_result, COMPRESSED_FLAG),
        (rle_result.len(), rle_result, RLE_FLAG),
        (delta_result.len(), delta_result, DELTA_FLAG),
        (bwt_result.len(), bwt_result, BWT_FLAG),
    ];

    let best = results.iter()
        .min_by_key(|(size, _, _)| size)
        .unwrap();

    if best.0 < data.len() {
        CompressionResult::Compressed(best.1.clone(), best.2)
    } else {
        CompressionResult::Uncompressed(data.to_vec())
    }
}
