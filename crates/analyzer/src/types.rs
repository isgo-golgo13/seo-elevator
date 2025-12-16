//! Core types for analysis results

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Result of analyzing HTML content
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnalysisResult {
    /// Extracted keywords with frequency and relevance scores
    pub keywords: Vec<Keyword>,

    /// Detected business/service type
    pub business_type: BusinessType,

    /// Detected language
    pub language: Option<String>,

    /// Existing SEO elements found
    pub existing_seo: ExistingSeo,

    /// Content summary for meta generation
    pub content_summary: Option<String>,

    /// Sentiment score (-1.0 to 1.0)
    pub sentiment_score: Option<f32>,

    /// Raw text content (for ML processing)
    pub raw_text: Option<String>,
}

impl AnalysisResult {
    /// Merge another result into this one
    pub fn merge(&mut self, other: AnalysisResult) {
        // Merge keywords (dedupe by word)
        for kw in other.keywords {
            if !self.keywords.iter().any(|k| k.word == kw.word) {
                self.keywords.push(kw);
            }
        }

        // Take non-default values
        if other.business_type != BusinessType::Unknown {
            self.business_type = other.business_type;
        }
        if other.language.is_some() {
            self.language = other.language;
        }
        if other.content_summary.is_some() {
            self.content_summary = other.content_summary;
        }
        if other.sentiment_score.is_some() {
            self.sentiment_score = other.sentiment_score;
        }
        if other.raw_text.is_some() {
            self.raw_text = other.raw_text;
        }

        // Merge existing SEO (OR operation)
        self.existing_seo.merge(other.existing_seo);
    }

    /// Get top N keywords by score
    pub fn top_keywords(&self, n: usize) -> Vec<&Keyword> {
        let mut sorted: Vec<_> = self.keywords.iter().collect();
        sorted.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        sorted.into_iter().take(n).collect()
    }
}

/// Extracted keyword with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyword {
    pub word: String,
    pub frequency: u32,
    pub score: f32,
    pub is_phrase: bool,
}

/// Detected business/service type
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BusinessType {
    #[default]
    Unknown,
    Service,
    Ecommerce,
    Blog,
    Portfolio,
    SaaS,
    LocalBusiness,
    Restaurant,
    Agency,
    NonProfit,
    Education,
    Healthcare,
    RealEstate,
    Technology,
}

impl BusinessType {
    /// Get Schema.org type for this business
    pub fn schema_type(&self) -> &'static str {
        match self {
            Self::Unknown => "Organization",
            Self::Service => "ProfessionalService",
            Self::Ecommerce => "Store",
            Self::Blog => "Blog",
            Self::Portfolio => "Person",
            Self::SaaS => "SoftwareApplication",
            Self::LocalBusiness => "LocalBusiness",
            Self::Restaurant => "Restaurant",
            Self::Agency => "Organization",
            Self::NonProfit => "NGO",
            Self::Education => "EducationalOrganization",
            Self::Healthcare => "MedicalOrganization",
            Self::RealEstate => "RealEstateAgent",
            Self::Technology => "TechArticle",
        }
    }
}

/// Existing SEO elements found in HTML
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExistingSeo {
    pub has_title: bool,
    pub title: Option<String>,
    pub has_description: bool,
    pub description: Option<String>,
    pub has_og_tags: bool,
    pub has_twitter_cards: bool,
    pub has_schema: bool,
    pub has_canonical: bool,
    pub has_viewport: bool,
    pub has_charset: bool,
    pub h1_count: u32,
    pub img_without_alt: u32,
}

impl ExistingSeo {
    pub fn merge(&mut self, other: ExistingSeo) {
        self.has_title = self.has_title || other.has_title;
        self.has_description = self.has_description || other.has_description;
        self.has_og_tags = self.has_og_tags || other.has_og_tags;
        self.has_twitter_cards = self.has_twitter_cards || other.has_twitter_cards;
        self.has_schema = self.has_schema || other.has_schema;
        self.has_canonical = self.has_canonical || other.has_canonical;
        self.has_viewport = self.has_viewport || other.has_viewport;
        self.has_charset = self.has_charset || other.has_charset;
        self.h1_count += other.h1_count;
        self.img_without_alt += other.img_without_alt;

        if other.title.is_some() {
            self.title = other.title;
        }
        if other.description.is_some() {
            self.description = other.description;
        }
    }

    /// Calculate SEO completeness score (0-100)
    pub fn completeness_score(&self) -> u32 {
        let mut score = 0u32;
        if self.has_title { score += 15; }
        if self.has_description { score += 15; }
        if self.has_og_tags { score += 20; }
        if self.has_twitter_cards { score += 15; }
        if self.has_schema { score += 20; }
        if self.has_canonical { score += 5; }
        if self.has_viewport { score += 5; }
        if self.has_charset { score += 5; }
        score
    }
}

/// Detected web framework
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Framework {
    #[default]
    VanillaHtml,
    React,
    NextJs,
    Vue,
    Nuxt,
    Svelte,
    Vite,
    Angular,
    Unknown,
}

impl Framework {
    /// Get the injection target file pattern
    pub fn injection_target(&self) -> &'static str {
        match self {
            Self::VanillaHtml => "index.html",
            Self::React => "public/index.html",
            Self::NextJs => "app/layout.tsx or pages/_document.tsx",
            Self::Vue => "index.html",
            Self::Nuxt => "nuxt.config.ts",
            Self::Svelte => "src/app.html",
            Self::Vite => "index.html",
            Self::Angular => "src/index.html",
            Self::Unknown => "index.html",
        }
    }
}

/// Analysis of a single file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAnalysis {
    pub path: PathBuf,
    pub result: AnalysisResult,
}

/// Analysis of entire directory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryAnalysis {
    pub root: PathBuf,
    pub main_file: Option<PathBuf>,
    pub files: Vec<FileAnalysis>,
    pub framework: Framework,
}

impl DirectoryAnalysis {
    /// Get merged analysis from all files
    pub fn merged_result(&self) -> AnalysisResult {
        let mut merged = AnalysisResult::default();
        for file in &self.files {
            merged.merge(file.result.clone());
        }
        merged
    }
}
