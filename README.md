# CLI File Analyser

A command-line tool for analyzing files, built in Rust.

**Phase 1 practice project** — focusing on ownership, borrowing, error handling, and file I/O.

## Features

- [ ] Read and analyze files from command-line arguments
- [ ] Display file metadata (size, line count, word count, character count)
- [ ] Detect file encoding and type
- [ ] Search for patterns within files
- [ ] Support multiple files and directory scanning
- [ ] Formatted output (table, JSON)

## Rust Concepts Practiced

- Ownership & borrowing with file handles and string buffers
- Error handling with `Result`, `?` operator, and custom error types
- Lifetimes in struct references
- Iterators and closures for text processing
- CLI argument parsing
- File I/O with `std::fs` and `std::io`

## Usage

```bash
cargo run -- <file_path>
```

## Build

```bash
cargo build --release
```
