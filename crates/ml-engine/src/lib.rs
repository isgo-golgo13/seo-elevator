//! # site-ranker-ml-engine
//!
//! ML-powered SEO optimization engine for site-ranker-rs.
//!
//! ## The Mega-Winning Sell
//!
//! This crate provides:
//! - **Sentiment-optimized meta descriptions** - Emotional triggers that boost CTR
//! - **Smart keyword density analysis** - Avoid stuffing, find semantic relationships
//! - **Trend prediction** - Which Schema.org types are gaining SERP features
//! - **Title optimization** - A/B variant suggestions
//!
//! ## Architecture
//! - `MlStrategy` trait defines the contract
//! - Rule-based algorithms work out of the box
//! - PyTorch integration via `torch` feature for deep learning

mod error;
mod sentiment;
mod optimizer;
mod trend;

pub use error::MlEngineError;
pub use sentiment::*;
pub use optimizer::*;
pub use trend::*;

use site_ranker_analyzer::AnalysisResult;

/// Core trait for ML strategies.
/// Implement this to create custom ML-powered analyzers.
pub trait MlStrategy: Send + Sync {
    /// Unique identifier for this strategy
    fn name(&self) -> &'static str;

    /// Process analysis and return ML-enhanced results
    fn process(&self, analysis: &AnalysisResult) -> Result<MlResult, MlEngineError>;
}

/// Boxed ML strategy for runtime polymorphism
pub type BoxedMlStrategy = Box<dyn MlStrategy>;

/// Combined ML processing result
#[derive(Debug, Clone, Default)]
pub struct MlResult {
    /// Sentiment analysis result
    pub sentiment: Option<SentimentResult>,

    /// Optimized title suggestions
    pub title_suggestions: Vec<TitleSuggestion>,

    /// Optimized description suggestions
    pub description_suggestions: Vec<DescriptionSuggestion>,

    /// Keyword optimization results
    pub keyword_analysis: Option<KeywordOptimization>,

    /// Schema trend predictions
    pub schema_trends: Vec<SchemaTrend>,

    /// Overall optimization score (0-100)
    pub optimization_score: u32,

    /// Recommendations for improvement
    pub recommendations: Vec<Recommendation>,
}

impl MlResult {
    pub fn merge(&mut self, other: MlResult) {
        if other.sentiment.is_some() {
            self.sentiment = other.sentiment;
        }
        self.title_suggestions.extend(other.title_suggestions);
        self.description_suggestions.extend(other.description_suggestions);
        if other.keyword_analysis.is_some() {
            self.keyword_analysis = other.keyword_analysis;
        }
        self.schema_trends.extend(other.schema_trends);
        self.recommendations.extend(other.recommendations);

        // Average optimization scores
        if other.optimization_score > 0 {
            self.optimization_score = (self.optimization_score + other.optimization_score) / 2;
        }
    }
}

/// Title optimization suggestion
#[derive(Debug, Clone)]
pub struct TitleSuggestion {
    pub text: String,
    pub score: f32,
    pub reasoning: String,
}

/// Description optimization suggestion
#[derive(Debug, Clone)]
pub struct DescriptionSuggestion {
    pub text: String,
    pub score: f32,
    pub emotional_triggers: Vec<String>,
    pub cta_included: bool,
}

/// Recommendation for SEO improvement
#[derive(Debug, Clone)]
pub struct Recommendation {
    pub category: RecommendationCategory,
    pub priority: Priority,
    pub message: String,
    pub action: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecommendationCategory {
    Title,
    Description,
    Keywords,
    Schema,
    Performance,
    Social,
    Technical,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// ML Engine pipeline that composes multiple strategies
pub struct MlEngine {
    strategies: Vec<BoxedMlStrategy>,
}

impl MlEngine {
    /// Create empty engine
    pub fn new() -> Self {
        Self { strategies: Vec::new() }
    }

    /// Create engine with default strategies
    pub fn default_engine() -> Self {
        let mut engine = Self::new();
        engine.add(Box::new(SentimentAnalyzer::new()));
        engine.add(Box::new(ContentOptimizer::new()));
        engine.add(Box::new(TrendPredictor::new()));
        engine
    }

    /// Add strategy to engine
    pub fn add(&mut self, strategy: BoxedMlStrategy) -> &mut Self {
        self.strategies.push(strategy);
        self
    }

    /// Process analysis through all strategies
    pub fn process(&self, analysis: &AnalysisResult) -> Result<MlResult, MlEngineError> {
        let mut result = MlResult::default();

        for strategy in &self.strategies {
            tracing::debug!("Running ML strategy: {}", strategy.name());
            let ml_result = strategy.process(analysis)?;
            result.merge(ml_result);
        }

        // Calculate final optimization score
        result.optimization_score = self.calculate_final_score(&result, analysis);

        // Generate final recommendations
        self.generate_recommendations(&mut result, analysis);

        Ok(result)
    }

    fn calculate_final_score(&self, ml_result: &MlResult, analysis: &AnalysisResult) -> u32 {
        let mut score = 0u32;

        // SEO completeness (40%)
        score += analysis.existing_seo.completeness_score() * 40 / 100;

        // Sentiment positivity (20%)
        if let Some(ref sentiment) = ml_result.sentiment {
            let sentiment_score = ((sentiment.score + 1.0) / 2.0 * 20.0) as u32;
            score += sentiment_score;
        }

        // Keyword quality (20%)
        if let Some(ref kw) = ml_result.keyword_analysis {
            score += (kw.density_score * 20.0) as u32;
        }

        // Content quality (20%)
        let keyword_count = analysis.keywords.len();
        let content_score = if keyword_count >= 10 { 20 } else { keyword_count as u32 * 2 };
        score += content_score;

        score.min(100)
    }

    fn generate_recommendations(&self, result: &mut MlResult, analysis: &AnalysisResult) {
        // Title recommendations
        if !analysis.existing_seo.has_title {
            result.recommendations.push(Recommendation {
                category: RecommendationCategory::Title,
                priority: Priority::Critical,
                message: "Missing title tag".to_string(),
                action: "Add a descriptive title tag (50-60 characters)".to_string(),
            });
        } else if let Some(ref title) = analysis.existing_seo.title {
            if title.len() < 30 {
                result.recommendations.push(Recommendation {
                    category: RecommendationCategory::Title,
                    priority: Priority::Medium,
                    message: "Title too short".to_string(),
                    action: "Expand title to 50-60 characters for better CTR".to_string(),
                });
            } else if title.len() > 60 {
                result.recommendations.push(Recommendation {
                    category: RecommendationCategory::Title,
                    priority: Priority::Medium,
                    message: "Title too long".to_string(),
                    action: "Shorten title to 60 characters to avoid truncation".to_string(),
                });
            }
        }

        // Description recommendations
        if !analysis.existing_seo.has_description {
            result.recommendations.push(Recommendation {
                category: RecommendationCategory::Description,
                priority: Priority::Critical,
                message: "Missing meta description".to_string(),
                action: "Add a compelling meta description (150-160 characters)".to_string(),
            });
        }

        // Schema recommendations
        if !analysis.existing_seo.has_schema {
            result.recommendations.push(Recommendation {
                category: RecommendationCategory::Schema,
                priority: Priority::High,
                message: "Missing Schema.org structured data".to_string(),
                action: "Add JSON-LD schema for rich snippets in search results".to_string(),
            });
        }

        // Social recommendations
        if !analysis.existing_seo.has_og_tags {
            result.recommendations.push(Recommendation {
                category: RecommendationCategory::Social,
                priority: Priority::High,
                message: "Missing Open Graph tags".to_string(),
                action: "Add OG tags for better social media sharing".to_string(),
            });
        }

        if !analysis.existing_seo.has_twitter_cards {
            result.recommendations.push(Recommendation {
                category: RecommendationCategory::Social,
                priority: Priority::Medium,
                message: "Missing Twitter Cards".to_string(),
                action: "Add Twitter Card meta tags for better Twitter previews".to_string(),
            });
        }

        // Technical recommendations
        if analysis.existing_seo.h1_count == 0 {
            result.recommendations.push(Recommendation {
                category: RecommendationCategory::Technical,
                priority: Priority::High,
                message: "Missing H1 heading".to_string(),
                action: "Add exactly one H1 heading per page".to_string(),
            });
        } else if analysis.existing_seo.h1_count > 1 {
            result.recommendations.push(Recommendation {
                category: RecommendationCategory::Technical,
                priority: Priority::Medium,
                message: format!("Multiple H1 headings ({})", analysis.existing_seo.h1_count),
                action: "Use only one H1 heading per page".to_string(),
            });
        }

        if analysis.existing_seo.img_without_alt > 0 {
            result.recommendations.push(Recommendation {
                category: RecommendationCategory::Technical,
                priority: Priority::Medium,
                message: format!("{} images missing alt text", analysis.existing_seo.img_without_alt),
                action: "Add descriptive alt text to all images".to_string(),
            });
        }

        // Sort by priority
        result.recommendations.sort_by(|a, b| b.priority.cmp(&a.priority));
    }
}

impl Default for MlEngine {
    fn default() -> Self {
        Self::default_engine()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ml_engine() {
        let analysis = AnalysisResult::default();
        let engine = MlEngine::default_engine();
        let result = engine.process(&analysis).unwrap();

        assert!(!result.recommendations.is_empty());
    }
}
