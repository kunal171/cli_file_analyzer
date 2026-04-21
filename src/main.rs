// src/main.rs
// Main file for the CLI File Analyser application
// This application reads the contents of a specified file and prints it to the console.

// Importing the reader module which contains the function to read files
mod reader;
mod analyzer;
mod error;

use serde_json;

use std::env;

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().skip(1).collect();
    let mut json_output = false;
    let mut file_paths: Vec<String> = Vec::new();

    for arg in args {
        if arg == "--json" {
            json_output = true;
        } else {
            file_paths.push(arg);
        }
    }

    // Check if any file paths were provided, if not, print usage instructions and exit
    if file_paths.is_empty() {
        eprintln!("Usage: cargo run -- [--json] <file1> <file2> ...");
        std::process::exit(1);
    }

    // Loop through each file path provided and analyze the file content
    for file_path in file_paths {
        match reader::read_file(&file_path) {
            // If the file is read successfully, analyze its content and print the results
            Ok(content) => {
                // Analyze the file content using the analyzer module and print the results to the console
                let analysis = analyzer::analyze_file_content(&content);
                if json_output {
                    println!("{}", serde_json::to_string_pretty(&analysis).unwrap());
                } else {
                    println!("File: {}", file_path);
                    println!("Lines: {}", analysis.line_count);
                    println!("Words: {}", analysis.word_count);
                    println!("Characters: {}", analysis.character_count);
                    println!("\nTop Words:");
                    for (i, freq) in analysis.top_words.iter().enumerate() {
                        println!("{}. {} ({})", i + 1, freq.word, freq.count);
                    }
                }
            }
            Err(e) => eprintln!("Error analyzing {}: {}", file_path, e),
        }
    }
}

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