//! # site-ranker-analyzer
//!
//! HTML analysis engine for site-ranker-rs.
//! Implements Strategy pattern for extensible analyzers.
//!
//! ## Architecture
//! - `AnalyzerStrategy` trait defines the contract
//! - Boxed strategies allow runtime polymorphism
//! - Multiple analyzers can be composed via `AnalyzerPipeline`

mod error;
mod strategies;
mod types;

pub use error::AnalyzerError;
pub use strategies::*;
pub use types::*;

use std::path::Path;

/// Core trait for all analyzer strategies.
/// Implement this to create custom analyzers.
pub trait AnalyzerStrategy: Send + Sync {
    /// Unique identifier for this strategy
    fn name(&self) -> &'static str;

    /// Analyze HTML content and return results
    fn analyze(&self, content: &str) -> Result<AnalysisResult, AnalyzerError>;

    /// Optional: Analyze from file path (default reads file)
    fn analyze_file(&self, path: &Path) -> Result<AnalysisResult, AnalyzerError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| AnalyzerError::FileRead(path.to_path_buf(), e))?;
        self.analyze(&content)
    }
}

/// Boxed analyzer for runtime polymorphism
pub type BoxedAnalyzer = Box<dyn AnalyzerStrategy>;

/// Pipeline to compose multiple analyzers
pub struct AnalyzerPipeline {
    analyzers: Vec<BoxedAnalyzer>,
}

impl AnalyzerPipeline {
    /// Create empty pipeline
    pub fn new() -> Self {
        Self { analyzers: Vec::new() }
    }

    /// Create pipeline with default analyzers
    pub fn default_pipeline() -> Self {
        let mut pipeline = Self::new();
        pipeline.add(Box::new(KeywordAnalyzer::new()));
        pipeline.add(Box::new(BusinessTypeAnalyzer::new()));
        pipeline.add(Box::new(SeoAuditAnalyzer::new()));
        pipeline
    }

    /// Add analyzer to pipeline
    pub fn add(&mut self, analyzer: BoxedAnalyzer) -> &mut Self {
        self.analyzers.push(analyzer);
        self
    }

    /// Run all analyzers and merge results
    pub fn analyze(&self, content: &str) -> Result<AnalysisResult, AnalyzerError> {
        let mut merged = AnalysisResult::default();

        for analyzer in &self.analyzers {
            tracing::debug!("Running analyzer: {}", analyzer.name());
            let result = analyzer.analyze(content)?;
            merged.merge(result);
        }

        Ok(merged)
    }

    /// Analyze entire directory (finds HTML files)
    pub fn analyze_directory(&self, dir: &Path) -> Result<DirectoryAnalysis, AnalyzerError> {
        use walkdir::WalkDir;

        let mut results = Vec::new();
        let mut main_file: Option<std::path::PathBuf> = None;

        for entry in WalkDir::new(dir)
            .max_depth(5)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if Self::is_html_file(path) {
                // Detect main file
                if main_file.is_none() && Self::is_main_file(path) {
                    main_file = Some(path.to_path_buf());
                }

                let content = std::fs::read_to_string(path)
                    .map_err(|e| AnalyzerError::FileRead(path.to_path_buf(), e))?;

                let result = self.analyze(&content)?;
                results.push(FileAnalysis {
                    path: path.to_path_buf(),
                    result,
                });
            }
        }

        Ok(DirectoryAnalysis {
            root: dir.to_path_buf(),
            main_file,
            files: results,
            framework: Self::detect_framework(dir),
        })
    }

    fn is_html_file(path: &Path) -> bool {
        path.extension()
            .map(|ext| ext == "html" || ext == "htm")
            .unwrap_or(false)
    }

    fn is_main_file(path: &Path) -> bool {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        matches!(name, "index.html" | "index.htm" | "_document.tsx" | "layout.tsx")
    }

    fn detect_framework(dir: &Path) -> Framework {
        if dir.join("next.config.js").exists() || dir.join("next.config.mjs").exists() {
            Framework::NextJs
        } else if dir.join("vite.config.js").exists() || dir.join("vite.config.ts").exists() {
            Framework::Vite
        } else if dir.join("package.json").exists() {
            // Check for React in package.json
            if let Ok(content) = std::fs::read_to_string(dir.join("package.json")) {
                if content.contains("\"react\"") {
                    return Framework::React;
                }
                if content.contains("\"vue\"") {
                    return Framework::Vue;
                }
                if content.contains("\"svelte\"") {
                    return Framework::Svelte;
                }
            }
            Framework::Unknown
        } else {
            Framework::VanillaHtml
        }
    }
}

impl Default for AnalyzerPipeline {
    fn default() -> Self {
        Self::default_pipeline()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_HTML: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Professional Web Services - Custom Development</title>
    <meta name="description" content="We offer professional web development services">
</head>
<body>
    <h1>Welcome to Our Services</h1>
    <p>We provide high-quality software consulting and development.</p>
    <p>Our team specializes in cloud migration and security assessments.</p>
</body>
</html>
    "#;

    #[test]
    fn test_pipeline_analysis() {
        let pipeline = AnalyzerPipeline::default_pipeline();
        let result = pipeline.analyze(SAMPLE_HTML).unwrap();

        assert!(!result.keywords.is_empty());
        assert!(result.existing_seo.has_title);
        assert!(result.existing_seo.has_description);
    }

    #[test]
    fn test_keyword_extraction() {
        let analyzer = KeywordAnalyzer::new();
        let result = analyzer.analyze(SAMPLE_HTML).unwrap();

        assert!(result.keywords.iter().any(|k| k.word.contains("service")));
    }
}
