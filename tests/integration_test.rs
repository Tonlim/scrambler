use scrambler::translate::translate_word;

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
