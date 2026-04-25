# CLI File Analyser

A command-line tool for analyzing text files, built in Rust.

**Phase 1 practice project** — focusing on ownership, borrowing, iterators, error handling, and file I/O.

## Project 1 — CLI File Analyzer (Detailed Guide)

### Goal
Build a CLI tool to analyze text files and report:
- line count
- word count
- character count
- word frequency and top words

### Concepts Covered
- Ownership & borrowing
- `String` vs `&str`
- Iterators and iterator adapters
- `HashMap` for frequency counting
- Error handling with `Result`

### Architecture
CLI → File Reader → Text Analyzer → Output Formatter

### Step-by-Step Build
1. Create a new Rust project using `cargo new cli_file_analyser`
2. Parse CLI arguments from `std::env::args`
3. Read the file using `std::fs::read_to_string`
4. Split the text into words using `split_whitespace`
5. Count word frequency with `HashMap<String, usize>`
6. Sort the frequency results and print the top words

### Recommended structure
- `src/main.rs` — CLI entry point and argument handling
- `src/reader.rs` — file reading and I/O error conversion
- `src/analyzer.rs` — text analysis, counts, and frequency computation
- `src/output.rs` — formatted output for summary and top words

### Example task list
- [ ] Parse a single file path from CLI arguments
- [ ] Read the file contents into a `String`
- [ ] Compute line count with `content.lines().count()`
- [ ] Compute word count with `content.split_whitespace().count()`
- [ ] Compute character count with `content.chars().count()`
- [ ] Count frequencies with `HashMap::new()` and `entry().or_insert(0)`
- [ ] Sort frequency pairs by count and display the top N words
- [ ] Print a summary and optional top words list

### Example implementation notes
- Use `map_err` or `?` to convert I/O errors to a user-friendly message.
- Keep the analyzer functions generic over `&str` so they can be tested easily.
- Use `content.lines()` and `content.split_whitespace()` to avoid manual parsing.
- Avoid cloning the full file content; pass references when possible.

### Interview questions
- Explain the ownership model in Rust.
- What is the difference between `String` and `&str`?
- How does `HashMap` store and look up values?
- How does Rust handle errors with `Result` and `panic`?

## Features

- [x] Read and analyze files from command-line arguments
- [x] Display file metadata (size, line count, word count, character count)
- [ ] Detect file encoding and type
- [x] Search for patterns within files
- [x] Support multiple files and directory scanning
- [x] JSON output format
- [x] Formatted output (table)
- [x] CSV output format

## Rust Concepts Practiced

- Ownership & borrowing with file handles and string buffers
- Error handling with `Result`, `?` operator, and custom error types
- Lifetimes in struct references
- Iterators and closures for text processing
- CLI argument parsing
- File I/O with `std::fs` and `std::io`

## Usage

```bash
# Analyze a file with default output
cargo run -- <file_path>

# Analyze multiple files
cargo run -- file1.txt file2.txt

# Output results in JSON format
cargo run -- --json <file_path>

# Choose an output format
cargo run -- --format table <file_path>
cargo run -- --format csv <file_path>

# Search with line numbers
cargo run -- --search rust <file_path>

# Recursively scan a directory and filter extensions
cargo run -- --recursive --ext rs,txt <directory>
```

## Build

```bash
cargo build --release
```
