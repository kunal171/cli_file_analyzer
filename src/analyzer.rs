// src/analyzer.rs
// This module contains functions for analyzing the contents of a file.

use std::collections::HashMap;
use serde::Serialize;

// Struct to represent the analysis results of a file
// This struct contains the line count, word count, character count, and top words in the file
#[derive(Serialize)]
pub struct FileAnalysis {
    pub line_count: usize,
    pub word_count: usize,
    pub character_count: usize,
    pub top_words: Vec<WordFrequency>, // Placeholder for top words
}

// Struct to represent a word and its frequency
// This struct is used to store the word and its count for the top words in the file
#[derive(Serialize)]
pub struct WordFrequency {
    pub word: String,
    pub count: usize,
}

// Struct to represent the results of a search operation in the file
// This struct contains the search pattern, the count of matches, and the lines that contain the
#[derive(Serialize)]
pub struct SearchResult {
    pub pattern: String,
    pub matches_count: usize,
    pub matching_lines: Vec<String>,
}

// Function to analyze the contents of the file and return a FileAnalysis struct
pub fn analyze_file_content(content: &str) -> FileAnalysis {
    // Counting lines, words, and characters in the file content
    let line_count = content.lines().count();
    let word_count = content.split_whitespace().count();
    let character_count = content.chars().count();
    let mut word_freq: HashMap<String, usize> = HashMap::new();

    // let mut word_frequencies = HashMap::new();
    for word in content.split_whitespace() {
        let normalized = normalize_word(word);
        if !normalized.is_empty() {
            *word_freq.entry(normalized).or_insert(0) += 1;
        }
    }

    // Converting the word frequency HashMap into a vector of WordFrequency structs
    let mut top_words: Vec<WordFrequency> = word_freq
        .into_iter()
        .map(|(word, count)| WordFrequency { word, count })
        .collect();

    // Sorting the words by frequency in descending order
    top_words.sort_by(|a, b| b.count.cmp(&a.count));
    
    // Keep only top 10
    top_words.truncate(10);

    // Returning the analysis results as a FileAnalysis struct
    FileAnalysis {
        line_count,
        word_count,
        character_count,
        top_words,
    }
}

// Helper function to normalize words by converting them to lowercase and removing punctuation
fn normalize_word(word: &str) -> String {
    word
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect()
}

/// Search for a pattern in the file content and return matching lines
pub fn search_pattern(content: &str, pattern: &str) -> SearchResult {
    let mut matches_count = 0;
    let mut matching_lines = Vec::new();
    let pattern_lower = pattern.to_lowercase();

    // Convert the search pattern to lowercase for case-insensitive search
    for line in content.lines() {
        if line.to_lowercase().contains(&pattern_lower) {
            matches_count += 1;
            matching_lines.push(line.to_string());
        }
    }

    SearchResult {
        pattern: pattern.to_string(),
        matches_count,
        matching_lines,
    }
}