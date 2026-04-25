use crate::analyzer::{FileAnalysis, SearchResult};
use crate::error::FileAnalyserError;
use serde::Serialize;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Table,
    Json,
    Csv,
}

impl OutputFormat {
    pub fn parse(value: &str) -> Result<Self, FileAnalyserError> {
        match value {
            "text" => Ok(Self::Text),
            "table" => Ok(Self::Table),
            "json" => Ok(Self::Json),
            "csv" => Ok(Self::Csv),
            other => Err(FileAnalyserError::InvalidFilePath(format!(
                "unsupported output format: {other}"
            ))),
        }
    }
}

pub fn print_analysis(
    file_path: &str,
    analysis: &FileAnalysis,
    format: OutputFormat,
) -> Result<(), FileAnalyserError> {
    match format {
        OutputFormat::Text => print_analysis_text(file_path, analysis),
        OutputFormat::Table => print_analysis_table(file_path, analysis),
        OutputFormat::Json => print_json(analysis),
        OutputFormat::Csv => print_analysis_csv(file_path, analysis),
    }
}

pub fn print_search(
    file_path: &str,
    result: &SearchResult<'_>,
    format: OutputFormat,
) -> Result<(), FileAnalyserError> {
    match format {
        OutputFormat::Text => print_search_text(file_path, result),
        OutputFormat::Table => print_search_table(file_path, result),
        OutputFormat::Json => print_json(result),
        OutputFormat::Csv => print_search_csv(file_path, result),
    }
}

fn print_json<T: Serialize>(value: &T) -> Result<(), FileAnalyserError> {
    let json = serde_json::to_string_pretty(value)
        .map_err(|error| FileAnalyserError::AnalysisError(error.to_string()))?;
    println!("{json}");
    Ok(())
}

fn print_analysis_text(file_path: &str, analysis: &FileAnalysis) -> Result<(), FileAnalyserError> {
    println!("File: {file_path}");
    println!("Lines: {}", analysis.line_count);
    println!("Blank Lines: {}", analysis.blank_line_count);
    println!("Words: {}", analysis.word_count);
    println!("Characters: {}", analysis.character_count);
    println!("Bytes: {}", analysis.byte_count);
    println!("\nTop Words:");

    for (index, freq) in analysis.top_words.iter().enumerate() {
        println!("{}. {} ({})", index + 1, freq.word, freq.count);
    }

    Ok(())
}

fn print_analysis_table(file_path: &str, analysis: &FileAnalysis) -> Result<(), FileAnalyserError> {
    println!(
        "{:<30} {:>8} {:>8} {:>8} {:>10} {:>8}",
        "File", "Lines", "Blank", "Words", "Chars", "Bytes"
    );
    println!(
        "{:<30} {:>8} {:>8} {:>8} {:>10} {:>8}",
        truncate(file_path, 30),
        analysis.line_count,
        analysis.blank_line_count,
        analysis.word_count,
        analysis.character_count,
        analysis.byte_count
    );

    Ok(())
}

fn print_analysis_csv(file_path: &str, analysis: &FileAnalysis) -> Result<(), FileAnalyserError> {
    println!("file,lines,blank_lines,words,characters,bytes");
    println!(
        "{},{},{},{},{},{}",
        escape_csv(file_path),
        analysis.line_count,
        analysis.blank_line_count,
        analysis.word_count,
        analysis.character_count,
        analysis.byte_count
    );

    Ok(())
}

fn print_search_text(file_path: &str, result: &SearchResult<'_>) -> Result<(), FileAnalyserError> {
    println!("File: {file_path}");
    println!("Pattern: \"{}\"", result.pattern);
    println!("Matching Lines: {}", result.matching_line_count);
    println!("Total Matches: {}", result.total_matches);
    println!("\nMatches:");

    for search_match in &result.matches {
        println!("{}: {}", search_match.line_number, search_match.line);
    }

    Ok(())
}

fn print_search_table(file_path: &str, result: &SearchResult<'_>) -> Result<(), FileAnalyserError> {
    println!("{:<30} {:>8} {:>8}  Pattern", "File", "Lines", "Matches");
    println!(
        "{:<30} {:>8} {:>8}  {}",
        truncate(file_path, 30),
        result.matching_line_count,
        result.total_matches,
        result.pattern
    );

    Ok(())
}

fn print_search_csv(file_path: &str, result: &SearchResult<'_>) -> Result<(), FileAnalyserError> {
    println!("file,line_number,line");

    for search_match in &result.matches {
        println!(
            "{},{},{}",
            escape_csv(file_path),
            search_match.line_number,
            escape_csv(search_match.line)
        );
    }

    Ok(())
}

fn truncate(value: &str, max_chars: usize) -> String {
    let mut chars = value.chars();
    let truncated: String = chars.by_ref().take(max_chars).collect();

    if chars.next().is_some() {
        format!("{truncated}...")
    } else {
        truncated
    }
}

fn escape_csv(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}
