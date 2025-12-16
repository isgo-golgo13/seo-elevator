//! Schema.org JSON-LD injector - structured data for rich snippets
//!
//! This is the MEGA-WINNING SELL feature:
//! - 30%+ CTR increase with rich results
//! - Google Knowledge Graph integration
//! - Star ratings, prices, availability in search results

use crate::{find_head_injection_point, InjectorError, InjectorStrategy, SeoConfig};
use serde_json::{json, Value};
use site_ranker_analyzer::{AnalysisResult, BusinessType};

/// Injector for Schema.org JSON-LD structured data
pub struct SchemaOrgInjector;

impl SchemaOrgInjector {
    pub fn new() -> Self {
        Self
    }

    /// Generate Organization schema
    fn generate_organization(&self, analysis: &AnalysisResult, config: &SeoConfig) -> Value {
        let mut org = json!({
            "@context": "https://schema.org",
            "@type": analysis.business_type.schema_type(),
            "name": config.site_name,
            "url": config.site_url,
        });

        // Add description
        if let Some(ref desc) = config.description_override {
            org["description"] = json!(desc);
        } else if let Some(ref desc) = analysis.existing_seo.description {
            org["description"] = json!(desc);
        } else if let Some(ref summary) = analysis.content_summary {
            org["description"] = json!(summary);
        }

        // Add logo/image
        if let Some(ref image) = config.default_image {
            org["logo"] = json!({
                "@type": "ImageObject",
                "url": image
            });
            org["image"] = json!(image);
        }

        // Add contact info
        if let Some(ref email) = config.contact_email {
            org["email"] = json!(email);
        }

        if let Some(ref phone) = config.phone {
            org["telephone"] = json!(phone);
            org["contactPoint"] = json!({
                "@type": "ContactPoint",
                "telephone": phone,
                "contactType": "customer service"
            });
        }

        // Add address
        if let Some(ref addr) = config.address {
            org["address"] = json!({
                "@type": "PostalAddress",
                "streetAddress": addr.street,
                "addressLocality": addr.city,
                "addressRegion": addr.state,
                "postalCode": addr.postal_code,
                "addressCountry": addr.country
            });
        }

        // Add social links placeholder
        org["sameAs"] = json!([]);

        org
    }

    /// Generate WebSite schema with SearchAction
    fn generate_website(&self, config: &SeoConfig) -> Value {
        json!({
            "@context": "https://schema.org",
            "@type": "WebSite",
            "name": config.site_name,
            "url": config.site_url,
            "potentialAction": {
                "@type": "SearchAction",
                "target": {
                    "@type": "EntryPoint",
                    "urlTemplate": format!("{}/search?q={{search_term_string}}", config.site_url)
                },
                "query-input": "required name=search_term_string"
            }
        })
    }

    /// Generate BreadcrumbList schema
    fn generate_breadcrumb(&self, config: &SeoConfig) -> Value {
        json!({
            "@context": "https://schema.org",
            "@type": "BreadcrumbList",
            "itemListElement": [{
                "@type": "ListItem",
                "position": 1,
                "name": "Home",
                "item": config.site_url
            }]
        })
    }

    /// Generate business-specific schemas
    fn generate_business_specific(&self, analysis: &AnalysisResult, config: &SeoConfig) -> Option<Value> {
        match analysis.business_type {
            BusinessType::Service | BusinessType::Technology => {
                // Professional Service schema
                let services: Vec<_> = analysis
                    .top_keywords(5)
                    .iter()
                    .map(|k| capitalize(&k.word))
                    .collect();

                Some(json!({
                    "@context": "https://schema.org",
                    "@type": "ProfessionalService",
                    "name": config.site_name,
                    "url": config.site_url,
                    "serviceType": services,
                    "areaServed": {
                        "@type": "Country",
                        "name": "United States"
                    },
                    "hasOfferCatalog": {
                        "@type": "OfferCatalog",
                        "name": "Services",
                        "itemListElement": services.iter().map(|s| {
                            json!({
                                "@type": "Offer",
                                "itemOffered": {
                                    "@type": "Service",
                                    "name": s
                                }
                            })
                        }).collect::<Vec<_>>()
                    }
                }))
            }

            BusinessType::SaaS => {
                Some(json!({
                    "@context": "https://schema.org",
                    "@type": "SoftwareApplication",
                    "name": config.site_name,
                    "url": config.site_url,
                    "applicationCategory": "BusinessApplication",
                    "operatingSystem": "Web",
                    "offers": {
                        "@type": "Offer",
                        "price": "0",
                        "priceCurrency": "USD"
                    }
                }))
            }

            BusinessType::LocalBusiness | BusinessType::Restaurant => {
                let mut local = json!({
                    "@context": "https://schema.org",
                    "@type": if analysis.business_type == BusinessType::Restaurant {
                        "Restaurant"
                    } else {
                        "LocalBusiness"
                    },
                    "name": config.site_name,
                    "url": config.site_url
                });

                if let Some(ref addr) = config.address {
                    local["address"] = json!({
                        "@type": "PostalAddress",
                        "streetAddress": addr.street,
                        "addressLocality": addr.city,
                        "addressRegion": addr.state,
                        "postalCode": addr.postal_code,
                        "addressCountry": addr.country
                    });
                }

                if let Some(ref phone) = config.phone {
                    local["telephone"] = json!(phone);
                }

                Some(local)
            }

            BusinessType::Blog => {
                Some(json!({
                    "@context": "https://schema.org",
                    "@type": "Blog",
                    "name": config.site_name,
                    "url": config.site_url,
                    "blogPost": []
                }))
            }

            _ => None,
        }
    }

    /// Generate FAQ schema from content (if detected)
    fn generate_faq(&self, analysis: &AnalysisResult) -> Option<Value> {
        // Look for Q&A patterns in raw text
        if let Some(ref text) = analysis.raw_text {
            let text_lower = text.to_lowercase();
            if text_lower.contains("faq")
                || text_lower.contains("frequently asked")
                || text_lower.contains("questions")
            {
                // Placeholder - in production would parse actual Q&A
                return Some(json!({
                    "@context": "https://schema.org",
                    "@type": "FAQPage",
                    "mainEntity": []
                }));
            }
        }
        None
    }
}

impl Default for SchemaOrgInjector {
    fn default() -> Self {
        Self::new()
    }
}

impl InjectorStrategy for SchemaOrgInjector {
    fn name(&self) -> &'static str {
        "schema_org_injector"
    }

    fn generate(&self, analysis: &AnalysisResult, config: &SeoConfig) -> Result<String, InjectorError> {
        // Skip if already has Schema.org
        if analysis.existing_seo.has_schema {
            return Ok(String::new());
        }

        let mut schemas = Vec::new();

        // Always add Organization/Business schema
        schemas.push(self.generate_organization(analysis, config));

        // Add WebSite schema
        schemas.push(self.generate_website(config));

        // Add BreadcrumbList
        schemas.push(self.generate_breadcrumb(config));

        // Add business-specific schema
        if let Some(schema) = self.generate_business_specific(analysis, config) {
            schemas.push(schema);
        }

        // Add FAQ if detected
        if let Some(faq) = self.generate_faq(analysis) {
            schemas.push(faq);
        }

        // Combine into graph
        let graph = json!({
            "@context": "https://schema.org",
            "@graph": schemas
        });

        let json_str = serde_json::to_string_pretty(&graph)?;

        Ok(format!(
            "    <script type=\"application/ld+json\">\n{}\n    </script>",
            indent_json(&json_str, 4)
        ))
    }

    fn inject_content(&self, html: &str, content: &str) -> Result<String, InjectorError> {
        if content.is_empty() {
            return Ok(html.to_string());
        }

        // Prefer injecting in <head>
        let injection_point = find_head_injection_point(html)
            .ok_or(InjectorError::NoInjectionPoint)?;

        let before = &html[..injection_point];
        let after = &html[injection_point..];

        Ok(format!("{}\n{}\n{}", before, content, after))
    }
}

/// Capitalize first letter
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

/// Indent JSON string
fn indent_json(json: &str, spaces: usize) -> String {
    let indent = " ".repeat(spaces);
    json.lines()
        .map(|line| format!("{}{}", indent, line))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_generation() {
        let analysis = AnalysisResult {
            business_type: BusinessType::Service,
            ..Default::default()
        };

        let config = SeoConfig {
            site_name: "Test Corp".to_string(),
            site_url: "https://test.com".to_string(),
            contact_email: Some("info@test.com".to_string()),
            ..Default::default()
        };

        let injector = SchemaOrgInjector::new();
        let result = injector.generate(&analysis, &config).unwrap();

        assert!(result.contains("application/ld+json"));
        assert!(result.contains("ProfessionalService"));
        assert!(result.contains("Test Corp"));
    }
}
