# Site Ranker (Rust)
Site SEO Rank Spiking (ML-Driven) Engine providing Rolex-grade at-scale site-rank spiking for site developers in Rust. 

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


