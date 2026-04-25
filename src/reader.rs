use crate::error::FileAnalyserError;
use std::fs;

// Function to read the contents of the file and return it as a String
pub fn read_file(file_path: &str) -> Result<String, FileAnalyserError> {
    Ok(fs::read_to_string(file_path)?)
}
