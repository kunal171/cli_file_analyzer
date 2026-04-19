// src/main.rs
// Main file for the CLI File Analyser application
// This application reads the contents of a specified file and prints it to the console.

// Importing the reader module which contains the function to read files
mod reader;
mod analyzer;

use std::io;
use std::env::args;

fn main() {
    // Getting the file path from the command line arguments
    let mut file_path = args().nth(1);

    let file_path = match file_path {
        Some(path) => path,
        None => {
            eprintln!("Error: no file path provided.");
            eprintln!("Usage: cargo run -- <file_path>");
            std::process::exit(1);
        }
    };
    // io::stdin()
    //     .read_line(&mut file_path)
    //     .expect("Failed to read the input");

    // // Trimming the input to remove any leading or trailing whitespace characters
    // let file_path = file_path.trim();

    // Calling the read_file function from the reader module to read the contents of the text file and print it to the console
    let content = reader::read_file(&file_path).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });
    // println!("{}", content);

    let analysis = analyzer::analyze_file_content(&content);
    println!("Lines: {}", analysis.line_count);
    println!("Words: {}", analysis.word_count);
    println!("Characters: {}", analysis.character_count);
    println!("\nTop Words:");
    for (i, freq) in analysis.top_words.iter().enumerate() {
        println!("{}. {} ({})", i + 1, freq.word, freq.count);
    }
}