//! Content optimization for SEO
//!
//! Optimizes titles and descriptions for maximum CTR.
//! Provides keyword density analysis and suggestions.

use crate::{
    DescriptionSuggestion, MlEngineError, MlResult, MlStrategy,
    TitleSuggestion,
};
use site_ranker_analyzer::AnalysisResult;
use chrono::Utc;

/// Keyword optimization analysis
#[derive(Debug, Clone)]
pub struct KeywordOptimization {
    /// Keyword density (0.0 - 1.0)
    pub density: f32,

    /// Density score (0.0 = too low/high, 1.0 = optimal)
    pub density_score: f32,

    /// Whether keyword stuffing is detected
    pub is_stuffed: bool,

    /// Recommended keywords to add
    pub recommended_additions: Vec<String>,

    /// Keywords that appear too often
    pub over_used: Vec<String>,
}

/// Content optimizer for titles and descriptions
pub struct ContentOptimizer {
    /// Optimal title length range
    title_length: (usize, usize),
    /// Optimal description length range
    description_length: (usize, usize),
    /// Optimal keyword density range (as percentage)
    keyword_density: (f32, f32),
}

impl ContentOptimizer {
    pub fn new() -> Self {
        Self {
            title_length: (50, 60),
            description_length: (150, 160),
            keyword_density: (1.0, 3.0), // 1-3% is optimal
        }
    }

    fn analyze_keyword_density(&self, analysis: &AnalysisResult) -> KeywordOptimization {
        let text = analysis.raw_text.as_deref().unwrap_or("");
        let word_count = text.split_whitespace().count() as f32;

        if word_count == 0.0 {
            return KeywordOptimization {
                density: 0.0,
                density_score: 0.0,
                is_stuffed: false,
                recommended_additions: Vec::new(),
                over_used: Vec::new(),
            };
        }

        // Calculate total keyword occurrences
        let total_keyword_freq: u32 = analysis.keywords.iter().map(|k| k.frequency).sum();
        let density = (total_keyword_freq as f32 / word_count) * 100.0;

        // Score the density (optimal is 1-3%)
        let density_score = if density < self.keyword_density.0 {
            density / self.keyword_density.0
        } else if density > self.keyword_density.1 {
            1.0 - ((density - self.keyword_density.1) / 10.0).min(1.0)
        } else {
            1.0
        };

        let is_stuffed = density > 5.0;

        // Find over-used keywords (>2% individually)
        let over_used: Vec<String> = analysis
            .keywords
            .iter()
            .filter(|k| (k.frequency as f32 / word_count) * 100.0 > 2.0)
            .map(|k| k.word.clone())
            .collect();

        // Recommend additions if density is low
        let recommended_additions = if density < self.keyword_density.0 {
            analysis
                .top_keywords(3)
                .iter()
                .map(|k| k.word.clone())
                .collect()
        } else {
            Vec::new()
        };

        KeywordOptimization {
            density,
            density_score,
            is_stuffed,
            recommended_additions,
            over_used,
        }
    }

    fn generate_title_suggestions(&self, analysis: &AnalysisResult) -> Vec<TitleSuggestion> {
        let mut suggestions = Vec::new();

        let keywords: Vec<_> = analysis.top_keywords(3).iter().map(|k| &k.word).collect();
        let site_topic = keywords.first().map(|s| capitalize(s)).unwrap_or_default();

        // Pattern 1: Direct benefit
        if !keywords.is_empty() {
            let title = format!(
                "{} - Professional {} Solutions",
                capitalize(keywords[0]),
                if keywords.len() > 1 {
                    capitalize(keywords[1])
                } else {
                    "Business".to_string()
                }
            );
            if title.len() <= 60 {
                suggestions.push(TitleSuggestion {
                    text: title,
                    score: 0.85,
                    reasoning: "Combines primary keyword with benefit-focused language".to_string(),
                });
            }
        }

        // Pattern 2: Question format (curiosity trigger)
        if !site_topic.is_empty() {
            let title = format!("Need {}? Get Expert Help Today", site_topic);
            if title.len() <= 60 {
                suggestions.push(TitleSuggestion {
                    text: title,
                    score: 0.80,
                    reasoning: "Question format triggers curiosity and engagement".to_string(),
                });
            }
        }

        // Pattern 3: List/Number format
        if !site_topic.is_empty() {
            let title = format!("Top {} Services | Trusted Experts", site_topic);
            if title.len() <= 60 {
                suggestions.push(TitleSuggestion {
                    text: title,
                    score: 0.75,
                    reasoning: "Authority positioning with trust signal".to_string(),
                });
            }
        }

        // Pattern 4: Year freshness
        let year = Utc::now().format("%Y");
        if !site_topic.is_empty() {
            let title = format!("{} Guide {} - Expert Resources", site_topic, year);
            if title.len() <= 60 {
                suggestions.push(TitleSuggestion {
                    text: title,
                    score: 0.78,
                    reasoning: "Year signals freshness, improves CTR".to_string(),
                });
            }
        }

        suggestions.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        suggestions
    }

    fn generate_description_suggestions(
        &self,
        analysis: &AnalysisResult,
    ) -> Vec<DescriptionSuggestion> {
        let mut suggestions = Vec::new();

        let keywords: Vec<String> = analysis.top_keywords(5).iter().map(|k| k.word.clone()).collect();

        if keywords.is_empty() {
            return suggestions;
        }

        let default_proven = "proven".to_string();
        
        // Pattern 1: Problem-Solution with CTA
        let desc = format!(
            "Looking for {}? Our {} experts deliver {} results. Get started today with a free consultation.",
            keywords[0],
            keywords.get(1).unwrap_or(&keywords[0]),
            keywords.get(2).unwrap_or(&default_proven)
        );

        if desc.len() <= 160 {
            suggestions.push(DescriptionSuggestion {
                text: desc,
                score: 0.90,
                emotional_triggers: vec!["free".to_string(), "expert".to_string()],
                cta_included: true,
            });
        }

        // Pattern 2: Benefit-focused
        let desc = format!(
            "Transform your {} with our professional {} services. Trusted by businesses worldwide for quality and reliability.",
            keywords[0],
            keywords.get(1).unwrap_or(&keywords[0])
        );

        if desc.len() <= 160 {
            suggestions.push(DescriptionSuggestion {
                text: desc,
                score: 0.85,
                emotional_triggers: vec!["transform".to_string(), "trusted".to_string()],
                cta_included: false,
            });
        }

        // Pattern 3: Social proof
        let desc = format!(
            "Join thousands who trust us for {}. {} solutions backed by expertise and dedication. Contact us now.",
            keywords[0],
            capitalize(keywords.get(1).unwrap_or(&keywords[0]))
        );

        if desc.len() <= 160 {
            suggestions.push(DescriptionSuggestion {
                text: desc,
                score: 0.82,
                emotional_triggers: vec!["trust".to_string(), "join".to_string()],
                cta_included: true,
            });
        }

        suggestions.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        suggestions
    }
}

impl Default for ContentOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

impl MlStrategy for ContentOptimizer {
    fn name(&self) -> &'static str {
        "content_optimizer"
    }

    fn process(&self, analysis: &AnalysisResult) -> Result<MlResult, MlEngineError> {
        let keyword_analysis = self.analyze_keyword_density(analysis);
        let title_suggestions = self.generate_title_suggestions(analysis);
        let description_suggestions = self.generate_description_suggestions(analysis);

        Ok(MlResult {
            keyword_analysis: Some(keyword_analysis),
            title_suggestions,
            description_suggestions,
            ..Default::default()
        })
    }
}

/// Capitalize first letter
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use site_ranker_analyzer::Keyword;

    #[test]
    fn test_title_suggestions() {
        let analysis = AnalysisResult {
            keywords: vec![
                Keyword {
                    word: "software".to_string(),
                    frequency: 10,
                    score: 0.9,
                    is_phrase: false,
                },
                Keyword {
                    word: "development".to_string(),
                    frequency: 8,
                    score: 0.8,
                    is_phrase: false,
                },
            ],
            ..Default::default()
        };

        let optimizer = ContentOptimizer::new();
        let result = optimizer.process(&analysis).unwrap();

        assert!(!result.title_suggestions.is_empty());
    }
}
