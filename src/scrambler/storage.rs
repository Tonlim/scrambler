use const_format::concatcp;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::BufWriter;

const DATA_DIRECTORY: &str = "scrambler_data";

const TRANSLATED_WORDS_FILENAME: &str = "translated_words.json";
const TRANSLATED_WORDS_PATH: &str = concatcp!(DATA_DIRECTORY, "/", TRANSLATED_WORDS_FILENAME);

const ALPHABET_FILENAME: &str = "alphabet.json";
const ALPHABET_PATH: &str = concatcp!(DATA_DIRECTORY, "/", ALPHABET_FILENAME);

pub fn load_translated_words() -> Result<HashMap<String, String>, Box<dyn Error>> {
    load_from_file(TRANSLATED_WORDS_PATH)
}

pub fn save_translated_words(words: &HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    save_to_file(words, TRANSLATED_WORDS_PATH)
}

pub fn load_alphabet() -> Result<Vec<String>, Box<dyn Error>> {
    load_from_file(ALPHABET_PATH)
}

pub fn save_alphabet(alphabet: &Vec<String>) -> Result<(), Box<dyn Error>> {
    save_to_file(alphabet, ALPHABET_PATH)
}

fn save_to_file<TData>(data: &TData, path: &'static str) -> Result<(), Box<dyn Error>>
where
    TData: serde::ser::Serialize,
{
    initialize_directory()?;
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .map_err(|inner| SaveFileError {
            name: path,
            source: inner,
        })?;

    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, data).map_err(|inner| SaveFileError {
        name: path,
        source: inner,
    })?;
    Ok(())
}

fn load_from_file<TData>(path: &'static str) -> Result<TData, Box<dyn Error>>
where
    TData: for<'de> serde::Deserialize<'de>,
{
    initialize_directory()?;
    let file = File::open(path).map_err(|inner| LoadFileError {
        name: path,
        source: inner,
    })?;

    let reader = BufReader::new(file);
    let result = serde_json::from_reader(reader).map_err(|inner| LoadFileError {
        name: path,
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
    name: &'static str,
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
    name: &'static str,
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
