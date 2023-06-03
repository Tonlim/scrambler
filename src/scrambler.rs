use log::error;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

pub mod storage;

#[derive(Debug)]
pub struct ScramblerError(String);

impl fmt::Display for ScramblerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ScramblerError {}

pub async fn initialize() -> Result<(), Box<dyn Error>> {
    storage::initialize_directory()
}

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
        // TODO: generate a new word
        known_translations.insert(
            word.to_owned(),
            "Translation of \"".to_owned()
                + word
                + "\"."
                + &std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    .to_string(),
        );
    }

    if let Err(error) = storage::save_translated_words(&known_translations) {
        error!("{error}");
    }

    let result = known_translations
        .remove(word)
        .expect("If the word did not exist, we just inserted it. It should still be there.");
    Ok(result.clone())
}

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
