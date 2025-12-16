//! Twitter Cards injector

use crate::{find_head_injection_point, InjectorError, InjectorStrategy, SeoConfig};
use site_ranker_analyzer::AnalysisResult;

/// Injector for Twitter Card meta tags
pub struct TwitterCardInjector;

impl TwitterCardInjector {
    pub fn new() -> Self {
        Self
    }

    fn get_card_type(config: &SeoConfig) -> &'static str {
        // Use summary_large_image if we have an image, otherwise summary
        if config.default_image.is_some() {
            "summary_large_image"
        } else {
            "summary"
        }
    }
}

impl Default for TwitterCardInjector {
    fn default() -> Self {
        Self::new()
    }
}

impl InjectorStrategy for TwitterCardInjector {
    fn name(&self) -> &'static str {
        "twitter_card_injector"
    }

    fn generate(&self, analysis: &AnalysisResult, config: &SeoConfig) -> Result<String, InjectorError> {
        // Skip if already has Twitter cards
        if analysis.existing_seo.has_twitter_cards {
            return Ok(String::new());
        }

        let title = config
            .title_override
            .clone()
            .or_else(|| analysis.existing_seo.title.clone())
            .unwrap_or_else(|| config.site_name.clone());

        let description = config
            .description_override
            .clone()
            .or_else(|| analysis.existing_seo.description.clone())
            .or_else(|| analysis.content_summary.clone())
            .unwrap_or_else(|| {
                let keywords: Vec<_> = analysis
                    .top_keywords(5)
                    .iter()
                    .map(|k| k.word.clone())
                    .collect();
                format!("{} - {}", config.site_name, keywords.join(", "))
            });

        let card_type = Self::get_card_type(config);

        let mut tags = vec![
            format!("    <meta name=\"twitter:card\" content=\"{}\">", card_type),
            format!(
                "    <meta name=\"twitter:title\" content=\"{}\">",
                html_escape(&truncate(&title, 70))
            ),
            format!(
                "    <meta name=\"twitter:description\" content=\"{}\">",
                html_escape(&truncate(&description, 200))
            ),
        ];

        // Add Twitter handle if provided
        if let Some(ref handle) = config.twitter_handle {
            let handle = if handle.starts_with('@') {
                handle.clone()
            } else {
                format!("@{}", handle)
            };
            tags.push(format!(
                "    <meta name=\"twitter:site\" content=\"{}\">",
                html_escape(&handle)
            ));
            tags.push(format!(
                "    <meta name=\"twitter:creator\" content=\"{}\">",
                html_escape(&handle)
            ));
        }

        // Add image if provided
        if let Some(ref image) = config.default_image {
            tags.push(format!(
                "    <meta name=\"twitter:image\" content=\"{}\">",
                html_escape(image)
            ));
            tags.push(format!(
                "    <meta name=\"twitter:image:alt\" content=\"{}\">",
                html_escape(&title)
            ));
        }

        Ok(tags.join("\n"))
    }

    fn inject_content(&self, html: &str, content: &str) -> Result<String, InjectorError> {
        if content.is_empty() {
            return Ok(html.to_string());
        }

        let injection_point = find_head_injection_point(html)
            .ok_or(InjectorError::NoInjectionPoint)?;

        let before = &html[..injection_point];
        let after = &html[injection_point..];

        Ok(format!("{}\n{}\n{}", before, content, after))
    }
}

/// HTML escape helper
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Truncate text to max length
fn truncate(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        return text.to_string();
    }

    let truncated = &text[..max_len];
    if let Some(last_space) = truncated.rfind(' ') {
        format!("{}...", &truncated[..last_space])
    } else {
        format!("{}...", truncated)
    }
}
