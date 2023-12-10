use itertools::Itertools;
use log::error;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::time::SystemTime;
use unicode_segmentation::UnicodeSegmentation;

mod generator;
pub mod storage;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Translation {
    pub translation: String,
    pub time_added: SystemTime,
}

impl Translation {
    fn new(translation: String) -> Translation {
        Translation {
            translation,
            time_added: SystemTime::now(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Glyph {
    pub symbol: String,
    pub time_added: SystemTime,
}

impl Glyph {
    fn new(symbol: String) -> Glyph {
        Glyph {
            symbol,
            time_added: SystemTime::now(),
        }
    }
}

pub fn translate_word(word: &str) -> Result<Translation, Box<dyn Error>> {
    match word.split_whitespace().count() {
        0 => Ok(Translation::new("".to_owned())),
        1 => translate_word_impl(word),
        _ => Err(ScramblerError(
            "Error! I can only translate single words. The input \"".to_owned()
                + word
                + "\" is not a single word.",
        )
        .into()),
    }
}

/// Adds a character to the alphabet used to generate new words
///
/// # Arguments
///
/// * `character` - A single character to be added to the alphabet.
///                 A character is defined as a unicode grapheme cluster.
///                 See http://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries
///                 The character must not be whitespace as adding whitespace to the alphabet
///                 break the whole "translated words are of equivalent size" rule.
///
/// This function panics if the input does not contain a single non-whitespace character.
pub fn add_to_alphabet(character: &str) -> Result<(), Box<dyn Error>> {
    let char_count = character.graphemes(true).count();
    if char_count == 0 {
        panic!("Expected a single character. Received no character.");
    }

    if char_count > 1 {
        panic!("Expected a single character. Received multiple characters.")
    }

    if character.trim().is_empty() {
        panic!("Expected a non-whitespace character. Received whitespace.")
    }

    let mut current_alphabet = match storage::load_alphabet() {
        Ok(alphabet) => alphabet,
        Err(error) => {
            error!("{error}");
            Vec::new()
        }
    };
    let glyph = Glyph::new(character.to_owned());

    if !current_alphabet
        .iter()
        .any(|char| char.symbol == glyph.symbol)
    {
        current_alphabet.push(glyph)
    }

    storage::save_alphabet(&current_alphabet)
}

fn translate_word_impl(word: &str) -> Result<Translation, Box<dyn Error>> {
    let mut known_translations = match storage::load_translated_words() {
        Ok(translations) => translations,
        Err(error) => {
            error!("{error}");
            HashMap::new()
        }
    };

    let word = strip_punctuation(word);
    if word.trim().is_empty() {
        return Err(ScramblerError(
            "I cannot translate a string that consists of only whitespace!".to_owned(),
        )
        .into());
    }

    if !known_translations.contains_key(&word) {
        error!("Using dummy block list of `1`. Proper block list is not implemented yet.");
        storage::save_blocked_translations(vec![Translation::new("1".to_owned())])?;

        let blocked_translations = storage::load_blocked_translations()?;

        let mut new_translation = generator::new_translation(&word)?;
        while translation_is_rejected(&new_translation, &blocked_translations, &known_translations)
        {
            new_translation = generator::new_translation(&word)?;
        }

        known_translations.insert(word.to_owned(), new_translation);
    }

    if let Err(error) = storage::save_translated_words(&known_translations) {
        error!("{error}");
    }

    let result = known_translations
        .remove(&word)
        .expect("If the word did not exist, we just inserted it. It should still be there.");
    Ok(result.clone())
}

fn translation_is_rejected(
    new_translation: &Translation,
    blocked_translations: &Vec<Translation>,
    known_translations: &HashMap<String, Translation>,
) -> bool {
    translation_is_blocked(new_translation, blocked_translations)
        || translation_already_exists(new_translation, known_translations)
}

fn translation_is_blocked(
    new_translation: &Translation,
    blocked_translations: &Vec<Translation>,
) -> bool {
    blocked_translations
        .iter()
        .any(|blocked_translation| blocked_translation.translation == new_translation.translation)
}

fn translation_already_exists(
    new_translation: &Translation,
    known_translations: &HashMap<String, Translation>,
) -> bool {
    known_translations
        .iter()
        .map(|(_key, value)| &value.translation)
        .contains(&new_translation.translation)
}

fn strip_punctuation(word: &str) -> String {
    let regex = Regex::new(r"([[:punct:]])").expect("Hardcoded regex must be valid");
    let result = regex.replace_all(word, "");
    result.into_owned()
}

#[derive(Debug)]
struct ScramblerError(String);

impl fmt::Display for ScramblerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ScramblerError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translate_empty() {
        let result = translate_word("");
        assert!(result.is_ok());
        assert_eq!(result.ok().unwrap().translation, "");
    }

    #[test]
    fn cant_translate_two_words() {
        let result = translate_word("word another one");
        assert!(result.is_err());
    }

    #[test]
    fn reject_blocked_translation() {
        let new = Translation::new("foo".to_owned());
        let blocked = vec![Translation::new("foo".to_owned())];
        let known = HashMap::from([("bar".to_owned(), Translation::new("drink".to_owned()))]);
        assert!(translation_is_rejected(&new, &blocked, &known));
    }

    #[test]
    fn reject_known_translation() {
        let new = Translation::new("foo".to_owned());
        let blocked = vec![Translation::new("bar".to_owned())];
        let known = HashMap::from([("hello".to_owned(), Translation::new("foo".to_owned()))]);
        assert!(translation_is_rejected(&new, &blocked, &known));
    }

    #[test]
    fn accept_new_translation() {
        let new = Translation::new("foo".to_owned());
        let blocked = vec![Translation::new("bar".to_owned())];
        let known = HashMap::from([("hello".to_owned(), Translation::new("world".to_owned()))]);
        assert!(!translation_is_rejected(&new, &blocked, &known));
    }

    #[test]
    fn strip_dot() {
        let result = strip_punctuation("a.b");
        assert_eq!(result, "ab")
    }

    #[test]
    fn strip_leading_dot() {
        let result = strip_punctuation(".ab");
        assert_eq!(result, "ab")
    }

    #[test]
    fn strip_trailing_dot() {
        let result = strip_punctuation("ab.");
        assert_eq!(result, "ab")
    }

    #[test]
    fn strip_multiple_dots() {
        let result = strip_punctuation("a.b.c");
        assert_eq!(result, "abc")
    }

    #[test]
    fn strip_multiple_consecutive_dots() {
        let result = strip_punctuation("a..b");
        assert_eq!(result, "ab")
    }

    #[test]
    fn strip_all_dots() {
        let result = strip_punctuation(".a..b.c.");
        assert_eq!(result, "abc")
    }

    #[test]
    fn strip_question_marks() {
        let result = strip_punctuation("a?b");
        assert_eq!(result, "ab")
    }

    #[test]
    fn strip_no_spaces() {
        let result = strip_punctuation("a b.");
        assert_eq!(result, "a b")
    }
}
