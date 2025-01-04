use wasm_bindgen::prelude::*;
use std::collections::HashMap;

const CONTROL_WORD: u8 = 0xFF;
const REPEAT_MARKER: u8 = 0xFE;
const DICT_MARKER: u8 = 0xFD;

#[wasm_bindgen]
pub fn compress(input: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(input.len());
    let mut dictionary: HashMap<Vec<u8>, u8> = HashMap::new();
    let mut dict_counter: u8 = 0;
    let mut word = Vec::new();
    let mut i = 0;

    while i < input.len() {
        // Handle repeated characters
        let current = input[i];
        let mut count = 1;
        while i + count < input.len() && input[i + count] == current {
            count += 1;
        }

        if count > 4 {
            result.push(CONTROL_WORD);
            result.push(REPEAT_MARKER);
            result.push(current);
            result.push(count as u8);
            i += count;
            continue;
        }

        // Collect word
        if input[i].is_ascii_alphabetic() || input[i] == b'_' {
            word.push(input[i]);
        } else {
            // Process collected word
            if word.len() > 3 {
                if let Some(&code) = dictionary.get(&word) {
                    result.push(CONTROL_WORD);
                    result.push(DICT_MARKER);
                    result.push(code);
                } else if dict_counter < 250 {
                    dictionary.insert(word.clone(), dict_counter);
                    result.push(CONTROL_WORD);
                    result.push(DICT_MARKER);
                    result.push(dict_counter);
                    result.extend_from_slice(&word);
                    result.push(0);
                    dict_counter += 1;
                } else {
                    result.extend_from_slice(&word);
                }
            } else {
                result.extend_from_slice(&word);
            }
            word.clear();
            result.push(input[i]);
        }
        i += 1;
    }

    // Process last word if any
    if !word.is_empty() {
        if word.len() > 3 {
            if let Some(&code) = dictionary.get(&word) {
                result.push(CONTROL_WORD);
                result.push(DICT_MARKER);
                result.push(code);
            } else {
                result.extend_from_slice(&word);
            }
        } else {
            result.extend_from_slice(&word);
        }
    }

    result
}

#[wasm_bindgen]
pub fn decompress(input: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(input.len());
    let mut dictionary: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut i = 0;

    while i < input.len() {
        if input[i] == CONTROL_WORD && i + 1 < input.len() {
            match input[i + 1] {
                REPEAT_MARKER => {
                    let chr = input[i + 2];
                    let count = input[i + 3] as usize;
                    result.extend(std::iter::repeat(chr).take(count));
                    i += 4;
                }
                DICT_MARKER => {
                    let code = input[i + 2];
                    if let Some(word) = dictionary.get(&code) {
                        result.extend_from_slice(word);
                        i += 3;
                    } else {
                        i += 3;
                        let mut word = Vec::new();
                        while i < input.len() && input[i] != 0 {
                            word.push(input[i]);
                            i += 1;
                        }
                        dictionary.insert(code, word.clone());
                        result.extend_from_slice(&word);
                        i += 1;
                    }
                }
                _ => {
                    result.push(input[i]);
                    i += 1;
                }
            }
        } else {
            result.push(input[i]);
            i += 1;
        }
    }

    result
}