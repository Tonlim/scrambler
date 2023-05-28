use std::fs;

pub fn initialize_directory() -> Result<(), String> {
    match fs::create_dir_all("scrambler_data") {
        Ok(_) => Ok(()),
        Err(io_error) => Err(io_error.to_string()),
    }
}
