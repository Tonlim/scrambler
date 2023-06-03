use log::error;
use std::cmp::min;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

mod storage;

pub fn translate_word(word: &str) -> Result<String, Box<dyn Error>> {
    match word.split_whitespace().count() {
        0 => Ok("".to_owned()),
        1 => translate_word_impl(word),
        _ => Err(ScramblerError(
            "Error! I can only translate single words. The input \"".to_owned()
                + word
                + "\" is not a single word.",
        )
        .into()),
    }
}

fn translate_word_impl(word: &str) -> Result<String, Box<dyn Error>> {
    let mut known_translations = match storage::load_translated_words() {
        Ok(translations) => translations,
        Err(error) => {
            error!("{error}");
            HashMap::new()
        }
    };

    if !known_translations.contains_key(word) {
        known_translations.insert(word.to_owned(), generate_new_translation(word)?);
    }

    if let Err(error) = storage::save_translated_words(&known_translations) {
        error!("{error}");
    }

    let result = known_translations
        .remove(word)
        .expect("If the word did not exist, we just inserted it. It should still be there.");
    Ok(result.clone())
}

fn generate_new_translation(word: &str) -> Result<String, Box<dyn Error>> {
    error!("Using dummy alphabet of `abc`. Proper alphabet is not implemented yet.");
    let dummy_alphabet = vec!["a".to_owned(), "b".to_owned(), "c".to_owned()];
    storage::save_alphabet(&dummy_alphabet)?;

    let alphabet = storage::load_alphabet()?;

    error!("Using dummy translation generation by simply replacing the first three letters with the alphabet. Proper generation is not implemented yet.");
    let mut result = word.to_owned();
    let range = min(result.len(), alphabet.len());
    result.replace_range(0..range, &alphabet.join("")[0..range]);

    Ok(result)
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
    use super::translate_word;

    #[test]
    fn translate_single_word() {
        let result = translate_word("word");
        assert!(result.is_ok());
        assert_eq!(result.ok().unwrap(), "Translation of \"word\".");
    }

    #[test]
    fn translate_empty() {
        let result = translate_word("");
        assert!(result.is_ok());
        assert_eq!(result.ok().unwrap(), "");
    }

    #[test]
    fn cant_translate_two_words() {
        let result = translate_word("word another one");
        assert!(result.is_err());
    }
}
