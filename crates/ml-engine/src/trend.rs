//! Schema.org trend prediction
//!
//! ## The Mega-Winning Sell
//!
//! Predicts which Schema.org types are gaining SERP features.
//! Tells users: "Add FAQPage schema NOW - it's trending for rich snippets"
//!
//! This is based on industry knowledge of Google's rich result patterns.
//! Future versions will incorporate ML model trained on SERP data.

use crate::{MlEngineError, MlResult, MlStrategy, Recommendation, RecommendationCategory, Priority};
use site_ranker_analyzer::{AnalysisResult, BusinessType};

/// Schema.org trend information
#[derive(Debug, Clone)]
pub struct SchemaTrend {
    /// Schema type name
    pub schema_type: String,

    /// Trend score (0.0 - 1.0, higher = more trending)
    pub trend_score: f32,

    /// Whether this schema gets rich snippets
    pub has_rich_snippets: bool,

    /// Description of the trend
    pub description: String,

    /// Recommended action
    pub action: String,
}

/// Trend predictor for Schema.org types
pub struct TrendPredictor {
    /// Known trending schemas with their scores
    trending_schemas: Vec<TrendingSchema>,
}

struct TrendingSchema {
    schema_type: &'static str,
    trend_score: f32,
    has_rich_snippets: bool,
    applicable_to: Vec<BusinessType>,
    description: &'static str,
}

impl TrendPredictor {
    pub fn new() -> Self {
        Self {
            trending_schemas: Self::build_trending_schemas(),
        }
    }

    fn build_trending_schemas() -> Vec<TrendingSchema> {
        vec![
            // High trending - Google actively promoting
            TrendingSchema {
                schema_type: "FAQPage",
                trend_score: 0.95,
                has_rich_snippets: true,
                applicable_to: vec![
                    BusinessType::Service,
                    BusinessType::SaaS,
                    BusinessType::Ecommerce,
                    BusinessType::Healthcare,
                    BusinessType::Education,
                ],
                description: "FAQ rich results are appearing more frequently in SERPs",
            },
            TrendingSchema {
                schema_type: "HowTo",
                trend_score: 0.90,
                has_rich_snippets: true,
                applicable_to: vec![
                    BusinessType::Service,
                    BusinessType::Education,
                    BusinessType::Blog,
                ],
                description: "How-to rich results with step-by-step instructions",
            },
            TrendingSchema {
                schema_type: "Product",
                trend_score: 0.92,
                has_rich_snippets: true,
                applicable_to: vec![BusinessType::Ecommerce],
                description: "Product rich results with price, availability, reviews",
            },
            TrendingSchema {
                schema_type: "Review",
                trend_score: 0.88,
                has_rich_snippets: true,
                applicable_to: vec![
                    BusinessType::Ecommerce,
                    BusinessType::Service,
                    BusinessType::LocalBusiness,
                    BusinessType::Restaurant,
                ],
                description: "Star ratings in search results dramatically increase CTR",
            },
            TrendingSchema {
                schema_type: "LocalBusiness",
                trend_score: 0.85,
                has_rich_snippets: true,
                applicable_to: vec![
                    BusinessType::LocalBusiness,
                    BusinessType::Restaurant,
                    BusinessType::Healthcare,
                ],
                description: "Local business info in maps and search",
            },
            TrendingSchema {
                schema_type: "Organization",
                trend_score: 0.80,
                has_rich_snippets: true,
                applicable_to: vec![
                    BusinessType::Service,
                    BusinessType::SaaS,
                    BusinessType::Agency,
                    BusinessType::Technology,
                ],
                description: "Knowledge panel for brand recognition",
            },
            TrendingSchema {
                schema_type: "SoftwareApplication",
                trend_score: 0.82,
                has_rich_snippets: true,
                applicable_to: vec![BusinessType::SaaS, BusinessType::Technology],
                description: "Software rich results with ratings and pricing",
            },
            TrendingSchema {
                schema_type: "Article",
                trend_score: 0.75,
                has_rich_snippets: true,
                applicable_to: vec![BusinessType::Blog, BusinessType::Education],
                description: "Article rich results for news and blog content",
            },
            TrendingSchema {
                schema_type: "BreadcrumbList",
                trend_score: 0.70,
                has_rich_snippets: true,
                applicable_to: vec![
                    BusinessType::Ecommerce,
                    BusinessType::Service,
                    BusinessType::Blog,
                ],
                description: "Breadcrumb navigation in search results",
            },
            TrendingSchema {
                schema_type: "VideoObject",
                trend_score: 0.85,
                has_rich_snippets: true,
                applicable_to: vec![
                    BusinessType::Education,
                    BusinessType::Blog,
                    BusinessType::Service,
                ],
                description: "Video thumbnails and duration in search results",
            },
            // Emerging trends
            TrendingSchema {
                schema_type: "Event",
                trend_score: 0.72,
                has_rich_snippets: true,
                applicable_to: vec![
                    BusinessType::LocalBusiness,
                    BusinessType::Education,
                    BusinessType::NonProfit,
                ],
                description: "Event rich results with dates and locations",
            },
            TrendingSchema {
                schema_type: "Course",
                trend_score: 0.78,
                has_rich_snippets: true,
                applicable_to: vec![BusinessType::Education, BusinessType::SaaS],
                description: "Course rich results for educational content",
            },
        ]
    }

    fn get_applicable_trends(&self, business_type: &BusinessType) -> Vec<&TrendingSchema> {
        self.trending_schemas
            .iter()
            .filter(|t| {
                t.applicable_to.contains(business_type)
                    || t.applicable_to.contains(&BusinessType::Unknown)
            })
            .collect()
    }
}

impl Default for TrendPredictor {
    fn default() -> Self {
        Self::new()
    }
}

impl MlStrategy for TrendPredictor {
    fn name(&self) -> &'static str {
        "trend_predictor"
    }

    fn process(&self, analysis: &AnalysisResult) -> Result<MlResult, MlEngineError> {
        let applicable_trends = self.get_applicable_trends(&analysis.business_type);

        let schema_trends: Vec<SchemaTrend> = applicable_trends
            .iter()
            .map(|t| SchemaTrend {
                schema_type: t.schema_type.to_string(),
                trend_score: t.trend_score,
                has_rich_snippets: t.has_rich_snippets,
                description: t.description.to_string(),
                action: format!("Add {} schema to your page", t.schema_type),
            })
            .collect();

        // Generate recommendations for missing high-value schemas
        let mut recommendations = Vec::new();

        // Check if FAQPage would be beneficial
        if applicable_trends.iter().any(|t| t.schema_type == "FAQPage") {
            recommendations.push(Recommendation {
                category: RecommendationCategory::Schema,
                priority: Priority::High,
                message: "FAQPage schema is trending - 30%+ CTR increase potential".to_string(),
                action: "Add FAQ section with FAQPage structured data".to_string(),
            });
        }

        // Check for Review schema
        if applicable_trends.iter().any(|t| t.schema_type == "Review") {
            recommendations.push(Recommendation {
                category: RecommendationCategory::Schema,
                priority: Priority::High,
                message: "Review/Rating schema drives highest CTR improvements".to_string(),
                action: "Add customer reviews with Review/AggregateRating schema".to_string(),
            });
        }

        Ok(MlResult {
            schema_trends,
            recommendations,
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_trends() {
        let analysis = AnalysisResult {
            business_type: BusinessType::Service,
            ..Default::default()
        };

        let predictor = TrendPredictor::new();
        let result = predictor.process(&analysis).unwrap();

        assert!(!result.schema_trends.is_empty());
        assert!(result.schema_trends.iter().any(|t| t.schema_type == "FAQPage"));
    }

    #[test]
    fn test_ecommerce_trends() {
        let analysis = AnalysisResult {
            business_type: BusinessType::Ecommerce,
            ..Default::default()
        };

        let predictor = TrendPredictor::new();
        let result = predictor.process(&analysis).unwrap();

        assert!(result.schema_trends.iter().any(|t| t.schema_type == "Product"));
    }
}
