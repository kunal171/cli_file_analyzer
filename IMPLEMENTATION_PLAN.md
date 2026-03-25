# CLI File Analyser — Implementation Plan

---

## Phase A — Basic File Reading & CLI Args

**Rust concepts:** `std::env::args`, `String` vs `&str`, ownership, `Result`, `?` operator

### Tasks
- [ ] Parse command-line arguments manually (no external crates)
- [ ] Read a file into a `String` using `std::fs::read_to_string`
- [ ] Handle errors gracefully (file not found, permission denied)
- [ ] Print raw file contents

### What You'll Learn
- How `String` ownership works when reading files
- Pattern matching on `Result<T, E>`
- The difference between `String` (owned) and `&str` (borrowed)
- Using `std::env::args()` to collect CLI input
- The `?` operator for propagating errors

---

## Phase B — File Statistics (Structs + Iterators)

**Rust concepts:** structs, methods (`impl`), iterators, closures, `Display` trait

### Tasks
- [ ] Create a `FileStats` struct (lines, words, chars, bytes, blank lines)
- [ ] Implement methods to compute stats using iterators
- [ ] Implement `fmt::Display` for pretty output
- [ ] Handle both UTF-8 and non-UTF-8 files

### What You'll Learn
- Defining structs and implementing methods with `impl`
- Iterator chains: `.lines()`, `.split_whitespace()`, `.count()`, `.filter()`
- Trait implementation (`Display`)
- Borrowing — passing `&str` slices to functions instead of cloning

---

## Phase C — Multiple Files & Error Handling

**Rust concepts:** `Vec`, loops, custom error types, `From` trait, enums

### Tasks
- [ ] Accept multiple file paths as arguments
- [ ] Create a custom `AnalyserError` enum (IoError, InvalidPath, EncodingError)
- [ ] Implement `From<std::io::Error>` for automatic error conversion
- [ ] Aggregate stats across files and show a summary table

### What You'll Learn
- Enum-based error handling (the Rust way)
- The `From` trait and how `?` uses it for conversion
- Iterating over `Vec<String>` with borrowing
- Collecting results: `Vec<Result<T, E>>` patterns

---

## Phase D — Pattern Search (Lifetimes + Borrowing)

**Rust concepts:** lifetimes, `&str` references in structs, `Option<T>`, regex basics

### Tasks
- [ ] Add `--search <pattern>` flag to grep for text
- [ ] Return matching lines with line numbers
- [ ] Create a `SearchResult<'a>` struct holding `&'a str` references
- [ ] Highlight matches in output

### What You'll Learn
- **Lifetimes** — why `SearchResult<'a>` needs a lifetime parameter
- Borrowing content without cloning — holding `&str` into the original file buffer
- `Option<T>` for optional CLI flags
- Basic manual argument parsing with flags

---

## Phase E — Directory Walking (Recursion + Ownership)

**Rust concepts:** `std::fs::read_dir`, recursion, `PathBuf` vs `Path`, `Box` for recursive types

### Tasks
- [ ] Add `--recursive` flag to scan directories
- [ ] Walk directory trees using `read_dir`
- [ ] Filter by file extension (`--ext rs,txt`)
- [ ] Collect results into a summary

### What You'll Learn
- `PathBuf` (owned) vs `&Path` (borrowed) — mirrors `String` vs `&str`
- Recursive directory traversal with ownership challenges
- Building a `Vec<PathBuf>` by collecting owned paths
- When you need `clone()` vs when borrowing is enough

---

## Phase F — Formatted Output & Polish

**Rust concepts:** traits, generics, `serde` (optional), `clap` (optional)

### Tasks
- [ ] Add `--format` flag: `table` (default), `json`, `csv`
- [ ] Implement an `OutputFormatter` trait with different implementations
- [ ] Replace manual arg parsing with `clap` (learn external crates)
- [ ] Add `--help` usage text

### What You'll Learn
- Defining and implementing **custom traits**
- **Trait objects** (`Box<dyn OutputFormatter>`) for runtime polymorphism
- Using external crates and `Cargo.toml` dependencies
- `serde` serialization (if using JSON output)

---

## Concepts Covered Per Phase

| Phase | Key Concepts |
|-------|-------------|
| A | Ownership, `String` vs `&str`, `Result`, error handling |
| B | Structs, `impl`, iterators, closures, `Display` trait |
| C | Enums, custom errors, `From` trait, `Vec` |
| D | **Lifetimes**, borrowing, `Option`, references in structs |
| E | `PathBuf` vs `Path`, recursion, `clone` vs borrow |
| F | Traits, generics, trait objects, external crates |
