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
    let mut search_pattern: Option<String> = None;

    // Loop through the command line arguments and separate file paths, JSON output flag, and search pattern
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => json_output = true,
            "--search" => {
                if i + 1 < args.len() {
                    search_pattern = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Error: --search requires a pattern");
                    std::process::exit(1);
                }
            }
            _ => file_paths.push(args[i].clone()),
        }
        i += 1;
    }

    // Check if any file paths were provided, if not, print usage instructions and exit
    if file_paths.is_empty() {
        eprintln!("Usage: cargo run -- [--json] <file1> <file2> ...");
        std::process::exit(1);
    }

       // Loop through each file path provided and analyze the file content
    for file_path in file_paths {
        match reader::read_file(&file_path) {
            Ok(content) => {
                // If search pattern is provided, search instead of analyzing
                if let Some(ref pattern) = search_pattern {
                    let result = analyzer::search_pattern(&content, pattern);
                    if json_output {
                        println!("{}", serde_json::to_string_pretty(&result).unwrap());
                    } else {
                        println!("File: {}", file_path);
                        println!("Pattern: \"{}\"", result.pattern);
                        println!("Matches: {}", result.matches_count);
                        println!("\nMatching Lines:");
                        for (i, line) in result.matching_lines.iter().enumerate() {
                            println!("{}. {}", i + 1, line);
                        }
                    }
                } else {
                    // Normal analysis mode
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
            }
            Err(e) => eprintln!("Error analyzing {}: {}", file_path, e),
        }
    }
}