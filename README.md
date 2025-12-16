# Site Ranker (Rust)
Site SEO Rank Spiking (ML-Driven) Engine providing Rolex-grade at-scale site-rank spiking for site developers in Rust. 


**AI-powered SEO rank accelerator** - Analyze, optimize, and inject enterprise-grade SEO metadata into any website.

[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)


## Why site-ranker-rs?

| Traditional SEO Tools | site-ranker-rs |
|-----------------------|----------------|
| ❌ Rule-based audits only | ✅ ML-powered optimization |
| ❌ Manual implementation | ✅ Auto-inject metadata |
| ❌ Generic recommendations | ✅ Business-type aware |
| ❌ No Schema.org generation | ✅ Full JSON-LD with trends |


**Don't just audit SEO — let AI write SEO that converts.**

## Features

### Intelligent Analysis
- **Framework Detection** - Next.js, React, Vue, Vite, vanilla HTML
- **Business Type Recognition** - SaaS, E-commerce, Services, Blog ...
- **Keyword Extraction** - TF-IDF inspired scoring with phrase detection
- **SEO Audit** - Title, description, OG tags, Twitter Cards, Schema.org

### ML-Powered Optimization
- **Sentiment Analysis** - Optimize for positive sentiment triggers
- **Power Word Detection** - Find words that boost CTR
- **Title Suggestions** - A/B variants with scoring
- **Description Generation** - CTA-focused, engagement-optimized

### Schema.org Trend Prediction
- **Rich Snippet Trends** - Know which schemas are gaining SERP features
- **Business-Specific Schemas** - FAQPage, Product, LocalBusiness, etc.
- **Proactive Recommendations** - "Add FAQPage schema NOW - trending!"

### Automatic Injection
- **Meta Tags** - Title, description, keywords, canonical
- **Open Graph** - LinkedIn optimization
- **Twitter (X) Cards** - Summary and large image cards
- **JSON-LD Schema** - Full structured data graph



# Project Structure
```shell
site-ranker-rs/
├── Cargo.toml              # Workspace configuration
├── Dockerfile              # Rootless multi-stage container
├── README.md               # Main documentation
├── QUICKSTART.md           # Quick start and deploy documentation
├── compiling.md            # Compile and execution guide
├── LICENSE                 # MIT License
├── .dockerignore           # Docker build exclusions
│
├── crates/                 # Modular crate architecture
│   ├── analyzer/           # Website analysis
│   ├── injector/           # SEO injection
│   ├── ml-engine/          # ML optimization
│   └── cli/                # Command-line interface
│
└── test-site/               # Test websites
    └── test-site/
        └── index.html
```

## Key Dependencies Per-Crate


| Crate              | Dependencies                              |
|--------------------|-------------------------------------------|
| `analyzer`         | scraper, regex, serde                     | 
| `injector`         | scraper, serde_json, regex                | 
| `ml-engine`        | tch (feature-gated), tokio                |      
| `cli`              | clap, tokio, colored, all internal crates | 


