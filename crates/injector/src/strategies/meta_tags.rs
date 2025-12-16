//! Meta tags injector - title, description, keywords, canonical

use crate::{find_head_injection_point, InjectorError, InjectorStrategy, SeoConfig};
use site_ranker_analyzer::AnalysisResult;

/// Injector for standard meta tags
pub struct MetaTagInjector;

impl MetaTagInjector {
    pub fn new() -> Self {
        Self
    }

    fn generate_title(&self, analysis: &AnalysisResult, config: &SeoConfig) -> String {
        // Use override if provided
        if let Some(ref title) = config.title_override {
            return self.truncate(title, config.max_title_length);
        }

        // Use existing title if present
        if let Some(ref title) = analysis.existing_seo.title {
            if !title.is_empty() {
                return self.truncate(title, config.max_title_length);
            }
        }

        // Generate from keywords and business type
        let top_keywords: Vec<_> = analysis
            .top_keywords(3)
            .iter()
            .map(|k| capitalize(&k.word))
            .collect();

        let title = if top_keywords.is_empty() {
            config.site_name.clone()
        } else {
            format!("{} | {}", top_keywords.join(" - "), config.site_name)
        };

        self.truncate(&title, config.max_title_length)
    }

    fn generate_description(&self, analysis: &AnalysisResult, config: &SeoConfig) -> String {
        // Use override if provided
        if let Some(ref desc) = config.description_override {
            return self.truncate(desc, config.max_description_length);
        }

        // Use existing description if present
        if let Some(ref desc) = analysis.existing_seo.description {
            if !desc.is_empty() {
                return self.truncate(desc, config.max_description_length);
            }
        }

        // Generate from content summary and keywords
        if let Some(ref summary) = analysis.content_summary {
            return self.truncate(summary, config.max_description_length);
        }

        // Fallback: generate from keywords
        let keywords: Vec<_> = analysis
            .top_keywords(5)
            .iter()
            .map(|k| k.word.clone())
            .collect();

        let desc = format!(
            "{} offers {}. Professional solutions for your needs.",
            config.site_name,
            keywords.join(", ")
        );

        self.truncate(&desc, config.max_description_length)
    }

    fn generate_keywords(&self, analysis: &AnalysisResult, config: &SeoConfig) -> String {
        let mut keywords: Vec<String> = analysis
            .top_keywords(10)
            .iter()
            .map(|k| k.word.clone())
            .collect();

        keywords.extend(config.extra_keywords.clone());
        keywords.dedup();
        keywords.join(", ")
    }

    fn truncate(&self, text: &str, max_len: usize) -> String {
        if text.len() <= max_len {
            return text.to_string();
        }

        // Try to break at word boundary
        let truncated = &text[..max_len];
        if let Some(last_space) = truncated.rfind(' ') {
            format!("{}...", &truncated[..last_space])
        } else {
            format!("{}...", truncated)
        }
    }
}

impl Default for MetaTagInjector {
    fn default() -> Self {
        Self::new()
    }
}

impl InjectorStrategy for MetaTagInjector {
    fn name(&self) -> &'static str {
        "meta_tag_injector"
    }

    fn generate(&self, analysis: &AnalysisResult, config: &SeoConfig) -> Result<String, InjectorError> {
        let title = self.generate_title(analysis, config);
        let description = self.generate_description(analysis, config);
        let keywords = self.generate_keywords(analysis, config);

        let mut tags = Vec::new();

        // Title tag (will replace existing)
        tags.push(format!("    <title>{}</title>", html_escape(&title)));

        // Meta description
        if !analysis.existing_seo.has_description {
            tags.push(format!(
                "    <meta name=\"description\" content=\"{}\">",
                html_escape(&description)
            ));
        }

        // Meta keywords (less important but still used by some engines)
        if !keywords.is_empty() {
            tags.push(format!(
                "    <meta name=\"keywords\" content=\"{}\">",
                html_escape(&keywords)
            ));
        }

        // Canonical URL
        if config.generate_canonical && !analysis.existing_seo.has_canonical {
            tags.push(format!(
                "    <link rel=\"canonical\" href=\"{}\">",
                html_escape(&config.site_url)
            ));
        }

        // Viewport (if missing)
        if !analysis.existing_seo.has_viewport {
            tags.push("    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">".to_string());
        }

        // Charset (if missing)
        if !analysis.existing_seo.has_charset {
            tags.push("    <meta charset=\"UTF-8\">".to_string());
        }

        // Robots
        tags.push("    <meta name=\"robots\" content=\"index, follow\">".to_string());

        Ok(tags.join("\n"))
    }

    fn inject_content(&self, html: &str, content: &str) -> Result<String, InjectorError> {
        let injection_point = find_head_injection_point(html)
            .ok_or(InjectorError::NoInjectionPoint)?;

        // Check if we need to replace existing title
        let mut result = html.to_string();

        // Remove existing title if present (we'll add our optimized one)
        let title_regex = regex::Regex::new(r"<title>[^<]*</title>").unwrap();
        result = title_regex.replace(&result, "").to_string();

        // Find new injection point after potential title removal
        let injection_point = find_head_injection_point(&result)
            .ok_or(InjectorError::NoInjectionPoint)?;

        // Insert new meta tags
        let before = &result[..injection_point];
        let after = &result[injection_point..];

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

/// Capitalize first letter
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}
