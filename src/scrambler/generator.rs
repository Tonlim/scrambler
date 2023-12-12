use std::error::Error;

use itertools::Itertools;
use rand::Rng;
use unicode_segmentation::UnicodeSegmentation;

use crate::scrambler::storage;

use super::Glyph;
use super::Translation;

pub fn new_translation(word: &str) -> Result<Translation, Box<dyn Error>> {
    let alphabet = storage::load_alphabet();

    let original_length = word.graphemes(true).count();
    let mut result = create_random_word(&alphabet, original_length);
    while !is_valid_word(&result) {
        result = create_random_word(&alphabet, original_length)
    }

    Ok(Translation::new(result))
}

fn is_valid_word(word: &str) -> bool {
    let graphemes = word.graphemes(true);
    for (a, b, c) in graphemes.tuple_windows() {
        if a == b && b == c {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::is_valid_word;

    #[test]
    fn triple_char_is_not_valid_word() {
        assert!(!is_valid_word("aaa"))
    }

    #[test]
    fn double_char_is_valid_word() {
        assert!(is_valid_word("aa"))
    }

    #[test]
    fn triple_char_with_other_char_in_between_is_valid_word() {
        assert!(is_valid_word("aabaa"))
    }
}

fn create_random_word(alphabet: &Vec<Glyph>, original_length: usize) -> String {
    let mut random_generator = rand::thread_rng();

    let (min_length, max_length) = calculate_new_length(original_length);
    let length = random_generator.gen_range(min_length..=max_length);

    let mut result = String::with_capacity(length);
    for _ in 0..length {
        let random_glyph = &alphabet[random_generator.gen_range(0..alphabet.len())];
        result.push_str(&random_glyph.symbol);
    }

    result
}

fn calculate_new_length(original_length: usize) -> (usize, usize) {
    let mut min_length = original_length / 2;
    if min_length == 0 {
        min_length = 1;
    }
    let max_length = original_length * 2;

    (min_length, max_length)
}
