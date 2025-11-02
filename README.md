# seo-lovator
Site SEO Engine in Rust

# Project Structure
```shell
seo-lovator/
├── Cargo.toml              # Workspace configuration
├── Dockerfile              # Rootless multi-stage container
├── README.md               # Main documentation
├── compiling.md             # Compile and execution guide
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

