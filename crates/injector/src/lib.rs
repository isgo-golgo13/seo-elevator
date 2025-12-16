//! # site-ranker-injector
//!
//! SEO metadata injection engine for site-ranker-rs.
//! Implements Strategy pattern for extensible injectors.
//!
//! ## Supported Injections
//! - Meta tags (title, description, keywords)
//! - Open Graph tags (Facebook, LinkedIn)
//! - Twitter Cards
//! - Schema.org JSON-LD structured data

mod error;
mod strategies;
mod types;

pub use error::InjectorError;
pub use strategies::*;
pub use types::*;

use site_ranker_analyzer::AnalysisResult;

/// Core trait for all injector strategies.
/// Implement this to create custom injectors.
pub trait InjectorStrategy: Send + Sync {
    /// Unique identifier for this strategy
    fn name(&self) -> &'static str;

    /// Generate SEO content to inject
    fn generate(&self, analysis: &AnalysisResult, config: &SeoConfig) -> Result<String, InjectorError>;

    /// Inject generated content into HTML
    fn inject(&self, html: &str, analysis: &AnalysisResult, config: &SeoConfig) -> Result<String, InjectorError> {
        let content = self.generate(analysis, config)?;
        self.inject_content(html, &content)
    }

    /// Inject pre-generated content into HTML
    fn inject_content(&self, html: &str, content: &str) -> Result<String, InjectorError>;
}

/// Boxed injector for runtime polymorphism
pub type BoxedInjector = Box<dyn InjectorStrategy>;

/// Pipeline to compose multiple injectors
pub struct InjectorPipeline {
    injectors: Vec<BoxedInjector>,
}

impl InjectorPipeline {
    /// Create empty pipeline
    pub fn new() -> Self {
        Self { injectors: Vec::new() }
    }

    /// Create pipeline with default injectors
    pub fn default_pipeline() -> Self {
        let mut pipeline = Self::new();
        pipeline.add(Box::new(MetaTagInjector::new()));
        pipeline.add(Box::new(OpenGraphInjector::new()));
        pipeline.add(Box::new(TwitterCardInjector::new()));
        pipeline.add(Box::new(SchemaOrgInjector::new()));
        pipeline
    }

    /// Add injector to pipeline
    pub fn add(&mut self, injector: BoxedInjector) -> &mut Self {
        self.injectors.push(injector);
        self
    }

    /// Run all injectors sequentially
    pub fn inject(
        &self,
        html: &str,
        analysis: &AnalysisResult,
        config: &SeoConfig,
    ) -> Result<String, InjectorError> {
        let mut result = html.to_string();

        for injector in &self.injectors {
            tracing::debug!("Running injector: {}", injector.name());
            result = injector.inject(&result, analysis, config)?;
        }

        Ok(result)
    }

    /// Generate all SEO content without injecting
    pub fn generate_all(
        &self,
        analysis: &AnalysisResult,
        config: &SeoConfig,
    ) -> Result<GeneratedSeo, InjectorError> {
        let mut generated = GeneratedSeo::default();

        for injector in &self.injectors {
            let content = injector.generate(analysis, config)?;
            match injector.name() {
                "meta_tag_injector" => generated.meta_tags = content,
                "open_graph_injector" => generated.open_graph = content,
                "twitter_card_injector" => generated.twitter_cards = content,
                "schema_org_injector" => generated.schema_org = content,
                _ => {}
            }
        }

        Ok(generated)
    }
}

impl Default for InjectorPipeline {
    fn default() -> Self {
        Self::default_pipeline()
    }
}

/// Helper to find injection point in HTML
pub fn find_head_injection_point(html: &str) -> Option<usize> {
    // Look for </head> tag
    if let Some(pos) = html.to_lowercase().find("</head>") {
        return Some(pos);
    }

    // Fallback: look for <body> tag
    if let Some(pos) = html.to_lowercase().find("<body") {
        return Some(pos);
    }

    None
}

/// Helper to find body injection point for schema
pub fn find_body_end_injection_point(html: &str) -> Option<usize> {
    html.to_lowercase().find("</body>")
}

#[cfg(test)]
mod tests {
    use super::*;
    use site_ranker_analyzer::{AnalyzerPipeline, BusinessType};

    const TEST_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Test Site</title>
</head>
<body>
    <h1>Welcome</h1>
    <p>We offer professional software services.</p>
</body>
</html>"#;

    #[test]
    fn test_pipeline_injection() {
        let analyzer = AnalyzerPipeline::default_pipeline();
        let analysis = analyzer.analyze(TEST_HTML).unwrap();

        let config = SeoConfig {
            site_name: "Test Site".to_string(),
            site_url: "https://example.com".to_string(),
            ..Default::default()
        };

        let injector = InjectorPipeline::default_pipeline();
        let result = injector.inject(TEST_HTML, &analysis, &config).unwrap();

        assert!(result.contains("og:title"));
        assert!(result.contains("twitter:card"));
        assert!(result.contains("application/ld+json"));
    }
}
