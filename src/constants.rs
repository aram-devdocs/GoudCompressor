pub const WINDOW_SIZE: usize = 4096; // 4KB
pub const MIN_MATCH_LEN: usize = 3;   // Reduced from 4 to catch more matches
pub const MAX_MATCH_LEN: usize = 255; // 1 byte for length
pub const UNCOMPRESSED_FLAG: u8 = 0xAA; // 10101010
pub const COMPRESSED_FLAG: u8 = 0xBB; // 10111011
pub const RLE_FLAG: u8 = 0xCC; // 11001100
pub const DELTA_FLAG: u8 = 0xDD; // 11011101
pub const BWT_FLAG: u8 = 0xEE; // 11101110
pub const CHUNKED_FLAG: u8 = 0xFF; // 11111111

pub const ALGO_UNCOMPRESSED: &str = "Uncompressed";
pub const ALGO_LZ_HUFFMAN: &str = "LZ+Huffman";
pub const ALGO_RLE: &str = "RLE";
pub const ALGO_DELTA: &str = "Delta";
pub const ALGO_BWT: &str = "bwt";

pub const LOG_LEVEL_NONE: &str = "none";
pub const LOG_LEVEL_ERROR: &str = "error";
pub const LOG_LEVEL_INFO: &str = "info";
pub const LOG_LEVEL_DEBUG: &str = "debug";
pub const LOG_LEVEL_PERFORMANCE: &str = "performance";

pub const MIN_FILE_SIZE: usize = 64;  // Don't compress files smaller than this