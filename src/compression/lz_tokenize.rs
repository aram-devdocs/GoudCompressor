use crate::compression::matcher::find_longest_match;
use crate::shared::token::Token;
use crate::constants::MIN_MATCH_LEN;

pub fn lz_tokenize(data: &[u8]) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < data.len() {
        // Try to find a match at position i
        let (offset, length) = find_longest_match(data, i);

        // Only use a match if it meets MIN_MATCH_LEN
        if length >= MIN_MATCH_LEN {
            tokens.push(Token::Match(offset as u16, length as u16));
            i += length;
        } else {
            // Just emit a literal token
            tokens.push(Token::Literal(data[i]));
            i += 1;
        }
    }

    tokens
}
