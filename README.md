# Site Ranker (Rust)
Site SEO Rank Spiking (ML-Driven) Engine providing Rolex-grade at-scale site-rank spiking for site developers in Rust. 


**AI-powered SEO rank accelerator** - Analyze, optimize, and inject enterprise-grade SEO metadata into any website.

[![Rust](https://img.shields.io/badge/rust-1.75+-red.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)


## Intention of site-ranker-rs?

| Traditional SEO Tools | site-ranker-rs |
|-----------------------|----------------|
| [X] Rule-based audits only | [+] ML-powered optimization |
| [X] Manual implementation | [+] Auto-inject metadata |
| [X] Generic recommendations | [+] Business-type aware |
| [X] No Schema.org generation | [+] Full JSON-LD with trends |


**Don't just audit SEO — let AI write SEO that converts.**

## Features

### Intelligent Analysis
- **Framework Detection** - Next.js, React, Vue, Vite, vanilla HTML
- **Site Type Recognition** - SaaS, E-commerce, Services,  ...
- **Keyword Extraction** - TF-IDF inspired scoring with phrase detection
- **SEO Audit** - Title, description, OG tags, Twitter Cards, Schema.org

### ML-Powered Optimization
- **Sentiment Analysis** - Optimize for positive sentiment triggers
- **Power Word Detection** - Find words that boost CTR
- **Title Suggestions** - A/B variants with scoring
- **Description Generation** - CTA-focused, engagement-optimized

### Schema.org Trend Prediction
- **Rich Trends** - Know which schemas are gaining SERP features
- **Site-Specific Schemas** - FAQPage, Product, ...
- **Proactive Recommendations** - "Add FAQPage schema NOW - trending"

### Automatic Injection
- **Meta Tags** - Title, description, keywords, canonical
- **Open Graph** - LinkedIn optimization
- **Twitter (X) Cards** - Summary and large image cards
- **JSON-LD Schema** - Full structured data graph

---


# Project Structure
```shell
site-ranker-rs/
├── Cargo.toml                          # Workspace config
├── Dockerfile                          # Rootless multi-stage
├── README.md                           # Full documentation
├── QUICKSTART.md                       # 2-minute guide
├── crates/
│   ├── analyzer/                       # 5 files - HTML parsing, keyword extraction
│   │   └── src/{lib,error,types,strategies/*}.rs
│   ├── injector/                       # 7 files - Meta, OG, Twitter, Schema.org
│   │   └── src/{lib,error,types,strategies/*}.rs
│   ├── ml-engine/                      # 5 files - Sentiment, optimizer, trends
│   │   └── src/{lib,error,sentiment,optimizer,trend}.rs
│   └── cli/                            # Full CLI with analyze/inject/run/report
│       └── src/main.rs
└── site-templates/template-site/       # Test HTML site
    └── index.html
```

## Key Dependencies Per-Crate


| Crate              | Dependencies                              |
|--------------------|-------------------------------------------|
| `analyzer`         | scraper, regex, serde                     | 
| `injector`         | scraper, serde_json, regex                | 
| `ml-engine`        | tch (feature-gated), tokio                |      
| `cli`              | clap, tokio, colored, all internal crates | 


## Installation

### From Source

```bash
git clone https://github.com/enginevector/site-ranker-rs.git
cd site-ranker-rs
cargo build --release
```

### Docker

```bash
docker build -t site-ranker .
docker run -v $(pwd)/mysite:/home/siteranker/workspace site-ranker analyze /home/siteranker/workspace
```

### Using the Makefile

```shell
make build              # Build release binary
make test               # Run all tests
make lint               # fmt-check + clippy
make run ARGS='analyze ./site'
make install-local      # Install to ~/.local/bin
make dist               # Create distribution tarball

# Docker commands 
make docker-build       # Build via Docker
make docker-test        # Run tests via Docker
make docker-lint        # Lint Docker
make docker-run ARGS='analyze /workspace/site'
make docker-shell       # Open dev shell
make docker-image       # Build production image

# Docker Compose 
docker-compose up dev           # Development environment
docker-compose run build        # Build binary
docker-compose run test         # Run tests
docker-compose run analyze      # Analyze template
docker-compose run optimize     # Full optimization pipeline
docker-compose run watch        # Watch mode (cargo-watch)
```


## Quick Start

### 1. Analyze Website

```bash
site-ranker analyze ./website-x
```

Output:
```
Analyzing website...
──────────────────────────────────────────────────

ANALYSIS RESULTS
══════════════════════════════════════════════════

Framework: React
Main file: ./website-x/public/index.html
Business Type: SaaS

SEO Audit:
   [+] Title tag
   [x] Meta description
   [x] Open Graph tags
   [x] Twitter Cards
   [x] Schema.org markup

SEO Score: 15/100
  Optimization Score: 42/100

Recommendations:
   🔴 Missing meta description
      → Add a compelling meta description (150-160 characters)
   🟠 Missing Schema.org structured data
      → Add JSON-LD schema for rich snippets in search results
```

### 2. Inject SEO Metadata

```bash
# Test it (Safe dry-run analysis ONLY)
.site-ranker analyze ./site-templates/template-site


site-ranker inject ./website-x \
  --site-name "Website-X" \
  --site-url "https://website-x.com" \
  --twitter "webite-x" \
  --output ./optimized
```

### 3. Full Pipeline (Recommended)

```bash
site-ranker run ./website-x \
  --site-name "Website-X" \
  --site-url "https://website-x.com" \
  --twitter "website-x" \
  --email "info@website-x.com" \
  --output ./optimized
```

### 4. Generate Report

```bash
site-ranker report ./website-x --output seo-report.md
```


### Strategy Pattern

All major components use the Strategy pattern for extensibility:

```rust
// Add custom analyzer
pub trait AnalyzerStrategy: Send + Sync {
    fn name(&self) -> &'static str;
    fn analyze(&self, content: &str) -> Result<AnalysisResult, AnalyzerError>;
}

// Add custom injector
pub trait InjectorStrategy: Send + Sync {
    fn name(&self) -> &'static str;
    fn generate(&self, analysis: &AnalysisResult, config: &SeoConfig) -> Result<String, InjectorError>;
    fn inject_content(&self, html: &str, content: &str) -> Result<String, InjectorError>;
}

// Add custom ML strategy
pub trait MlStrategy: Send + Sync {
    fn name(&self) -> &'static str;
    fn process(&self, analysis: &AnalysisResult) -> Result<MlResult, MlEngineError>;
}
```



## Configuration

### SeoConfig Builder

```rust
use site_ranker_injector::{SeoConfig, Address};

let config = SeoConfig::builder()
    .site_name("EngineVector")
    .site_url("https://enginevector.io")
    .twitter_handle("enginevector")
    .default_image("https://enginevector.io/og-image.png")
    .contact_email("info@menginevector.io")
    .phone("+XX-XXXXXXXXX")
    .address(Address {
        street: "XXXXXXXXXX St.",
        city: "Los Angeles",
        state: "CA",
        postal_code: "90048",
        country: "US",
    })
    .build();
```

### CLI Options

```
USAGE:
    site-ranker [OPTIONS] <COMMAND>

COMMANDS:
    analyze  Analyze website for SEO opportunities
    inject   Inject optimized SEO metadata
    run      Full pipeline (analyze + optimize + inject)
    report   Generate detailed SEO report

OPTIONS:
    -v, --verbose        Enable verbose logging
    -f, --format <FMT>   Output format: text, json [default: text]
    -h, --help           Print help
    -V, --version        Print version
```

## SEO Injection Priority

Based on ROI analysis:

| Priority | Feature | Impact |
|----------|---------|--------|
| 🥇 | Schema.org JSON-LD | 30%+ CTR with rich snippets |
| 🥈 | Open Graph + Twitter | 2-3x social engagement |
| 🥉 | Meta Tags | Foundation for all SEO |
| 4 | Canonical URLs | Prevents duplicate content |

## Roadmap

- [x] Core analyzer engine
- [x] SEO injector pipeline  
- [x] Rule-based ML engine
- [x] CLI interface
- [ ] PyTorch deep learning models (`--features torch`)
- [ ] Web UI dashboard
- [ ] API server mode
- [ ] Chrome extension

## License

MIT License - see [LICENSE](LICENSE) for details.

## Author

**EngineVector** - [enginevector.io](https://enginevector.io)