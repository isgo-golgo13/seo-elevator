//! # site-ranker CLI
//!
//! AI-powered SEO rank accelerator.
//!
//! ## Commands
//!
//! - `analyze` - Analyze website for SEO opportunities
//! - `inject` - Inject optimized SEO metadata
//! - `run` - Full pipeline (analyze + optimize + inject)
//! - `report` - Generate detailed SEO report

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use site_ranker_analyzer::{AnalyzerPipeline, DirectoryAnalysis, Framework};
use site_ranker_injector::{InjectorPipeline, SeoConfig};
use site_ranker_ml_engine::{MlEngine, MlResult, Priority};
use std::path::PathBuf;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Parser)]
#[command(
    name = "site-ranker",
    author = "EngineVector <info@enginevector.io>",
    version,
    about = "🚀 AI-powered SEO rank accelerator - analyze, optimize, and inject enterprise-grade SEO metadata",
    long_about = None
)]
struct Cli {
    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Output format (text, json)
    #[arg(short, long, global = true, default_value = "text")]
    format: OutputFormat,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum OutputFormat {
    Text,
    Json,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(Self::Text),
            "json" => Ok(Self::Json),
            _ => Err(format!("Unknown format: {}", s)),
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze website for SEO opportunities
    Analyze {
        /// Directory containing the website
        #[arg(value_name = "DIRECTORY")]
        directory: PathBuf,

        /// Output analysis to file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Inject optimized SEO metadata into website
    Inject {
        /// Directory containing the website
        #[arg(value_name = "DIRECTORY")]
        directory: PathBuf,

        /// Output directory for modified files
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Site name for SEO tags
        #[arg(long, default_value = "My Site")]
        site_name: String,

        /// Site URL
        #[arg(long, default_value = "https://example.com")]
        site_url: String,

        /// Twitter handle (without @)
        #[arg(long)]
        twitter: Option<String>,

        /// Default social sharing image URL
        #[arg(long)]
        image: Option<String>,

        /// Contact email
        #[arg(long)]
        email: Option<String>,

        /// Dry run (don't write files)
        #[arg(long)]
        dry_run: bool,
    },

    /// Run full SEO optimization pipeline
    Run {
        /// Directory containing the website
        #[arg(value_name = "DIRECTORY")]
        directory: PathBuf,

        /// Output directory for optimized files
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Site name
        #[arg(long, default_value = "My Site")]
        site_name: String,

        /// Site URL
        #[arg(long, default_value = "https://example.com")]
        site_url: String,

        /// Twitter handle
        #[arg(long)]
        twitter: Option<String>,

        /// Social sharing image URL
        #[arg(long)]
        image: Option<String>,

        /// Contact email
        #[arg(long)]
        email: Option<String>,
    },

    /// Generate detailed SEO report
    Report {
        /// Directory containing the website
        #[arg(value_name = "DIRECTORY")]
        directory: PathBuf,

        /// Output report file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Setup logging
    let filter = if cli.verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::new("info")
    };

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();

    match cli.command {
        Commands::Analyze { directory, output } => {
            run_analyze(&directory, output.as_deref(), cli.format).await
        }
        Commands::Inject {
            directory,
            output,
            site_name,
            site_url,
            twitter,
            image,
            email,
            dry_run,
        } => {
            let config = build_config(&site_name, &site_url, twitter, image, email);
            run_inject(&directory, output.as_deref(), &config, dry_run, cli.format).await
        }
        Commands::Run {
            directory,
            output,
            site_name,
            site_url,
            twitter,
            image,
            email,
        } => {
            let config = build_config(&site_name, &site_url, twitter, image, email);
            run_full_pipeline(&directory, output.as_deref(), &config, cli.format).await
        }
        Commands::Report { directory, output } => {
            run_report(&directory, output.as_deref(), cli.format).await
        }
    }
}

fn build_config(
    site_name: &str,
    site_url: &str,
    twitter: Option<String>,
    image: Option<String>,
    email: Option<String>,
) -> SeoConfig {
    SeoConfig::builder()
        .site_name(site_name)
        .site_url(site_url)
        .twitter_handle(twitter.unwrap_or_default())
        .default_image(image.unwrap_or_default())
        .contact_email(email.unwrap_or_default())
        .build()
}

async fn run_analyze(
    directory: &PathBuf,
    output: Option<&std::path::Path>,
    format: OutputFormat,
) -> Result<()> {
    println!("\n{}", "🔍 Analyzing website...".cyan().bold());
    println!("{}", "─".repeat(50));

    let analyzer = AnalyzerPipeline::default_pipeline();
    let analysis = analyzer
        .analyze_directory(directory)
        .context("Failed to analyze directory")?;

    // Run ML analysis
    let ml_engine = MlEngine::default_engine();
    let merged = analysis.merged_result();
    let ml_result = ml_engine.process(&merged).context("ML analysis failed")?;

    if format == OutputFormat::Json {
        let json = serde_json::to_string_pretty(&analysis)?;
        if let Some(path) = output {
            std::fs::write(path, &json)?;
            println!("Analysis saved to: {}", path.display());
        } else {
            println!("{}", json);
        }
    } else {
        print_analysis_results(&analysis, &ml_result);

        if let Some(path) = output {
            let json = serde_json::to_string_pretty(&analysis)?;
            std::fs::write(path, &json)?;
            println!("\n{} {}", "📄 Analysis saved to:".green(), path.display());
        }
    }

    Ok(())
}

async fn run_inject(
    directory: &PathBuf,
    output: Option<&std::path::Path>,
    config: &SeoConfig,
    dry_run: bool,
    format: OutputFormat,
) -> Result<()> {
    println!("\n{}", "💉 Injecting SEO metadata...".cyan().bold());
    println!("{}", "─".repeat(50));

    // First analyze
    let analyzer = AnalyzerPipeline::default_pipeline();
    let analysis = analyzer
        .analyze_directory(directory)
        .context("Failed to analyze directory")?;

    let merged = analysis.merged_result();

    // Generate injections
    let injector = InjectorPipeline::default_pipeline();
    let generated = injector
        .generate_all(&merged, config)
        .context("Failed to generate SEO content")?;

    if format == OutputFormat::Text {
        println!("\n{}", "📋 Generated SEO Content:".yellow().bold());
        println!("{}", "─".repeat(50));
        println!("{}", generated.combined());
    }

    if dry_run {
        println!("\n{}", "🔍 Dry run - no files modified".yellow());
        return Ok(());
    }

    // Inject into files
    let output_dir = output.unwrap_or(directory.as_path());

    if let Some(main_file) = &analysis.main_file {
        let content = std::fs::read_to_string(main_file)?;
        let injected = injector.inject(&content, &merged, config)?;

        let output_path = if output.is_some() {
            output_dir.join(main_file.file_name().unwrap())
        } else {
            main_file.clone()
        };

        if output.is_some() {
            std::fs::create_dir_all(output_dir)?;
        }

        std::fs::write(&output_path, injected)?;
        println!(
            "\n{} {}",
            "✅ SEO injected into:".green(),
            output_path.display()
        );
    } else {
        println!("{}", "⚠️  No main HTML file found".yellow());
    }

    Ok(())
}

async fn run_full_pipeline(
    directory: &PathBuf,
    output: Option<&std::path::Path>,
    config: &SeoConfig,
    format: OutputFormat,
) -> Result<()> {
    println!("\n{}", "🚀 Running full SEO optimization pipeline...".cyan().bold());
    println!("{}", "═".repeat(50));

    // Step 1: Analyze
    println!("\n{}", "Step 1: Analyzing website...".yellow());
    let analyzer = AnalyzerPipeline::default_pipeline();
    let analysis = analyzer
        .analyze_directory(directory)
        .context("Failed to analyze directory")?;

    let merged = analysis.merged_result();

    // Step 2: ML Optimization
    println!("{}", "Step 2: Running ML optimization...".yellow());
    let ml_engine = MlEngine::default_engine();
    let ml_result = ml_engine.process(&merged).context("ML analysis failed")?;

    // Step 3: Generate & Inject
    println!("{}", "Step 3: Generating and injecting SEO...".yellow());
    let injector = InjectorPipeline::default_pipeline();

    let output_dir = output.unwrap_or(directory.as_path());

    if let Some(main_file) = &analysis.main_file {
        let content = std::fs::read_to_string(main_file)?;
        let injected = injector.inject(&content, &merged, config)?;

        let output_path = if output.is_some() {
            std::fs::create_dir_all(output_dir)?;
            output_dir.join(main_file.file_name().unwrap())
        } else {
            main_file.clone()
        };

        std::fs::write(&output_path, &injected)?;

        if format == OutputFormat::Text {
            print_analysis_results(&analysis, &ml_result);
            println!(
                "\n{} {}",
                "✅ Optimized file saved to:".green().bold(),
                output_path.display()
            );
        } else {
            let result = serde_json::json!({
                "analysis": analysis,
                "output_file": output_path.to_string_lossy(),
                "optimization_score": ml_result.optimization_score
            });
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
    } else {
        println!("{}", "⚠️  No main HTML file found".yellow());
    }

    Ok(())
}

async fn run_report(
    directory: &PathBuf,
    output: Option<&std::path::Path>,
    _format: OutputFormat,
) -> Result<()> {
    println!("\n{}", "📊 Generating SEO Report...".cyan().bold());
    println!("{}", "═".repeat(50));

    let analyzer = AnalyzerPipeline::default_pipeline();
    let analysis = analyzer
        .analyze_directory(directory)
        .context("Failed to analyze directory")?;

    let merged = analysis.merged_result();

    let ml_engine = MlEngine::default_engine();
    let ml_result = ml_engine.process(&merged).context("ML analysis failed")?;

    let report = generate_report(&analysis, &ml_result);

    if let Some(path) = output {
        std::fs::write(path, &report)?;
        println!("Report saved to: {}", path.display());
    } else {
        println!("{}", report);
    }

    Ok(())
}

fn print_analysis_results(analysis: &DirectoryAnalysis, ml_result: &MlResult) {
    let merged = analysis.merged_result();

    // Header
    println!("\n{}", "═".repeat(50));
    println!("{}", "📊 ANALYSIS RESULTS".cyan().bold());
    println!("{}", "═".repeat(50));

    // Framework detection
    println!(
        "\n{} {:?}",
        "🔧 Framework:".yellow(),
        analysis.framework
    );

    if let Some(ref main) = analysis.main_file {
        println!("{} {}", "📄 Main file:".yellow(), main.display());
    }

    println!(
        "{} {}",
        "📁 HTML files found:".yellow(),
        analysis.files.len()
    );

    // Business type
    println!(
        "\n{} {:?}",
        "🏢 Business Type:".yellow(),
        merged.business_type
    );

    // Language
    if let Some(ref lang) = merged.language {
        println!("{} {}", "🌍 Language:".yellow(), lang);
    }

    // Sentiment
    if let Some(ref sentiment) = ml_result.sentiment {
        let sentiment_color = if sentiment.score > 0.3 {
            "green"
        } else if sentiment.score < -0.3 {
            "red"
        } else {
            "yellow"
        };
        println!(
            "{} {:.2} ({:?})",
            "😊 Sentiment:".yellow(),
            sentiment.score,
            sentiment.label
        );
        if !sentiment.power_words.is_empty() {
            println!(
                "   {} {}",
                "Power words:".dimmed(),
                sentiment.power_words.join(", ")
            );
        }
    }

    // Keywords
    println!("\n{}", "🔑 Top Keywords:".yellow());
    for (i, kw) in merged.top_keywords(10).iter().enumerate() {
        println!(
            "   {}. {} (freq: {}, score: {:.2})",
            i + 1,
            kw.word,
            kw.frequency,
            kw.score
        );
    }

    // SEO Audit
    println!("\n{}", "📋 SEO Audit:".yellow());
    let seo = &merged.existing_seo;
    print_check("Title tag", seo.has_title);
    print_check("Meta description", seo.has_description);
    print_check("Open Graph tags", seo.has_og_tags);
    print_check("Twitter Cards", seo.has_twitter_cards);
    print_check("Schema.org markup", seo.has_schema);
    print_check("Canonical URL", seo.has_canonical);
    print_check("Viewport meta", seo.has_viewport);

    println!(
        "\n{} {}/100",
        "📈 SEO Score:".yellow(),
        seo.completeness_score()
    );

    println!(
        "{} {}/100",
        "🎯 Optimization Score:".yellow(),
        ml_result.optimization_score
    );

    // Schema Trends
    if !ml_result.schema_trends.is_empty() {
        println!("\n{}", "📈 Trending Schemas for Your Site:".yellow());
        for trend in ml_result.schema_trends.iter().take(5) {
            println!(
                "   {} {} (score: {:.0}%) {}",
                if trend.has_rich_snippets { "⭐" } else { "○" },
                trend.schema_type,
                trend.trend_score * 100.0,
                if trend.has_rich_snippets {
                    "- Rich snippets!".green().to_string()
                } else {
                    String::new()
                }
            );
        }
    }

    // Recommendations
    if !ml_result.recommendations.is_empty() {
        println!("\n{}", "💡 Recommendations:".yellow());
        for rec in &ml_result.recommendations {
            let priority_icon = match rec.priority {
                Priority::Critical => "🔴",
                Priority::High => "🟠",
                Priority::Medium => "🟡",
                Priority::Low => "🟢",
            };
            println!("   {} {}", priority_icon, rec.message);
            println!("      → {}", rec.action.dimmed());
        }
    }

    // Title suggestions
    if !ml_result.title_suggestions.is_empty() {
        println!("\n{}", "✏️  Title Suggestions:".yellow());
        for (i, sug) in ml_result.title_suggestions.iter().take(3).enumerate() {
            println!("   {}. \"{}\"", i + 1, sug.text.green());
            println!("      {} (score: {:.0}%)", sug.reasoning.dimmed(), sug.score * 100.0);
        }
    }

    println!("\n{}", "═".repeat(50));
}

fn print_check(label: &str, present: bool) {
    if present {
        println!("   {} {}", "✅".green(), label);
    } else {
        println!("   {} {}", "❌".red(), label);
    }
}

fn generate_report(analysis: &DirectoryAnalysis, ml_result: &MlResult) -> String {
    let merged = analysis.merged_result();
    let mut report = String::new();

    report.push_str("# SEO Analysis Report\n\n");
    report.push_str(&format!("Generated: {}\n\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));

    report.push_str("## Overview\n\n");
    report.push_str(&format!("- **Framework**: {:?}\n", analysis.framework));
    report.push_str(&format!("- **Business Type**: {:?}\n", merged.business_type));
    report.push_str(&format!("- **HTML Files**: {}\n", analysis.files.len()));
    report.push_str(&format!("- **SEO Score**: {}/100\n", merged.existing_seo.completeness_score()));
    report.push_str(&format!("- **Optimization Score**: {}/100\n\n", ml_result.optimization_score));

    report.push_str("## SEO Audit\n\n");
    report.push_str("| Element | Status |\n");
    report.push_str("|---------|--------|\n");
    report.push_str(&format!("| Title | {} |\n", if merged.existing_seo.has_title { "✅" } else { "❌" }));
    report.push_str(&format!("| Description | {} |\n", if merged.existing_seo.has_description { "✅" } else { "❌" }));
    report.push_str(&format!("| Open Graph | {} |\n", if merged.existing_seo.has_og_tags { "✅" } else { "❌" }));
    report.push_str(&format!("| Twitter Cards | {} |\n", if merged.existing_seo.has_twitter_cards { "✅" } else { "❌" }));
    report.push_str(&format!("| Schema.org | {} |\n", if merged.existing_seo.has_schema { "✅" } else { "❌" }));

    report.push_str("\n## Recommendations\n\n");
    for rec in &ml_result.recommendations {
        let priority = match rec.priority {
            Priority::Critical => "🔴 Critical",
            Priority::High => "🟠 High",
            Priority::Medium => "🟡 Medium",
            Priority::Low => "🟢 Low",
        };
        report.push_str(&format!("### {} - {}\n", priority, rec.message));
        report.push_str(&format!("**Action**: {}\n\n", rec.action));
    }

    report.push_str("## Top Keywords\n\n");
    for (i, kw) in merged.top_keywords(10).iter().enumerate() {
        report.push_str(&format!("{}. {} (score: {:.2})\n", i + 1, kw.word, kw.score));
    }

    report
}

// Add chrono for report timestamp
use chrono;
