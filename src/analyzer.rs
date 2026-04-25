// src/analyzer.rs
// This module contains functions for analyzing the contents of a file.

use serde::Serialize;
use std::collections::HashMap;

// Struct to represent the analysis results of a file
// This struct contains the line count, word count, character count, and top words in the file
#[derive(Serialize)]
pub struct FileAnalysis {
    pub line_count: usize,
    pub word_count: usize,
    pub character_count: usize,
    pub byte_count: usize,
    pub blank_line_count: usize,
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
pub struct SearchResult<'a> {
    pub pattern: String,
    pub matching_line_count: usize,
    pub total_matches: usize,
    pub matches: Vec<SearchMatch<'a>>,
}

#[derive(Serialize)]
pub struct SearchMatch<'a> {
    pub line_number: usize,
    pub line: &'a str,
}

// Function to analyze the contents of the file and return a FileAnalysis struct
pub fn analyze_file_content(content: &str) -> FileAnalysis {
    // Counting lines, words, and characters in the file content
    let line_count = content.lines().count();
    let word_count = content.split_whitespace().count();
    let character_count = content.chars().count();
    let byte_count = content.len();
    let blank_line_count = content
        .lines()
        .filter(|line| line.trim().is_empty())
        .count();
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
        byte_count,
        blank_line_count,
        top_words,
    }
}

// Helper function to normalize words by converting them to lowercase and removing punctuation
fn normalize_word(word: &str) -> String {
    word.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect()
}

/// Search for a pattern in the file content and return matching lines
pub fn search_pattern<'a>(content: &'a str, pattern: &str) -> SearchResult<'a> {
    let mut total_matches = 0;
    let mut matches = Vec::new();
    let pattern_lower = pattern.to_lowercase();

    if pattern_lower.is_empty() {
        return SearchResult {
            pattern: pattern.to_string(),
            matching_line_count: 0,
            total_matches: 0,
            matches,
        };
    }

    // Convert the search pattern to lowercase for case-insensitive search
    for (index, line) in content.lines().enumerate() {
        let line_lower = line.to_lowercase();
        let line_matches = line_lower.matches(&pattern_lower).count();

        if line_matches > 0 {
            total_matches += line_matches;
            matches.push(SearchMatch {
                line_number: index + 1,
                line,
            });
        }
    }

    SearchResult {
        pattern: pattern.to_string(),
        matching_line_count: matches.len(),
        total_matches,
        matches,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn find_word_count(analysis: &FileAnalysis, target: &str) -> Option<usize> {
        analysis
            .top_words
            .iter()
            .find(|freq| freq.word == target)
            .map(|freq| freq.count)
    }

    #[test]
    fn analyzes_basic_counts() {
        let content = "Rust is fast.\nRust is safe.";

        let analysis = analyze_file_content(content);

        assert_eq!(analysis.line_count, 2);
        assert_eq!(analysis.word_count, 6);
        assert_eq!(analysis.character_count, content.chars().count());
        assert_eq!(analysis.byte_count, content.len());
        assert_eq!(analysis.blank_line_count, 0);
    }

    #[test]
    fn counts_blank_lines() {
        let content = "Rust\n\n   \nGo";

        let analysis = analyze_file_content(content);

        assert_eq!(analysis.line_count, 4);
        assert_eq!(analysis.blank_line_count, 2);
    }

    #[test]
    fn normalizes_words_for_frequency_counting() {
        let content = "Rust, rust! RUST? safety.";

        let analysis = analyze_file_content(content);

        assert_eq!(find_word_count(&analysis, "rust"), Some(3));
        assert_eq!(find_word_count(&analysis, "safety"), Some(1));
    }

    #[test]
    fn ignores_words_that_become_empty_after_normalization() {
        let content = "... !!! Rust";

        let analysis = analyze_file_content(content);

        assert_eq!(analysis.word_count, 3);
        assert_eq!(analysis.top_words.len(), 1);
        assert_eq!(analysis.top_words[0].word, "rust");
        assert_eq!(analysis.top_words[0].count, 1);
    }

    #[test]
    fn keeps_only_top_ten_words() {
        let content = "one one one one one one one one one one one one \
            two two two two two two two two two two two \
            three three three three three three three three three three \
            four four four four four four four four four \
            five five five five five five five five \
            six six six six six six six \
            seven seven seven seven seven seven \
            eight eight eight eight eight \
            nine nine nine nine \
            ten ten ten \
            eleven eleven \
            twelve";

        let analysis = analyze_file_content(content);

        assert_eq!(analysis.top_words.len(), 10);
        assert!(find_word_count(&analysis, "one").is_some());
        assert!(find_word_count(&analysis, "ten").is_some());
        assert!(find_word_count(&analysis, "eleven").is_none());
        assert!(find_word_count(&analysis, "twelve").is_none());
    }

    #[test]
    fn handles_empty_content() {
        let analysis = analyze_file_content("");

        assert_eq!(analysis.line_count, 0);
        assert_eq!(analysis.word_count, 0);
        assert_eq!(analysis.character_count, 0);
        assert_eq!(analysis.byte_count, 0);
        assert_eq!(analysis.blank_line_count, 0);
        assert!(analysis.top_words.is_empty());
    }

    #[test]
    fn normalizes_word_to_lowercase_alphanumeric_text() {
        assert_eq!(normalize_word("Rust-lang!"), "rustlang");
        assert_eq!(normalize_word("123...Go"), "123go");
        assert_eq!(normalize_word("!!!"), "");
    }

    #[test]
    fn searches_case_insensitively_and_borrows_matching_lines() {
        let content = "Rust is fast\nPython is flexible\nasync rust is powerful";

        let result = search_pattern(content, "RUST");

        assert_eq!(result.pattern, "RUST");
        assert_eq!(result.matching_line_count, 2);
        assert_eq!(result.total_matches, 2);
        assert_eq!(result.matches[0].line_number, 1);
        assert_eq!(result.matches[0].line, "Rust is fast");
        assert_eq!(result.matches[1].line_number, 3);
        assert_eq!(result.matches[1].line, "async rust is powerful");
    }

    #[test]
    fn search_counts_multiple_matches_on_one_line() {
        let content = "rust rust\nno match\nRust";

        let result = search_pattern(content, "rust");

        assert_eq!(result.matching_line_count, 2);
        assert_eq!(result.total_matches, 3);
    }

    #[test]
    fn search_returns_no_matches_when_pattern_is_absent() {
        let content = "ownership\nborrowing\nlifetimes";

        let result = search_pattern(content, "thread");

        assert_eq!(result.matching_line_count, 0);
        assert_eq!(result.total_matches, 0);
        assert!(result.matches.is_empty());
    }

    #[test]
    fn search_treats_empty_pattern_as_no_match() {
        let result = search_pattern("rust", "");

        assert_eq!(result.matching_line_count, 0);
        assert_eq!(result.total_matches, 0);
        assert!(result.matches.is_empty());
    }
}
