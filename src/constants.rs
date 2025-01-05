pub const WINDOW_SIZE: usize = 4096;
pub const MIN_MATCH_LEN: usize = 3;   // Reduced from 4 to catch more matches
pub const MAX_MATCH_LEN: usize = 255;
pub const UNCOMPRESSED_FLAG: u8 = 0xAA;
pub const COMPRESSED_FLAG: u8 = 0xCC;
pub const RLE_FLAG: u8 = 0xBB;
pub const DELTA_FLAG: u8 = 0xDD;

pub const MIN_FILE_SIZE: usize = 64;  // Don't compress files smaller than this