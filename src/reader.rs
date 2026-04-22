//reader.rs
use std::{fs, io};
// use std::io::Error;
use crate::error::FileAnalyserError;


//Function to read the contents of the file and return it as a String
pub fn read_file(file_path: &str) -> Result<String, FileAnalyserError> {
    // Attempt to read the file and handle any potential I/O errors
    fs::read_to_string(file_path).map_err(|e| FileAnalyserError::IoError(e))
}