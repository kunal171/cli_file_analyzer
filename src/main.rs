// src/main.rs
// Main file for the CLI File Analyser application
// This application reads the contents of a specified file and prints it to the console.

// Importing the reader module which contains the function to read files
mod reader;
mod analyzer;
mod error;

use std::io;
use std::env::args;

fn main() {
    // Getting the file path from the command line arguments
    let file_paths: Vec<String> = std::env::args()
        .skip(1)
        .collect();

    if file_paths.is_empty() {
        eprintln!("Usage: cargo run -- <file1> <file2> ...");
        std::process::exit(1);
    }

    for file_path in file_paths {
        match reader::read_file(&file_path) {
            Ok(content) => {
                let analysis = analyzer::analyze_file_content(&content);
                println!("File: {}", file_path);
                println!("Lines: {}", analysis.line_count);
                println!("Words: {}", analysis.word_count);
                println!("Characters: {}", analysis.character_count);
                println!("\nTop Words:");
                for (i, freq) in analysis.top_words.iter().enumerate() {
                    println!("{}. {} ({})", i + 1, freq.word, freq.count);
                }
            }
            Err(e) => eprintln!("Error analyzing {}: {}", file_path, e),
        }
    }
    // io::stdin()
    //     .read_line(&mut file_path)
    //     .expect("Failed to read the input");

    // // Trimming the input to remove any leading or trailing whitespace characters
    // let file_path = file_path.trim();

    // Calling the read_file function from the reader module to read the contents of the text file and print it to the console
    // let content = reader::read_file(&file_path).unwrap_or_else(|e| {
    //     eprintln!("Error: {}", e);
    //     std::process::exit(1);
    // });
    // println!("{}", content);

    // let analysis = analyzer::analyze_file_content(&content);
    // println!("Lines: {}", analysis.line_count);
    // println!("Words: {}", analysis.word_count);
    // println!("Characters: {}", analysis.character_count);
    // println!("\nTop Words:");
    // for (i, freq) in analysis.top_words.iter().enumerate() {
    //     println!("{}. {} ({})", i + 1, freq.word, freq.count);
    // }
}