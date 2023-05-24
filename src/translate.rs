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
