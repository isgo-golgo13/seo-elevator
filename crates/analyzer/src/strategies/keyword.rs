//! Keyword extraction and scoring analyzer

use crate::{AnalysisResult, AnalyzerError, AnalyzerStrategy, Keyword};
use regex::Regex;
use scraper::{Html, Selector};
use std::collections::HashMap;

/// Analyzer that extracts keywords from HTML content
pub struct KeywordAnalyzer {
    stop_words: Vec<&'static str>,
    min_word_length: usize,
    max_keywords: usize,
}

impl KeywordAnalyzer {
    pub fn new() -> Self {
        Self {
            stop_words: vec![
                "the", "a", "an", "and", "or", "but", "in", "on", "at", "to", "for",
                "of", "with", "by", "from", "as", "is", "was", "are", "were", "been",
                "be", "have", "has", "had", "do", "does", "did", "will", "would",
                "could", "should", "may", "might", "must", "shall", "can", "need",
                "this", "that", "these", "those", "i", "you", "he", "she", "it",
                "we", "they", "what", "which", "who", "when", "where", "why", "how",
                "all", "each", "every", "both", "few", "more", "most", "other",
                "some", "such", "no", "nor", "not", "only", "own", "same", "so",
                "than", "too", "very", "just", "also", "now", "here", "there",
                "then", "once", "any", "about", "into", "through", "during",
                "before", "after", "above", "below", "between", "under", "again",
                "further", "because", "if", "else", "until", "while", "our", "your",
            ],
            min_word_length: 3,
            max_keywords: 50,
        }
    }

    fn extract_text(&self, html: &str) -> String {
        let document = Html::parse_document(html);

        // Remove script and style content
        let body_selector = Selector::parse("body").unwrap();
        let script_selector = Selector::parse("script, style, noscript").unwrap();

        let mut text = String::new();

        if let Some(body) = document.select(&body_selector).next() {
            for node in body.descendants() {
                if let Some(element) = node.value().as_element() {
                    // Skip script/style elements
                    if script_selector.matches(&scraper::ElementRef::wrap(node).unwrap()) {
                        continue;
                    }
                }
                if let Some(text_node) = node.value().as_text() {
                    text.push_str(text_node);
                    text.push(' ');
                }
            }
        }

        // Also extract from title and meta
        let title_selector = Selector::parse("title").unwrap();
        if let Some(title) = document.select(&title_selector).next() {
            text.push_str(&title.text().collect::<String>());
            text.push(' ');
        }

        let meta_selector = Selector::parse("meta[name='description']").unwrap();
        if let Some(meta) = document.select(&meta_selector).next() {
            if let Some(content) = meta.value().attr("content") {
                text.push_str(content);
            }
        }

        text
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        let word_regex = Regex::new(r"[a-zA-Z]+").unwrap();

        word_regex
            .find_iter(text)
            .map(|m| m.as_str().to_lowercase())
            .filter(|w| {
                w.len() >= self.min_word_length && !self.stop_words.contains(&w.as_str())
            })
            .collect()
    }

    fn extract_phrases(&self, text: &str) -> Vec<String> {
        let phrase_regex = Regex::new(r"[A-Z][a-z]+(?:\s+[A-Z][a-z]+)+").unwrap();

        phrase_regex
            .find_iter(text)
            .map(|m| m.as_str().to_lowercase())
            .filter(|p| p.split_whitespace().count() <= 4)
            .collect()
    }

    fn calculate_scores(&self, word_counts: HashMap<String, u32>, total_words: usize) -> Vec<Keyword> {
        let mut keywords: Vec<Keyword> = word_counts
            .into_iter()
            .map(|(word, frequency)| {
                // TF-IDF inspired scoring
                let tf = frequency as f32 / total_words.max(1) as f32;
                let length_bonus = (word.len() as f32 / 10.0).min(1.0);
                let score = tf * 100.0 * (1.0 + length_bonus);

                Keyword {
                    word,
                    frequency,
                    score,
                    is_phrase: false,
                }
            })
            .collect();

        keywords.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        keywords.truncate(self.max_keywords);
        keywords
    }
}

impl Default for KeywordAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalyzerStrategy for KeywordAnalyzer {
    fn name(&self) -> &'static str {
        "keyword_analyzer"
    }

    fn analyze(&self, content: &str) -> Result<AnalysisResult, AnalyzerError> {
        let text = self.extract_text(content);
        let words = self.tokenize(&text);
        let phrases = self.extract_phrases(&text);

        // Count word frequencies
        let mut word_counts: HashMap<String, u32> = HashMap::new();
        for word in &words {
            *word_counts.entry(word.clone()).or_insert(0) += 1;
        }

        // Count phrase frequencies
        let mut phrase_counts: HashMap<String, u32> = HashMap::new();
        for phrase in &phrases {
            *phrase_counts.entry(phrase.clone()).or_insert(0) += 1;
        }

        let total_words = words.len();
        let mut keywords = self.calculate_scores(word_counts, total_words);

        // Add top phrases
        let mut phrase_keywords: Vec<Keyword> = phrase_counts
            .into_iter()
            .map(|(phrase, frequency)| Keyword {
                word: phrase,
                frequency,
                score: frequency as f32 * 5.0, // Boost phrases
                is_phrase: true,
            })
            .collect();

        phrase_keywords.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        keywords.extend(phrase_keywords.into_iter().take(10));

        Ok(AnalysisResult {
            keywords,
            raw_text: Some(text),
            ..Default::default()
        })
    }
}
