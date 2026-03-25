// src/main.rs
// Main file for the CLI File Analyser application
// This application reads the contents of a specified file and prints it to the console.

// Importing the reader module which contains the function to read files
mod reader;

fn main() {
    // Calling the read_file function from the reader module to read the contents of the text file and print it to the console"
    let content = reader::read_file("src/main.rs").unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });
    println!("{}", content);
}