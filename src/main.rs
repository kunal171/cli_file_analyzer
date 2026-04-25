// src/main.rs
// Main file for the CLI File Analyser application
// This application reads the contents of a specified file and prints it to the console.

// Importing the reader module which contains the function to read files
mod analyzer;
mod error;
mod output;
mod reader;

use crate::error::FileAnalyserError;
use crate::output::OutputFormat;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

struct Config {
    format: OutputFormat,
    recursive: bool,
    extensions: Option<Vec<String>>,
    file_paths: Vec<String>,
    search_pattern: Option<String>,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
        print_usage();
        std::process::exit(1);
    }
}

fn run() -> Result<(), FileAnalyserError> {
    let config = Config::parse(env::args().skip(1).collect())?;
    let paths = collect_paths(&config.file_paths, config.recursive, &config.extensions)?;

    for path in paths {
        let file_path = path.to_string_lossy();

        match reader::read_file(&file_path) {
            Ok(content) => {
                if let Some(ref pattern) = config.search_pattern {
                    let result = analyzer::search_pattern(&content, pattern);
                    output::print_search(&file_path, &result, config.format)?;
                } else {
                    let analysis = analyzer::analyze_file_content(&content);
                    output::print_analysis(&file_path, &analysis, config.format)?;
                }
            }
            Err(e) => eprintln!("Error analyzing {}: {}", file_path, e),
        }
    }

    Ok(())
}

impl Config {
    fn parse(args: Vec<String>) -> Result<Self, FileAnalyserError> {
        let mut format = OutputFormat::Text;
        let mut recursive = false;
        let mut extensions = None;
        let mut file_paths = Vec::new();
        let mut search_pattern = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--help" | "-h" => {
                    print_usage();
                    std::process::exit(0);
                }
                "--json" => format = OutputFormat::Json,
                "--recursive" => recursive = true,
                "--format" => {
                    let value = args.get(i + 1).ok_or_else(|| {
                        FileAnalyserError::InvalidFilePath("--format requires a value".to_string())
                    })?;
                    format = OutputFormat::parse(value)?;
                    i += 1;
                }
                "--ext" => {
                    let value = args.get(i + 1).ok_or_else(|| {
                        FileAnalyserError::InvalidFilePath("--ext requires a value".to_string())
                    })?;
                    extensions = Some(parse_extensions(value));
                    i += 1;
                }
                "--search" => {
                    let value = args.get(i + 1).ok_or_else(|| {
                        FileAnalyserError::InvalidFilePath(
                            "--search requires a pattern".to_string(),
                        )
                    })?;
                    search_pattern = Some(value.clone());
                    i += 1;
                }
                value if value.starts_with('-') => {
                    return Err(FileAnalyserError::InvalidFilePath(format!(
                        "unknown flag: {value}"
                    )));
                }
                value => file_paths.push(value.to_string()),
            }

            i += 1;
        }

        if file_paths.is_empty() {
            return Err(FileAnalyserError::InvalidFilePath(
                "at least one file or directory path is required".to_string(),
            ));
        }

        Ok(Self {
            format,
            recursive,
            extensions,
            file_paths,
            search_pattern,
        })
    }
}

fn collect_paths(
    inputs: &[String],
    recursive: bool,
    extensions: &Option<Vec<String>>,
) -> Result<Vec<PathBuf>, FileAnalyserError> {
    let mut paths = Vec::new();

    for input in inputs {
        let path = Path::new(input);

        if path.is_file() {
            push_if_extension_matches(path, extensions, &mut paths);
        } else if path.is_dir() {
            if recursive {
                collect_directory_paths(path, extensions, &mut paths)?;
            } else {
                return Err(FileAnalyserError::InvalidFilePath(format!(
                    "{} is a directory; pass --recursive to scan it",
                    path.display()
                )));
            }
        } else {
            return Err(FileAnalyserError::InvalidFilePath(format!(
                "{} does not exist",
                path.display()
            )));
        }
    }

    Ok(paths)
}

fn collect_directory_paths(
    directory: &Path,
    extensions: &Option<Vec<String>>,
    paths: &mut Vec<PathBuf>,
) -> Result<(), FileAnalyserError> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            push_if_extension_matches(&path, extensions, paths);
        } else if path.is_dir() {
            collect_directory_paths(&path, extensions, paths)?;
        }
    }

    Ok(())
}

fn push_if_extension_matches(
    path: &Path,
    extensions: &Option<Vec<String>>,
    paths: &mut Vec<PathBuf>,
) {
    if extensions
        .as_ref()
        .is_none_or(|allowed| extension_matches(path, allowed))
    {
        paths.push(path.to_path_buf());
    }
}

fn extension_matches(path: &Path, allowed_extensions: &[String]) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| allowed_extensions.contains(&extension.to_lowercase()))
        .unwrap_or(false)
}

fn parse_extensions(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(|extension| extension.trim().trim_start_matches('.').to_lowercase())
        .filter(|extension| !extension.is_empty())
        .collect()
}

fn print_usage() {
    eprintln!(
        "Usage: cargo run -- [--json | --format text|table|json|csv] [--search <pattern>] [--recursive] [--ext rs,txt] <path>..."
    );
}
