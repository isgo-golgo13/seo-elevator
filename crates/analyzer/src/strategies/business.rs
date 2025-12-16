//! Business type detection analyzer

use crate::{AnalysisResult, AnalyzerError, AnalyzerStrategy, BusinessType};
use scraper::{Html, Selector};
use std::collections::HashMap;

/// Analyzer that detects the type of business/website
pub struct BusinessTypeAnalyzer {
    type_indicators: HashMap<BusinessType, Vec<&'static str>>,
}

impl BusinessTypeAnalyzer {
    pub fn new() -> Self {
        let mut indicators: HashMap<BusinessType, Vec<&'static str>> = HashMap::new();

        indicators.insert(
            BusinessType::Ecommerce,
            vec![
                "cart", "checkout", "buy", "shop", "store", "product", "price",
                "add to cart", "purchase", "order", "shipping", "payment",
                "catalog", "inventory", "sale", "discount", "coupon",
            ],
        );

        indicators.insert(
            BusinessType::SaaS,
            vec![
                "saas", "software", "platform", "dashboard", "api", "integration",
                "subscription", "trial", "demo", "features", "pricing", "plans",
                "enterprise", "startup", "cloud", "automation", "workflow",
            ],
        );

        indicators.insert(
            BusinessType::Blog,
            vec![
                "blog", "article", "post", "author", "published", "read more",
                "comments", "tags", "category", "archive", "recent posts",
            ],
        );

        indicators.insert(
            BusinessType::Portfolio,
            vec![
                "portfolio", "projects", "work", "case study", "client",
                "designer", "developer", "freelance", "hire me", "about me",
            ],
        );

        indicators.insert(
            BusinessType::Service,
            vec![
                "service", "consulting", "solutions", "expertise", "professional",
                "team", "approach", "methodology", "process", "engagement",
                "migration", "assessment", "audit", "implementation",
            ],
        );

        indicators.insert(
            BusinessType::Agency,
            vec![
                "agency", "creative", "marketing", "branding", "campaigns",
                "clients", "results", "strategy", "digital", "media",
            ],
        );

        indicators.insert(
            BusinessType::LocalBusiness,
            vec![
                "location", "address", "hours", "visit us", "directions",
                "local", "near", "store hours", "call us", "contact",
            ],
        );

        indicators.insert(
            BusinessType::Restaurant,
            vec![
                "menu", "restaurant", "dining", "reservation", "food",
                "cuisine", "chef", "table", "delivery", "takeout", "order online",
            ],
        );

        indicators.insert(
            BusinessType::Education,
            vec![
                "course", "learn", "student", "teacher", "education",
                "training", "curriculum", "enroll", "class", "lesson",
                "certification", "degree", "workshop",
            ],
        );

        indicators.insert(
            BusinessType::Healthcare,
            vec![
                "health", "medical", "doctor", "patient", "clinic",
                "hospital", "treatment", "appointment", "care", "wellness",
                "diagnosis", "symptoms", "therapy",
            ],
        );

        indicators.insert(
            BusinessType::RealEstate,
            vec![
                "property", "real estate", "home", "house", "apartment",
                "listing", "rent", "sale", "mortgage", "agent", "broker",
                "bedroom", "bathroom", "sqft",
            ],
        );

        indicators.insert(
            BusinessType::Technology,
            vec![
                "technology", "tech", "innovation", "engineering", "development",
                "infrastructure", "security", "data", "ai", "machine learning",
                "blockchain", "cloud computing", "devops",
            ],
        );

        indicators.insert(
            BusinessType::NonProfit,
            vec![
                "nonprofit", "charity", "donate", "volunteer", "mission",
                "cause", "foundation", "community", "impact", "support",
            ],
        );

        Self { type_indicators: indicators }
    }

    fn extract_all_text(&self, html: &str) -> String {
        let document = Html::parse_document(html);
        let mut text = String::new();

        // Get title
        if let Ok(selector) = Selector::parse("title") {
            if let Some(el) = document.select(&selector).next() {
                text.push_str(&el.text().collect::<String>());
                text.push(' ');
            }
        }

        // Get meta description
        if let Ok(selector) = Selector::parse("meta[name='description']") {
            if let Some(el) = document.select(&selector).next() {
                if let Some(content) = el.value().attr("content") {
                    text.push_str(content);
                    text.push(' ');
                }
            }
        }

        // Get headings
        for tag in ["h1", "h2", "h3"] {
            if let Ok(selector) = Selector::parse(tag) {
                for el in document.select(&selector) {
                    text.push_str(&el.text().collect::<String>());
                    text.push(' ');
                }
            }
        }

        // Get nav links
        if let Ok(selector) = Selector::parse("nav a, header a") {
            for el in document.select(&selector) {
                text.push_str(&el.text().collect::<String>());
                text.push(' ');
            }
        }

        // Get main content
        if let Ok(selector) = Selector::parse("main, article, section, .content") {
            for el in document.select(&selector) {
                text.push_str(&el.text().collect::<String>());
                text.push(' ');
            }
        }

        text.to_lowercase()
    }

    fn detect_language(&self, html: &str) -> Option<String> {
        let document = Html::parse_document(html);

        // Check html lang attribute
        if let Ok(selector) = Selector::parse("html") {
            if let Some(el) = document.select(&selector).next() {
                if let Some(lang) = el.value().attr("lang") {
                    return Some(lang.split('-').next().unwrap_or(lang).to_string());
                }
            }
        }

        // Check meta content-language
        if let Ok(selector) = Selector::parse("meta[http-equiv='content-language']") {
            if let Some(el) = document.select(&selector).next() {
                if let Some(content) = el.value().attr("content") {
                    return Some(content.split('-').next().unwrap_or(content).to_string());
                }
            }
        }

        None
    }
}

impl Default for BusinessTypeAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalyzerStrategy for BusinessTypeAnalyzer {
    fn name(&self) -> &'static str {
        "business_type_analyzer"
    }

    fn analyze(&self, content: &str) -> Result<AnalysisResult, AnalyzerError> {
        let text = self.extract_all_text(content);
        let language = self.detect_language(content);

        // Score each business type
        let mut scores: HashMap<BusinessType, u32> = HashMap::new();

        for (biz_type, indicators) in &self.type_indicators {
            let mut score = 0u32;
            for indicator in indicators {
                if text.contains(indicator) {
                    score += 1;
                    // Bonus for multiple occurrences
                    score += text.matches(indicator).count().saturating_sub(1) as u32 / 2;
                }
            }
            if score > 0 {
                scores.insert(biz_type.clone(), score);
            }
        }

        // Find highest scoring type
        let business_type = scores
            .into_iter()
            .max_by_key(|(_, score)| *score)
            .map(|(biz_type, _)| biz_type)
            .unwrap_or(BusinessType::Unknown);

        // Generate content summary
        let content_summary = self.generate_summary(&text);

        Ok(AnalysisResult {
            business_type,
            language,
            content_summary: Some(content_summary),
            ..Default::default()
        })
    }
}

impl BusinessTypeAnalyzer {
    fn generate_summary(&self, text: &str) -> String {
        // Extract first meaningful sentences
        let sentences: Vec<&str> = text
            .split(|c| c == '.' || c == '!' || c == '?')
            .map(|s| s.trim())
            .filter(|s| s.len() > 20 && s.len() < 200)
            .take(3)
            .collect();

        sentences.join(". ")
    }
}
