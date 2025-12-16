//! Types for SEO injection configuration and output

use serde::{Deserialize, Serialize};

/// Configuration for SEO generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeoConfig {
    /// Site name for branding
    pub site_name: String,

    /// Base URL of the site
    pub site_url: String,

    /// Default image for social sharing
    pub default_image: Option<String>,

    /// Twitter handle (without @)
    pub twitter_handle: Option<String>,

    /// Facebook App ID
    pub facebook_app_id: Option<String>,

    /// Business contact email
    pub contact_email: Option<String>,

    /// Business phone number
    pub phone: Option<String>,

    /// Business address
    pub address: Option<Address>,

    /// Override auto-generated title
    pub title_override: Option<String>,

    /// Override auto-generated description
    pub description_override: Option<String>,

    /// Additional keywords to include
    pub extra_keywords: Vec<String>,

    /// Locale for Open Graph
    pub locale: String,

    /// Generate canonical URL
    pub generate_canonical: bool,

    /// Max description length
    pub max_description_length: usize,

    /// Max title length
    pub max_title_length: usize,
}

impl Default for SeoConfig {
    fn default() -> Self {
        Self {
            site_name: String::new(),
            site_url: String::new(),
            default_image: None,
            twitter_handle: None,
            facebook_app_id: None,
            contact_email: None,
            phone: None,
            address: None,
            title_override: None,
            description_override: None,
            extra_keywords: Vec::new(),
            locale: "en_US".to_string(),
            generate_canonical: true,
            max_description_length: 160,
            max_title_length: 60,
        }
    }
}

impl SeoConfig {
    pub fn builder() -> SeoConfigBuilder {
        SeoConfigBuilder::default()
    }
}

/// Builder for SeoConfig
#[derive(Debug, Default)]
pub struct SeoConfigBuilder {
    config: SeoConfig,
}

impl SeoConfigBuilder {
    pub fn site_name(mut self, name: impl Into<String>) -> Self {
        self.config.site_name = name.into();
        self
    }

    pub fn site_url(mut self, url: impl Into<String>) -> Self {
        self.config.site_url = url.into();
        self
    }

    pub fn default_image(mut self, image: impl Into<String>) -> Self {
        self.config.default_image = Some(image.into());
        self
    }

    pub fn twitter_handle(mut self, handle: impl Into<String>) -> Self {
        self.config.twitter_handle = Some(handle.into());
        self
    }

    pub fn facebook_app_id(mut self, id: impl Into<String>) -> Self {
        self.config.facebook_app_id = Some(id.into());
        self
    }

    pub fn contact_email(mut self, email: impl Into<String>) -> Self {
        self.config.contact_email = Some(email.into());
        self
    }

    pub fn phone(mut self, phone: impl Into<String>) -> Self {
        self.config.phone = Some(phone.into());
        self
    }

    pub fn address(mut self, address: Address) -> Self {
        self.config.address = Some(address);
        self
    }

    pub fn title_override(mut self, title: impl Into<String>) -> Self {
        self.config.title_override = Some(title.into());
        self
    }

    pub fn description_override(mut self, desc: impl Into<String>) -> Self {
        self.config.description_override = Some(desc.into());
        self
    }

    pub fn extra_keywords(mut self, keywords: Vec<String>) -> Self {
        self.config.extra_keywords = keywords;
        self
    }

    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.config.locale = locale.into();
        self
    }

    pub fn generate_canonical(mut self, generate: bool) -> Self {
        self.config.generate_canonical = generate;
        self
    }

    pub fn build(self) -> SeoConfig {
        self.config
    }
}

/// Physical address for Schema.org
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}

/// Generated SEO content
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GeneratedSeo {
    pub meta_tags: String,
    pub open_graph: String,
    pub twitter_cards: String,
    pub schema_org: String,
}

impl GeneratedSeo {
    /// Combine all generated content
    pub fn combined(&self) -> String {
        format!(
            "<!-- SEO Meta Tags -->\n{}\n\n<!-- Open Graph -->\n{}\n\n<!-- Twitter Cards -->\n{}\n\n<!-- Schema.org -->\n{}",
            self.meta_tags,
            self.open_graph,
            self.twitter_cards,
            self.schema_org
        )
    }

    /// Check if any content was generated
    pub fn is_empty(&self) -> bool {
        self.meta_tags.is_empty()
            && self.open_graph.is_empty()
            && self.twitter_cards.is_empty()
            && self.schema_org.is_empty()
    }
}
