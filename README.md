# CLI File Analyser

A command-line tool for analyzing text files, built in Rust.

## What It Does

Analyzes one or more text files and reports:

- line count, blank line count
- word count
- character count, byte count
- top 10 word frequencies (normalized, case-insensitive)
- case-insensitive pattern search with line numbers and match counts

## Project Structure

```text
cli_file_analyser/
├── Cargo.toml
├── example.txt
└── src/
    ├── main.rs       — CLI parsing, directory walking, coordination
    ├── reader.rs      — file reading
    ├── analyzer.rs    — text analysis, word frequency, pattern search
    ├── output.rs      — text, table, JSON, CSV output formatting
    └── error.rs       — custom error type
```

## Architecture

```text
CLI args
    |
Config::parse()
    |
collect_paths() — resolve files, walk directories if --recursive
    |
for each file:
    reader::read_file()
        |
    analyzer::analyze_file_content()  or  analyzer::search_pattern()
        |
    output::print_analysis()  or  output::print_search()
```

## Usage

```bash
# Analyze a file
cargo run -- example.txt

# Analyze multiple files
cargo run -- file1.txt file2.txt

# JSON output
cargo run -- --json example.txt

# Choose output format: text, table, json, csv
cargo run -- --format table example.txt
cargo run -- --format csv example.txt

# Case-insensitive pattern search
cargo run -- --search rust example.txt

# Recursive directory scan with extension filter
cargo run -- --recursive --ext rs,txt .
```

## Output Formats

- **text** (default) — human-readable summary with top words list
- **table** — columnar summary
- **json** — structured JSON via serde
- **csv** — comma-separated values with proper escaping

## Dependencies

- `serde` — struct serialization
- `serde_json` — JSON output

## Commands

```bash
cargo run -- example.txt
cargo test
cargo fmt --check
cargo clippy
cargo build --release
```

## Tests

Unit tests in `analyzer.rs` cover:

- basic line, word, character, byte counts
- blank line counting
- word normalization and punctuation handling
- top 10 truncation
- empty input
- case-insensitive search
- multiple matches per line
- no-match and empty-pattern edge cases
