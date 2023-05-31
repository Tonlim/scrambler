use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    fs::{self, File, OpenOptions},
    io::{BufReader, BufWriter},
};

#[derive(Debug)]
pub struct CreateDirectoryError {
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
pub struct LoadFileError<TError: std::error::Error> {
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
pub struct SaveFileError<TError: std::error::Error> {
    name: &'static str,
    source: TError,
}

impl<TError: std::error::Error> Display for SaveFileError<TError> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Issue encountered loading JSON from file '{}': {}",
            self.name, self.source
        )
    }
}

impl<TError: std::error::Error + 'static> Error for SaveFileError<TError> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

pub fn initialize_directory() -> Result<(), CreateDirectoryError> {
    fs::create_dir_all("scrambler_data").map_err(|inner| CreateDirectoryError {
        name: "scrambler_data",
        source: inner,
    })
}

pub fn load_translated_words() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let file =
        File::open("scrambler_data/translated_words.json").map_err(|inner| LoadFileError {
            name: "scrambler_data/translated_words.json",
            source: inner,
        })?;

    let reader = BufReader::new(file);
    let result = serde_json::from_reader(reader).map_err(|inner| LoadFileError {
        name: "scrambler_data/translated_words.json",
        source: inner,
    })?;
    Ok(result)
}

pub fn save_translated_words(words: &HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("scrambler_data/translated_words.json")
        .map_err(|inner| SaveFileError {
            name: "scrambler_data/translated_words.json",
            source: inner,
        })?;

    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, words).map_err(|inner| SaveFileError {
        name: "scrambler_data/translated_words.json",
        source: inner,
    })?;

    Ok(())
}
