//! Error types for the ML engine crate

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MlEngineError {
    #[error("Sentiment analysis failed: {0}")]
    SentimentError(String),

    #[error("Optimization failed: {0}")]
    OptimizationError(String),

    #[error("Model loading failed: {0}")]
    ModelLoadError(String),

    #[error("Inference failed: {0}")]
    InferenceError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}
