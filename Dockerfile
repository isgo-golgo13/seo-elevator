# ============================================================================
# site-ranker-rs Dockerfile
# ============================================================================
# Multi-stage, rootless build for production security
#
# Build targets:
#   - builder     : Full Rust build environment with all tools
#   - runtime     : Minimal production image (default)
#   - dev         : Development image with cargo, rustfmt, clippy
#
# Usage:
#   docker build -t site-ranker .                    # Production image
#   docker build --target builder -t site-ranker:builder .  # Builder image
#   docker build --target dev -t site-ranker:dev .   # Dev image
# ============================================================================

# ============================================
# Stage 1: Chef (dependency caching)
# ============================================
FROM rust:1.75-bookworm AS chef

RUN cargo install cargo-chef
WORKDIR /build

# ============================================
# Stage 2: Planner (analyze dependencies)
# ============================================
FROM chef AS planner

COPY Cargo.toml Cargo.lock* ./
COPY crates/ crates/

RUN cargo chef prepare --recipe-path recipe.json

# ============================================
# Stage 3: Builder (compile dependencies + app)
# ============================================
FROM chef AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy and build dependencies (cached!)
COPY --from=planner /build/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Copy source and build application
COPY Cargo.toml Cargo.lock* ./
COPY crates/ crates/

RUN cargo build --release --package site-ranker-cli && \
    strip target/release/site-ranker

# Run tests in builder
RUN cargo test --release --workspace

# ============================================
# Stage 4: Development Image
# ============================================
FROM rust:1.75-bookworm AS dev

# Install dev tools
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    git \
    curl \
    vim \
    && rm -rf /var/lib/apt/lists/*

# Install Rust tools
RUN rustup component add clippy rustfmt && \
    cargo install cargo-watch cargo-audit cargo-outdated

# Create non-root user
RUN groupadd --gid 1000 developer && \
    useradd --uid 1000 --gid developer --shell /bin/bash --create-home developer

# Set up workspace
WORKDIR /workspace
RUN chown developer:developer /workspace

# Switch to non-root user
USER developer

# Default command: shell
CMD ["bash"]

# Labels
LABEL stage="development"
LABEL description="Development environment for site-ranker-rs"

# ============================================
# Stage 5: Runtime (Production - Rootless)
# ============================================
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies only
RUN apt-get update && apt-get install -y \
    ca-certificates \
    tini \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean \
    && rm -rf /var/cache/apt/archives/*

# Create non-root user
RUN groupadd --gid 1000 siteranker && \
    useradd --uid 1000 --gid siteranker --shell /bin/bash --create-home siteranker

# Copy binary from builder
COPY --from=builder /build/target/release/site-ranker /usr/local/bin/site-ranker

# Set ownership and permissions
RUN chown siteranker:siteranker /usr/local/bin/site-ranker && \
    chmod 755 /usr/local/bin/site-ranker

# Switch to non-root user
USER siteranker
WORKDIR /home/siteranker

# Create workspace directory with proper permissions
RUN mkdir -p /home/siteranker/workspace /home/siteranker/output

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD site-ranker --version || exit 1

# Use tini as init system for proper signal handling
ENTRYPOINT ["/usr/bin/tini", "--", "site-ranker"]
CMD ["--help"]

# ============================================
# Labels (OCI standard)
# ============================================
LABEL org.opencontainers.image.title="site-ranker-rs"
LABEL org.opencontainers.image.description="AI-powered SEO rank accelerator"
LABEL org.opencontainers.image.vendor="EngineVector"
LABEL org.opencontainers.image.url="https://enginevector.io"
LABEL org.opencontainers.image.source="https://github.com/enginevector/site-ranker-rs"
LABEL org.opencontainers.image.licenses="MIT"
LABEL org.opencontainers.image.version="0.1.0"
LABEL maintainer="EngineVector <info@enginevector.io>"
