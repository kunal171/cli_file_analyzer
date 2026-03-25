//reader.rs
use std::{fs, io};
use std::io::Error;


//Function to read the contents of the file and return it as a String
pub fn read_file(file_path: &str) ->Result<String, Error> {
    //read_to_string is a convenient method to read the entire contents of a file into a String
    let content = fs::read_to_string(file_path).
        map_err(|e| {
            format!("Failed to read file '{}': {}", file_path, e)
        })
    Ok(content)
}