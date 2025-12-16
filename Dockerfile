# site-ranker-rs Dockerfile
# Multi-stage, rootless build for production security

# ============================================
# Stage 1: Build
# ============================================
FROM rust:1.75-bookworm AS builder

WORKDIR /build

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests first for dependency caching
COPY Cargo.toml Cargo.lock* ./
COPY crates/analyzer/Cargo.toml crates/analyzer/
COPY crates/injector/Cargo.toml crates/injector/
COPY crates/ml-engine/Cargo.toml crates/ml-engine/
COPY crates/cli/Cargo.toml crates/cli/

# Create dummy source files for dependency compilation
RUN mkdir -p crates/analyzer/src crates/injector/src crates/ml-engine/src crates/cli/src && \
    echo "pub fn dummy() {}" > crates/analyzer/src/lib.rs && \
    echo "pub fn dummy() {}" > crates/injector/src/lib.rs && \
    echo "pub fn dummy() {}" > crates/ml-engine/src/lib.rs && \
    echo "fn main() {}" > crates/cli/src/main.rs

# Build dependencies (cached layer)
RUN cargo build --release --package site-ranker-cli 2>/dev/null || true

# Copy actual source code
COPY crates/ crates/

# Touch source files to trigger rebuild
RUN touch crates/*/src/*.rs

# Build release binary
RUN cargo build --release --package site-ranker-cli

# ============================================
# Stage 2: Runtime (Rootless)
# ============================================
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd --gid 1000 siteranker && \
    useradd --uid 1000 --gid siteranker --shell /bin/bash --create-home siteranker

# Copy binary from builder
COPY --from=builder /build/target/release/site-ranker /usr/local/bin/site-ranker

# Set ownership
RUN chown siteranker:siteranker /usr/local/bin/site-ranker

# Switch to non-root user
USER siteranker
WORKDIR /home/siteranker

# Create workspace directory
RUN mkdir -p /home/siteranker/workspace

# Set entrypoint
ENTRYPOINT ["site-ranker"]
CMD ["--help"]

# ============================================
# Labels
# ============================================
LABEL org.opencontainers.image.title="site-ranker-rs"
LABEL org.opencontainers.image.description="AI-powered SEO rank accelerator"
LABEL org.opencontainers.image.vendor="EngineVector"
LABEL org.opencontainers.image.source="https://github.com/enginevector/site-ranker-rs"
LABEL org.opencontainers.image.licenses="MIT"
