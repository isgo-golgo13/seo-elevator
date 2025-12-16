//! Error types for the analyzer crate

use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnalyzerError {
    #[error("Failed to read file '{0}': {1}")]
    FileRead(PathBuf, std::io::Error),

    #[error("Failed to parse HTML: {0}")]
    HtmlParse(String),

    #[error("Invalid selector: {0}")]
    InvalidSelector(String),

    #[error("Analysis failed: {0}")]
    AnalysisFailed(String),

    #[error("No HTML files found in directory: {0}")]
    NoHtmlFiles(PathBuf),
}
