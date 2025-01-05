use crate::constants::{MAX_MATCH_LEN, WINDOW_SIZE};

pub fn find_longest_match(data: &[u8], i: usize) -> (usize, usize) {
    let window_start = i.saturating_sub(WINDOW_SIZE);

    let mut best_offset = 0;
    let mut best_len = 0;

    for candidate_start in window_start..i {
        let mut length = 0;

        while i + length < data.len()
            && candidate_start + length < i
            && data[candidate_start + length] == data[i + length]
            && length < MAX_MATCH_LEN
        {
            length += 1;
        }

        if length > best_len {
            best_len = length;
            best_offset = i - candidate_start;
            if best_len == MAX_MATCH_LEN {
                break;
            }
        }
    }

    (best_offset, best_len)
}
