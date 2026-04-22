use std::io;

// Define a custom error type for the application
#[derive(Debug)]
pub enum FileAnalyserError {
    IoError(io::Error), // Error related to file I/O operations
    InvalidFilePath(String), // Error for invalid file paths
    AnalysisError(String), // Error during analysis of the file content
}

// Implement conversion from io::Error to FileAnalyserError
impl From<io::Error> for FileAnalyserError {
    fn from(err: io::Error) -> Self {
        FileAnalyserError::IoError(err)
    }
}

// Implement the Display trait for FileAnalyserError to provide user-friendly error messages
impl std::fmt::Display for FileAnalyserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FileAnalyserError::IoError(e) => write!(f, "IO Error: {}", e),
            FileAnalyserError::InvalidFilePath(p) => write!(f, "Invalid file path: {}", p),
            FileAnalyserError::AnalysisError(e) => write!(f, "Analysis Error: {}", e),
        }
    }
}