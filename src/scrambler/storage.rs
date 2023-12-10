use log::error;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::BufWriter;

use super::Glyph;
use super::Translation;

const DATA_DIRECTORY: &str = "scrambler_data";
const BACKUP_SUFFIX: &str = "_previous";
const EXTENSION: &str = "json";
const TRANSLATED_WORDS_FILENAME: &str = "translated_words";
const ALPHABET_FILENAME: &str = "alphabet";
const BLOCKED_TRANSLATIONS_FILENAME: &str = "blocked_translations";

pub fn load_translated_words() -> Result<HashMap<String, Translation>, Box<dyn Error>> {
    load_from_file(TRANSLATED_WORDS_FILENAME)
}

pub fn save_translated_words(words: &HashMap<String, Translation>) -> Result<(), Box<dyn Error>> {
    let mut sorted_words = BTreeMap::new();
    for word in words {
        sorted_words.insert(word.0, word.1);
    }
    save_to_file(&sorted_words, TRANSLATED_WORDS_FILENAME)
}

pub fn load_alphabet() -> Result<Vec<Glyph>, Box<dyn Error>> {
    load_from_file(ALPHABET_FILENAME)
}

pub fn save_alphabet(alphabet: &Vec<Glyph>) -> Result<(), Box<dyn Error>> {
    let mut sorted_alphabet = alphabet.clone();
    sorted_alphabet.sort_unstable_by(|l, r| l.symbol.cmp(&r.symbol));
    save_to_file(&sorted_alphabet, ALPHABET_FILENAME)
}

pub fn load_blocked_translations() -> Result<Vec<Translation>, Box<dyn Error>> {
    load_from_file(BLOCKED_TRANSLATIONS_FILENAME)
}

pub fn save_blocked_translations(translations: Vec<Translation>) -> Result<(), Box<dyn Error>> {
    let mut sorted_translations = translations.clone();
    sorted_translations.sort_by(|a, b| a.translation.cmp(&b.translation));
    save_to_file(&sorted_translations, BLOCKED_TRANSLATIONS_FILENAME)
}

fn save_to_file<TData>(data: &TData, filename: &str) -> Result<(), Box<dyn Error>>
where
    TData: serde::ser::Serialize,
{
    initialize_directory()?;

    let path = build_path(filename);
    let backup_path = build_backup_path(filename);

    if let Err(error) = std::fs::rename(&path, &backup_path) {
        error!(
            "Failed to move `{path}` to `{backup_path}`. The backup is NOT made! OS error: {error}."
        );
    }

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .map_err(|inner| SaveFileError {
            name: path.clone(),
            source: inner,
        })?;

    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, data).map_err(|inner| SaveFileError {
        name: path,
        source: inner,
    })?;
    Ok(())
}

fn load_from_file<TData>(filename: &str) -> Result<TData, Box<dyn Error>>
where
    TData: for<'de> serde::Deserialize<'de>,
{
    initialize_directory()?;

    let path = build_path(filename);
    match load_from_file_impl(&path) {
        Ok(result) => Ok(result),
        Err(error) => {
            let backup_path = build_backup_path(filename);
            error!("Failed to load data from `{path}`. Falling back to `{backup_path}`. Reason for failure: {error}");
            load_from_file_impl(&backup_path)
        }
    }
}

fn load_from_file_impl<TData>(path: &str) -> Result<TData, Box<dyn Error>>
where
    TData: for<'de> serde::Deserialize<'de>,
{
    let file = File::open(&path).map_err(|inner| LoadFileError {
        name: path.to_owned(),
        source: inner,
    })?;

    let reader = BufReader::new(file);
    let result = serde_json::from_reader(reader).map_err(|inner| LoadFileError {
        name: path.to_owned(),
        source: inner,
    })?;
    Ok(result)
}

fn initialize_directory() -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(DATA_DIRECTORY).map_err(|inner| CreateDirectoryError {
        name: DATA_DIRECTORY,
        source: inner,
    })?;
    Ok(())
}

fn build_path(filename: &str) -> String {
    DATA_DIRECTORY.to_owned() + "/" + filename + "." + EXTENSION
}

fn build_backup_path(filename: &str) -> String {
    DATA_DIRECTORY.to_owned() + "/" + filename + BACKUP_SUFFIX + "." + EXTENSION
}

#[derive(Debug)]
struct CreateDirectoryError {
    name: &'static str,
    source: std::io::Error,
}

impl Display for CreateDirectoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Issue encountered while creating directory '{}': {}",
            self.name, self.source
        )
    }
}

impl Error for CreateDirectoryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(Debug)]
struct LoadFileError<TError: std::error::Error> {
    name: String,
    source: TError,
}

impl<TError: std::error::Error> Display for LoadFileError<TError> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Issue encountered loading JSON from file '{}': {}",
            self.name, self.source
        )
    }
}

impl<TError: std::error::Error + 'static> Error for LoadFileError<TError> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(Debug)]
struct SaveFileError<TError: std::error::Error> {
    name: String,
    source: TError,
}

impl<TError: std::error::Error> Display for SaveFileError<TError> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Issue encountered save JSON to file '{}': {}",
            self.name, self.source
        )
    }
}

impl<TError: std::error::Error + 'static> Error for SaveFileError<TError> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}
