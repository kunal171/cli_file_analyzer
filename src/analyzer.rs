// src/analyzer.rs
// This module contains functions for analyzing the contents of a file.

use std::collections::HashMap;
use serde::Serialize;

// Struct to represent the analysis results of a file
#[derive(Serialize)]
pub struct FileAnalysis {
    pub line_count: usize,
    pub word_count: usize,
    pub character_count: usize,
    pub top_words: Vec<WordFrequency>, // Placeholder for top words
}

// Struct to represent a word and its frequency
#[derive(Serialize)]
pub struct WordFrequency {
    pub word: String,
    pub count: usize,
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