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
const TRANSLATED_WORDS: &str = "translated_words.json";
const TRANSLATED_WORDS_FILE: &str = concatcp!(DATA_DIRECTORY, "/", TRANSLATED_WORDS);

pub fn load_translated_words() -> Result<HashMap<String, String>, Box<dyn Error>> {
    initialize_directory()?;
    let file = File::open(TRANSLATED_WORDS_FILE).map_err(|inner| LoadFileError {
        name: TRANSLATED_WORDS_FILE,
        source: inner,
    })?;

    let reader = BufReader::new(file);
    let result = serde_json::from_reader(reader).map_err(|inner| LoadFileError {
        name: TRANSLATED_WORDS_FILE,
        source: inner,
    })?;
    Ok(result)
}

pub fn save_translated_words(words: &HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    initialize_directory()?;
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(TRANSLATED_WORDS_FILE)
        .map_err(|inner| SaveFileError {
            name: TRANSLATED_WORDS_FILE,
            source: inner,
        })?;

    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, words).map_err(|inner| SaveFileError {
        name: TRANSLATED_WORDS_FILE,
        source: inner,
    })?;

    Ok(())
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
