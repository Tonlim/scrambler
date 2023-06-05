use std::cmp::min;
use std::error::Error;

use log::error;

use crate::scrambler::storage;

use super::Glyph;
use super::Translation;

pub fn new_translation(word: &str) -> Result<Translation, Box<dyn Error>> {
    error!("Using dummy alphabet of `abc`. Proper alphabet is not implemented yet.");
    let dummy_alphabet = vec![
        Glyph::new("a".to_owned()),
        Glyph::new("b".to_owned()),
        Glyph::new("c".to_owned()),
    ];
    storage::save_alphabet(&dummy_alphabet)?;

    let alphabet = storage::load_alphabet()?;

    error!("Using dummy translation generation by simply replacing the first three letters with the alphabet. Proper generation is not implemented yet.");
    let mut result = word.to_owned();
    let range = min(result.len(), alphabet.len());
    result.replace_range(
        0..range,
        &alphabet
            .iter()
            .map(|glyph| glyph.symbol.as_str())
            .collect::<Vec<&str>>()
            .join("")[0..range],
    );

    Ok(Translation::new(result))
}
