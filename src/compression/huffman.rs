use crate::shared::token::Token;
use std::collections::HashMap;

// Simplified node representation
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum HuffmanNode {
    Leaf(Token),
    Internal(Box<HuffmanNode>, Box<HuffmanNode>),
}

// Simplified tree struct
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HuffmanTree {
    pub root: HuffmanNode,
}

// Build frequencies of each token and create a tree
pub fn build_huffman_tree(tokens: &[Token]) -> HuffmanTree {
    if tokens.is_empty() {
        // Edge case: no tokens
        return HuffmanTree {
            root: HuffmanNode::Leaf(Token::Literal(0)),
        };
    }

    // Count frequencies
    let mut freq_map = HashMap::new();
    for token in tokens {
        *freq_map.entry(token.clone()).or_insert(0) += 1;
    }

    // Build a simple priority queue
    let mut pq: Vec<(usize, HuffmanNode)> = freq_map
        .into_iter()
        .map(|(token, freq)| (freq, HuffmanNode::Leaf(token)))
        .collect();
    pq.sort_by_key(|(freq, _)| *freq);

    // Merge nodes until single tree
    while pq.len() > 1 {
        let (freq1, node1) = pq.remove(0);
        let (freq2, node2) = pq.remove(0);
        let new_node = HuffmanNode::Internal(Box::new(node1), Box::new(node2));
        let new_freq = freq1 + freq2;

        // Insert back, keeping sorted
        let idx = pq
            .binary_search_by_key(&new_freq, |(f, _)| *f)
            .unwrap_or_else(|e| e);
        pq.insert(idx, (new_freq, new_node));
    }

    HuffmanTree {
        root: pq.remove(0).1,
    }
}

// In a real implementation, you'd traverse the Huffman tree to assign bit patterns
// and then write out bits. For brevity, we'll just store the tokens with minimal encoding.
pub fn encode_tokens(tokens: &[Token], _tree: &HuffmanTree) -> Vec<u8> {
    // Pseudo-encoding: just store (tag byte + offset/length or literal).
    // This is obviously *not* a real bit-compressed output.
    let mut output = Vec::new();

    for token in tokens {
        match token {
            Token::Literal(b) => {
                output.push(0); // Tag for literal
                output.push(*b);
            }
            Token::Match(offset, length) => {
                output.push(1); // Tag for match
                // write offset (2 bytes) + length (2 bytes)
                output.extend_from_slice(&offset.to_le_bytes());
                output.extend_from_slice(&length.to_le_bytes());
            }
        }
    }

    output
}

// Serialize the Huffman tree into bytes (placeholder).
// In practice, you'd do a depth-first traversal and record structure + tokens.
impl HuffmanTree {
    pub fn serialize(&self) -> Vec<u8> {
        // Minimal placeholder: store a single byte “0xFF” to pretend
        // we have a tree. A real implementation would store entire node structure.
        vec![0xFF]
    }

    /// A simple traversal method that reads the tree structure,
    /// thereby removing the 'fields never read' warnings.
    #[allow(dead_code)]
    pub fn walk(&self) {
        fn recurse(node: &HuffmanNode) {
            match node {
                HuffmanNode::Leaf(token) => {
                    // Just debug-print the leaf's Token
                    println!("Leaf: {:?}", token);
                }
                HuffmanNode::Internal(left, right) => {
                    // Recursively walk left and right children
                    recurse(left);
                    recurse(right);
                }
            }
        }
        recurse(&self.root);
    }
}