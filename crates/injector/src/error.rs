//! Error types for the injector crate

use thiserror::Error;

#[derive(Error, Debug)]
pub enum InjectorError {
    #[error("Failed to find injection point in HTML")]
    NoInjectionPoint,

    #[error("Failed to parse HTML: {0}")]
    HtmlParse(String),

    #[error("Failed to generate SEO content: {0}")]
    GenerationFailed(String),

    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}
