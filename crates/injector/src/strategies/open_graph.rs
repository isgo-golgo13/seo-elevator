//! Open Graph injector - Facebook, LinkedIn, etc.

use crate::{find_head_injection_point, InjectorError, InjectorStrategy, SeoConfig};
use site_ranker_analyzer::AnalysisResult;

/// Injector for Open Graph meta tags
pub struct OpenGraphInjector;

impl OpenGraphInjector {
    pub fn new() -> Self {
        Self
    }

    fn get_og_type(analysis: &AnalysisResult) -> &'static str {
        use site_ranker_analyzer::BusinessType;

        match analysis.business_type {
            BusinessType::Blog => "article",
            BusinessType::Ecommerce => "product",
            BusinessType::Restaurant => "restaurant",
            _ => "website",
        }
    }
}

impl Default for OpenGraphInjector {
    fn default() -> Self {
        Self::new()
    }
}

impl InjectorStrategy for OpenGraphInjector {
    fn name(&self) -> &'static str {
        "open_graph_injector"
    }

    fn generate(&self, analysis: &AnalysisResult, config: &SeoConfig) -> Result<String, InjectorError> {
        // Skip if already has OG tags
        if analysis.existing_seo.has_og_tags {
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

        let og_type = Self::get_og_type(analysis);

        let mut tags = vec![
            format!("    <meta property=\"og:type\" content=\"{}\">", og_type),
            format!(
                "    <meta property=\"og:title\" content=\"{}\">",
                html_escape(&title)
            ),
            format!(
                "    <meta property=\"og:description\" content=\"{}\">",
                html_escape(&truncate(&description, 200))
            ),
            format!(
                "    <meta property=\"og:url\" content=\"{}\">",
                html_escape(&config.site_url)
            ),
            format!(
                "    <meta property=\"og:site_name\" content=\"{}\">",
                html_escape(&config.site_name)
            ),
            format!(
                "    <meta property=\"og:locale\" content=\"{}\">",
                config.locale
            ),
        ];

        // Add image if provided
        if let Some(ref image) = config.default_image {
            tags.push(format!(
                "    <meta property=\"og:image\" content=\"{}\">",
                html_escape(image)
            ));
            tags.push("    <meta property=\"og:image:width\" content=\"1200\">".to_string());
            tags.push("    <meta property=\"og:image:height\" content=\"630\">".to_string());
        }

        // Add Facebook App ID if provided
        if let Some(ref app_id) = config.facebook_app_id {
            tags.push(format!(
                "    <meta property=\"fb:app_id\" content=\"{}\">",
                html_escape(app_id)
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
