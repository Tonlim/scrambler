pub mod storage;

pub fn translate_word(word: &str) -> Result<String, String> {
    match word.split_whitespace().count() {
        0 => Ok("".to_owned()),
        1 => Ok("Translation of \"".to_owned() + word + "\"."),
        _ => Err(
            "Error! I can only translate single words. The input \"".to_owned()
                + word
                + "\" is not a single word.",
        ),
    }
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
