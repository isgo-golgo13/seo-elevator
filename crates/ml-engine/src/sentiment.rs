//! Sentiment analysis for SEO content optimization
//!
//! ## Why Sentiment Matters for SEO
//!
//! - Positive sentiment in titles/descriptions increases CTR
//! - Emotional triggers drive engagement
//! - Google measures user engagement as a ranking signal
//!
//! ## Implementation
//!
//! Currently uses rule-based analysis with comprehensive word lists.
//! PyTorch integration available via `torch` feature for deep learning models.

use crate::{MlEngineError, MlResult, MlStrategy};
use site_ranker_analyzer::AnalysisResult;
use std::collections::HashSet;

/// Result of sentiment analysis
#[derive(Debug, Clone)]
pub struct SentimentResult {
    /// Sentiment score (-1.0 = negative, 0 = neutral, 1.0 = positive)
    pub score: f32,

    /// Confidence in the score (0.0 - 1.0)
    pub confidence: f32,

    /// Detected emotional triggers
    pub emotional_triggers: Vec<String>,

    /// Power words found
    pub power_words: Vec<String>,

    /// Negative words found (to potentially replace)
    pub negative_words: Vec<String>,

    /// Overall sentiment label
    pub label: SentimentLabel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SentimentLabel {
    VeryNegative,
    Negative,
    Neutral,
    Positive,
    VeryPositive,
}

impl SentimentLabel {
    pub fn from_score(score: f32) -> Self {
        match score {
            s if s < -0.6 => Self::VeryNegative,
            s if s < -0.2 => Self::Negative,
            s if s < 0.2 => Self::Neutral,
            s if s < 0.6 => Self::Positive,
            _ => Self::VeryPositive,
        }
    }
}

/// Rule-based sentiment analyzer
pub struct SentimentAnalyzer {
    positive_words: HashSet<&'static str>,
    negative_words: HashSet<&'static str>,
    power_words: HashSet<&'static str>,
    emotional_triggers: HashSet<&'static str>,
}

impl SentimentAnalyzer {
    pub fn new() -> Self {
        Self {
            positive_words: Self::build_positive_words(),
            negative_words: Self::build_negative_words(),
            power_words: Self::build_power_words(),
            emotional_triggers: Self::build_emotional_triggers(),
        }
    }

    fn build_positive_words() -> HashSet<&'static str> {
        [
            "amazing", "awesome", "best", "brilliant", "excellent", "exceptional",
            "fantastic", "great", "incredible", "outstanding", "perfect", "remarkable",
            "stunning", "superb", "wonderful", "beautiful", "elegant", "impressive",
            "innovative", "professional", "quality", "reliable", "successful", "trusted",
            "valuable", "premium", "exclusive", "leading", "proven", "guaranteed",
            "certified", "award-winning", "top-rated", "highly-rated", "recommended",
            "popular", "favorite", "loved", "easy", "simple", "fast", "quick", "instant",
            "free", "save", "discount", "affordable", "efficient", "effective",
            "powerful", "advanced", "modern", "cutting-edge", "revolutionary",
        ].into_iter().collect()
    }

    fn build_negative_words() -> HashSet<&'static str> {
        [
            "bad", "terrible", "awful", "horrible", "poor", "worst", "disappointing",
            "frustrating", "annoying", "difficult", "complicated", "confusing",
            "expensive", "overpriced", "slow", "broken", "failed", "error", "problem",
            "issue", "bug", "crash", "spam", "scam", "fake", "cheap", "low-quality",
            "unreliable", "risky", "dangerous", "harmful", "boring", "ugly", "outdated",
        ].into_iter().collect()
    }

    fn build_power_words() -> HashSet<&'static str> {
        [
            // Urgency
            "now", "today", "instant", "immediately", "hurry", "limited", "deadline",
            "last-chance", "don't-miss", "act-now", "urgent",
            // Exclusivity
            "exclusive", "premium", "vip", "members-only", "insider", "secret",
            "limited-edition", "rare", "unique", "special",
            // Trust
            "guaranteed", "proven", "certified", "official", "authentic", "verified",
            "trusted", "secure", "safe", "protected", "backed",
            // Value
            "free", "bonus", "save", "discount", "deal", "bargain", "value", "worth",
            "affordable", "budget-friendly",
            // Results
            "results", "success", "achieve", "transform", "improve", "boost", "increase",
            "maximize", "optimize", "accelerate",
        ].into_iter().collect()
    }

    fn build_emotional_triggers() -> HashSet<&'static str> {
        [
            // Fear of missing out
            "don't-miss", "limited-time", "exclusive", "last-chance", "ending-soon",
            // Curiosity
            "discover", "reveal", "secret", "hidden", "surprising", "unexpected",
            "little-known", "insider",
            // Trust
            "proven", "guaranteed", "backed", "certified", "official", "trusted",
            // Desire
            "dream", "imagine", "achieve", "unlock", "transform", "revolutionize",
            // Social proof
            "popular", "trending", "best-selling", "top-rated", "award-winning",
            "recommended", "loved",
        ].into_iter().collect()
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        text.to_lowercase()
            .split(|c: char| !c.is_alphanumeric() && c != '-')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect()
    }

    pub fn analyze_text(&self, text: &str) -> SentimentResult {
        let tokens = self.tokenize(text);
        let total_words = tokens.len() as f32;

        if total_words == 0.0 {
            return SentimentResult {
                score: 0.0,
                confidence: 0.0,
                emotional_triggers: Vec::new(),
                power_words: Vec::new(),
                negative_words: Vec::new(),
                label: SentimentLabel::Neutral,
            };
        }

        let mut positive_count = 0;
        let mut negative_count = 0;
        let mut found_power_words = Vec::new();
        let mut found_emotional_triggers = Vec::new();
        let mut found_negative = Vec::new();

        for token in &tokens {
            if self.positive_words.contains(token.as_str()) {
                positive_count += 1;
            }
            if self.negative_words.contains(token.as_str()) {
                negative_count += 1;
                found_negative.push(token.clone());
            }
            if self.power_words.contains(token.as_str()) {
                found_power_words.push(token.clone());
            }
            if self.emotional_triggers.contains(token.as_str()) {
                found_emotional_triggers.push(token.clone());
            }
        }

        // Calculate sentiment score
        let sentiment_count = positive_count as f32 - negative_count as f32;
        let max_sentiment = (positive_count + negative_count).max(1) as f32;
        let base_score = sentiment_count / max_sentiment;

        // Boost for power words and emotional triggers
        let power_boost = (found_power_words.len() as f32 * 0.05).min(0.2);
        let emotion_boost = (found_emotional_triggers.len() as f32 * 0.03).min(0.15);

        let score = (base_score + power_boost + emotion_boost).clamp(-1.0, 1.0);

        // Calculate confidence
        let sentiment_density = (positive_count + negative_count) as f32 / total_words;
        let confidence = (sentiment_density * 5.0).min(1.0);

        SentimentResult {
            score,
            confidence,
            emotional_triggers: found_emotional_triggers,
            power_words: found_power_words,
            negative_words: found_negative,
            label: SentimentLabel::from_score(score),
        }
    }
}

impl Default for SentimentAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl MlStrategy for SentimentAnalyzer {
    fn name(&self) -> &'static str {
        "sentiment_analyzer"
    }

    fn process(&self, analysis: &AnalysisResult) -> Result<MlResult, MlEngineError> {
        // Analyze raw text content
        let text = analysis.raw_text.as_deref().unwrap_or("");

        // Also analyze title and description
        let title = analysis.existing_seo.title.as_deref().unwrap_or("");
        let description = analysis.existing_seo.description.as_deref().unwrap_or("");

        let combined_text = format!("{} {} {}", title, description, text);
        let sentiment = self.analyze_text(&combined_text);

        let mut result = MlResult::default();
        result.sentiment = Some(sentiment);
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_sentiment() {
        let analyzer = SentimentAnalyzer::new();
        let result = analyzer.analyze_text("This is an amazing, excellent, wonderful product!");

        assert!(result.score > 0.5);
        assert_eq!(result.label, SentimentLabel::Positive);
    }

    #[test]
    fn test_negative_sentiment() {
        let analyzer = SentimentAnalyzer::new();
        let result = analyzer.analyze_text("This is terrible, awful, and disappointing.");

        assert!(result.score < -0.5);
        assert_eq!(result.label, SentimentLabel::Negative);
    }

    #[test]
    fn test_power_words() {
        let analyzer = SentimentAnalyzer::new();
        let result = analyzer.analyze_text("Get exclusive free access now! Limited time offer.");

        assert!(!result.power_words.is_empty());
        assert!(result.power_words.contains(&"exclusive".to_string()));
        assert!(result.power_words.contains(&"free".to_string()));
    }
}
