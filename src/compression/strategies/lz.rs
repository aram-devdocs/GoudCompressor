use crate::compression::matcher;
use crate::compression::huffman;

pub fn compress_lz(data: &[u8]) -> Vec<u8> {
    // 1. Generate LZ77 tokens
    let tokens = matcher::find_matches(data);
    
    // 2. Build Huffman tree
    let tree = huffman::build_huffman_tree(&tokens);
    
    // 3. Encode tokens with Huffman
    let mut result = tree.serialize();
    result.extend(huffman::encode_tokens(&tokens, &tree));
    
    result
}
