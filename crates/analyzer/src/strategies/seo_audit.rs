//! SEO audit analyzer - checks existing SEO elements

use crate::{AnalysisResult, AnalyzerError, AnalyzerStrategy, ExistingSeo};
use scraper::{Html, Selector};

/// Analyzer that audits existing SEO elements
pub struct SeoAuditAnalyzer;

impl SeoAuditAnalyzer {
    pub fn new() -> Self {
        Self
    }

    fn check_selector(document: &Html, selector_str: &str) -> bool {
        Selector::parse(selector_str)
            .ok()
            .map(|sel| document.select(&sel).next().is_some())
            .unwrap_or(false)
    }

    fn get_attr(document: &Html, selector_str: &str, attr: &str) -> Option<String> {
        Selector::parse(selector_str).ok().and_then(|sel| {
            document
                .select(&sel)
                .next()
                .and_then(|el| el.value().attr(attr).map(String::from))
        })
    }

    fn get_text(document: &Html, selector_str: &str) -> Option<String> {
        Selector::parse(selector_str).ok().and_then(|sel| {
            document.select(&sel).next().map(|el| {
                el.text().collect::<String>().trim().to_string()
            })
        })
    }

    fn count_elements(document: &Html, selector_str: &str) -> u32 {
        Selector::parse(selector_str)
            .ok()
            .map(|sel| document.select(&sel).count() as u32)
            .unwrap_or(0)
    }
}

impl Default for SeoAuditAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalyzerStrategy for SeoAuditAnalyzer {
    fn name(&self) -> &'static str {
        "seo_audit_analyzer"
    }

    fn analyze(&self, content: &str) -> Result<AnalysisResult, AnalyzerError> {
        let document = Html::parse_document(content);

        // Check title
        let title = Self::get_text(&document, "title");
        let has_title = title.as_ref().map(|t| !t.is_empty()).unwrap_or(false);

        // Check meta description
        let description = Self::get_attr(&document, "meta[name='description']", "content");
        let has_description = description
            .as_ref()
            .map(|d| !d.is_empty())
            .unwrap_or(false);

        // Check Open Graph tags
        let has_og_tags = Self::check_selector(&document, "meta[property^='og:']");

        // Check Twitter Cards
        let has_twitter_cards = Self::check_selector(&document, "meta[name^='twitter:']");

        // Check Schema.org (JSON-LD)
        let has_schema = Self::check_selector(&document, "script[type='application/ld+json']");

        // Check canonical
        let has_canonical = Self::check_selector(&document, "link[rel='canonical']");

        // Check viewport
        let has_viewport = Self::check_selector(&document, "meta[name='viewport']");

        // Check charset
        let has_charset = Self::check_selector(&document, "meta[charset]")
            || Self::check_selector(&document, "meta[http-equiv='Content-Type']");

        // Count H1 tags
        let h1_count = Self::count_elements(&document, "h1");

        // Count images without alt
        let total_images = Self::count_elements(&document, "img");
        let images_with_alt = Self::count_elements(&document, "img[alt]:not([alt=''])");
        let img_without_alt = total_images.saturating_sub(images_with_alt);

        let existing_seo = ExistingSeo {
            has_title,
            title,
            has_description,
            description,
            has_og_tags,
            has_twitter_cards,
            has_schema,
            has_canonical,
            has_viewport,
            has_charset,
            h1_count,
            img_without_alt,
        };

        Ok(AnalysisResult {
            existing_seo,
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seo_audit_complete() {
        let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="description" content="Test description">
    <meta property="og:title" content="OG Title">
    <meta name="twitter:card" content="summary">
    <link rel="canonical" href="https://example.com">
    <title>Test Page</title>
    <script type="application/ld+json">{"@type": "Organization"}</script>
</head>
<body>
    <h1>Main Heading</h1>
    <img src="test.jpg" alt="Test image">
</body>
</html>
        "#;

        let analyzer = SeoAuditAnalyzer::new();
        let result = analyzer.analyze(html).unwrap();

        assert!(result.existing_seo.has_title);
        assert!(result.existing_seo.has_description);
        assert!(result.existing_seo.has_og_tags);
        assert!(result.existing_seo.has_twitter_cards);
        assert!(result.existing_seo.has_schema);
        assert!(result.existing_seo.has_canonical);
        assert!(result.existing_seo.has_viewport);
        assert!(result.existing_seo.has_charset);
        assert_eq!(result.existing_seo.h1_count, 1);
        assert_eq!(result.existing_seo.img_without_alt, 0);
        assert_eq!(result.existing_seo.completeness_score(), 100);
    }

    #[test]
    fn test_seo_audit_incomplete() {
        let html = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Basic Page</title>
</head>
<body>
    <h1>Hello</h1>
    <h1>Another H1</h1>
    <img src="no-alt.jpg">
</body>
</html>
        "#;

        let analyzer = SeoAuditAnalyzer::new();
        let result = analyzer.analyze(html).unwrap();

        assert!(result.existing_seo.has_title);
        assert!(!result.existing_seo.has_description);
        assert!(!result.existing_seo.has_og_tags);
        assert_eq!(result.existing_seo.h1_count, 2);
        assert_eq!(result.existing_seo.img_without_alt, 1);
    }
}
