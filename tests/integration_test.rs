use scrambler::scrambler::translate_word;

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
