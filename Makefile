# ============================================================================
# site-ranker-rs Makefile
# ============================================================================
# Production-grade build system with Docker-wrapped cargo alternatives
#
# Usage:
#   make help          - Show all available targets
#   make build         - Build release binary (native)
#   make docker-build  - Build release binary (via Docker)
#   make test          - Run tests (native)
#   make docker-test   - Run tests (via Docker)
# ============================================================================

# Configuration
# ============================================================================
BINARY_NAME     := site-ranker
PACKAGE_NAME    := site-ranker-cli
IMAGE_NAME      := site-ranker-rs
IMAGE_TAG       := latest
DOCKER_BUILDER  := $(IMAGE_NAME)-builder
RUST_VERSION    := 1.75

# Directories
BUILD_DIR       := target
RELEASE_DIR     := $(BUILD_DIR)/release
DEBUG_DIR       := $(BUILD_DIR)/debug
DIST_DIR        := dist
INSTALL_DIR     := /usr/local/bin
TEST_SITE_DIR   := .test-site

# Docker run options for cargo wrapper
DOCKER_RUN_OPTS := --rm -v $(PWD):/workspace -w /workspace
DOCKER_CARGO    := docker run $(DOCKER_RUN_OPTS) rust:$(RUST_VERSION)-bookworm

# Colors for output
CYAN    := \033[36m
GREEN   := \033[32m
YELLOW  := \033[33m
RED     := \033[31m
RESET   := \033[0m
BOLD    := \033[1m

# ============================================================================
# Default target
# ============================================================================
.DEFAULT_GOAL := help

# ============================================================================
# Help
# ============================================================================
.PHONY: help
help: ## Show this help message
	@echo ""
	@echo "$(BOLD)$(CYAN)site-ranker-rs$(RESET) - AI-powered SEO rank accelerator"
	@echo ""
	@echo "$(BOLD)Native Commands (requires Rust installed):$(RESET)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | grep -v docker | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(CYAN)%-20s$(RESET) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(BOLD)Docker Commands (no Rust required):$(RESET)"
	@grep -E '^docker-[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(CYAN)%-20s$(RESET) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(BOLD)Examples:$(RESET)"
	@echo "  make build                              # Build release binary"
	@echo "  make analyze-url URL=https://example.com  # Analyze any URL"
	@echo "  make analyze-enginevector               # Analyze enginevector.io"
	@echo "  make optimize-url URL=https://example.com NAME='My Site'"
	@echo ""

# ============================================================================
# Native Build Targets
# ============================================================================
.PHONY: build
build: ## Build release binary
	@echo "$(CYAN)Building release binary...$(RESET)"
	cargo build --release --package $(PACKAGE_NAME)
	@echo "$(GREEN)✓ Binary built: $(RELEASE_DIR)/$(BINARY_NAME)$(RESET)"

.PHONY: build-quiet
build-quiet: ## Build release binary (suppress warnings)
	@cargo build --release --package $(PACKAGE_NAME) 2>&1 | grep -v "^warning:" || true
	@echo "$(GREEN)✓ Binary built: $(RELEASE_DIR)/$(BINARY_NAME)$(RESET)"

.PHONY: build-debug
build-debug: ## Build debug binary
	@echo "$(CYAN)Building debug binary...$(RESET)"
	cargo build --package $(PACKAGE_NAME)
	@echo "$(GREEN)✓ Binary built: $(DEBUG_DIR)/$(BINARY_NAME)$(RESET)"

.PHONY: build-all
build-all: ## Build all crates
	@echo "$(CYAN)Building all crates...$(RESET)"
	cargo build --release --workspace
	@echo "$(GREEN)✓ All crates built$(RESET)"

.PHONY: build-ml
build-ml: ## Build with PyTorch ML support
	@echo "$(CYAN)Building with ML features...$(RESET)"
	cargo build --release --package $(PACKAGE_NAME) --features ml-engine/torch
	@echo "$(GREEN)✓ ML-enabled binary built$(RESET)"

# ============================================================================
# Native Test Targets
# ============================================================================
.PHONY: test
test: ## Run all tests
	@echo "$(CYAN)Running tests...$(RESET)"
	cargo test --workspace
	@echo "$(GREEN)✓ All tests passed$(RESET)"

.PHONY: test-verbose
test-verbose: ## Run tests with output
	cargo test --workspace -- --nocapture

.PHONY: test-lib
test-lib: ## Run library tests only
	cargo test --lib --workspace

.PHONY: test-doc
test-doc: ## Run documentation tests
	cargo test --doc --workspace

# ============================================================================
# Native Quality Targets
# ============================================================================
.PHONY: check
check: ## Check code compiles without building
	@echo "$(CYAN)Checking code...$(RESET)"
	cargo check --workspace
	@echo "$(GREEN)✓ Code check passed$(RESET)"

.PHONY: clippy
clippy: ## Run Clippy linter
	@echo "$(CYAN)Running Clippy...$(RESET)"
	cargo clippy --workspace --all-targets -- -D warnings
	@echo "$(GREEN)✓ Clippy passed$(RESET)"

.PHONY: fmt
fmt: ## Format code
	@echo "$(CYAN)Formatting code...$(RESET)"
	cargo fmt --all
	@echo "$(GREEN)✓ Code formatted$(RESET)"

.PHONY: fmt-check
fmt-check: ## Check code formatting
	@echo "$(CYAN)Checking format...$(RESET)"
	cargo fmt --all -- --check
	@echo "$(GREEN)✓ Format check passed$(RESET)"

.PHONY: lint
lint: fmt-check clippy ## Run all linters

.PHONY: audit
audit: ## Security audit dependencies
	@echo "$(CYAN)Auditing dependencies...$(RESET)"
	cargo audit
	@echo "$(GREEN)✓ Audit passed$(RESET)"

# ============================================================================
# Native Run Targets
# ============================================================================
.PHONY: run
run: build ## Run the CLI (use ARGS='...' for arguments)
	@echo "$(CYAN)Running site-ranker...$(RESET)"
	$(RELEASE_DIR)/$(BINARY_NAME) $(ARGS)

.PHONY: run-debug
run-debug: build-debug ## Run debug build
	$(DEBUG_DIR)/$(BINARY_NAME) $(ARGS)

# ============================================================================
# Analyze Targets
# ============================================================================
.PHONY: analyze-template
analyze-template: build ## Analyze the template site
	$(RELEASE_DIR)/$(BINARY_NAME) analyze ./site-templates/template-site

.PHONY: analyze-url
analyze-url: build ## Analyze any URL (use URL=https://...)
ifndef URL
	$(error URL is required. Usage: make analyze-url URL=https://example.com)
endif
	@echo "$(CYAN)Fetching $(URL)...$(RESET)"
	@mkdir -p $(TEST_SITE_DIR)
	@curl -sL -o $(TEST_SITE_DIR)/index.html "$(URL)"
	@echo "$(GREEN)✓ Fetched to $(TEST_SITE_DIR)/index.html$(RESET)"
	@echo ""
	$(RELEASE_DIR)/$(BINARY_NAME) analyze $(TEST_SITE_DIR)

.PHONY: analyze-enginevector
analyze-enginevector: ## Analyze enginevector.io
	@$(MAKE) analyze-url URL=https://www.enginevector.io

.PHONY: analyze-dir
analyze-dir: build ## Analyze a local directory (use DIR=./path)
ifndef DIR
	$(error DIR is required. Usage: make analyze-dir DIR=./my-site)
endif
	$(RELEASE_DIR)/$(BINARY_NAME) analyze $(DIR)

# ============================================================================
# Optimize Targets
# ============================================================================
.PHONY: optimize-template
optimize-template: build ## Run full optimization on template site
	@mkdir -p $(DIST_DIR)/optimized-site
	$(RELEASE_DIR)/$(BINARY_NAME) run ./site-templates/template-site \
		--site-name "Demo Site" \
		--site-url "https://demo.enginevector.io" \
		--output $(DIST_DIR)/optimized-site
	@echo "$(GREEN)✓ Optimized site in $(DIST_DIR)/optimized-site$(RESET)"

.PHONY: optimize-url
optimize-url: build ## Optimize any URL (URL=, NAME=, SITE_URL=, TWITTER=)
ifndef URL
	$(error URL is required. Usage: make optimize-url URL=https://example.com NAME='My Site')
endif
	@echo "$(CYAN)Fetching $(URL)...$(RESET)"
	@mkdir -p $(TEST_SITE_DIR)
	@curl -sL -o $(TEST_SITE_DIR)/index.html "$(URL)"
	@mkdir -p $(DIST_DIR)/optimized
	$(RELEASE_DIR)/$(BINARY_NAME) run $(TEST_SITE_DIR) \
		--site-name "$(or $(NAME),My Site)" \
		--site-url "$(or $(SITE_URL),$(URL))" \
		$(if $(TWITTER),--twitter $(TWITTER)) \
		$(if $(EMAIL),--email $(EMAIL)) \
		$(if $(IMAGE),--image $(IMAGE)) \
		--output $(DIST_DIR)/optimized
	@echo "$(GREEN)✓ Optimized site in $(DIST_DIR)/optimized$(RESET)"

.PHONY: optimize-enginevector
optimize-enginevector: ## Optimize enginevector.io with full config
	@$(MAKE) optimize-url \
		URL=https://www.enginevector.io \
		NAME="EngineVector" \
		SITE_URL="https://enginevector.io" \
		TWITTER="enginevector" \
		EMAIL="info@enginevector.io"

# ============================================================================
# Report Targets
# ============================================================================
.PHONY: report-url
report-url: build ## Generate SEO report for URL (use URL=https://...)
ifndef URL
	$(error URL is required. Usage: make report-url URL=https://example.com)
endif
	@echo "$(CYAN)Fetching $(URL)...$(RESET)"
	@mkdir -p $(TEST_SITE_DIR)
	@curl -sL -o $(TEST_SITE_DIR)/index.html "$(URL)"
	@mkdir -p $(DIST_DIR)
	$(RELEASE_DIR)/$(BINARY_NAME) report $(TEST_SITE_DIR) --output $(DIST_DIR)/seo-report.md
	@echo "$(GREEN)✓ Report saved to $(DIST_DIR)/seo-report.md$(RESET)"

.PHONY: report-enginevector
report-enginevector: ## Generate SEO report for enginevector.io
	@$(MAKE) report-url URL=https://www.enginevector.io

# ============================================================================
# Native Install Targets
# ============================================================================
.PHONY: install
install: build ## Install binary to system
	@echo "$(CYAN)Installing to $(INSTALL_DIR)...$(RESET)"
	sudo cp $(RELEASE_DIR)/$(BINARY_NAME) $(INSTALL_DIR)/
	@echo "$(GREEN)✓ Installed: $(INSTALL_DIR)/$(BINARY_NAME)$(RESET)"

.PHONY: install-local
install-local: build ## Install to ~/.local/bin
	@mkdir -p ~/.local/bin
	cp $(RELEASE_DIR)/$(BINARY_NAME) ~/.local/bin/
	@echo "$(GREEN)✓ Installed: ~/.local/bin/$(BINARY_NAME)$(RESET)"

.PHONY: uninstall
uninstall: ## Remove installed binary
	sudo rm -f $(INSTALL_DIR)/$(BINARY_NAME)
	rm -f ~/.local/bin/$(BINARY_NAME)
	@echo "$(GREEN)✓ Uninstalled$(RESET)"

# ============================================================================
# Docker Build Targets
# ============================================================================
.PHONY: docker-build
docker-build: ## Build release binary via Docker (no Rust required)
	@echo "$(CYAN)Building via Docker...$(RESET)"
	$(DOCKER_CARGO) cargo build --release --package $(PACKAGE_NAME)
	@echo "$(GREEN)✓ Binary built: $(RELEASE_DIR)/$(BINARY_NAME)$(RESET)"

.PHONY: docker-build-debug
docker-build-debug: ## Build debug binary via Docker
	$(DOCKER_CARGO) cargo build --package $(PACKAGE_NAME)

.PHONY: docker-build-all
docker-build-all: ## Build all crates via Docker
	$(DOCKER_CARGO) cargo build --release --workspace

# ============================================================================
# Docker Test Targets
# ============================================================================
.PHONY: docker-test
docker-test: ## Run all tests via Docker
	@echo "$(CYAN)Running tests via Docker...$(RESET)"
	$(DOCKER_CARGO) cargo test --workspace
	@echo "$(GREEN)✓ All tests passed$(RESET)"

.PHONY: docker-test-verbose
docker-test-verbose: ## Run tests with output via Docker
	$(DOCKER_CARGO) cargo test --workspace -- --nocapture

# ============================================================================
# Docker Quality Targets
# ============================================================================
.PHONY: docker-check
docker-check: ## Check code via Docker
	$(DOCKER_CARGO) cargo check --workspace

.PHONY: docker-clippy
docker-clippy: ## Run Clippy via Docker
	$(DOCKER_CARGO) cargo clippy --workspace --all-targets -- -D warnings

.PHONY: docker-fmt
docker-fmt: ## Format code via Docker
	$(DOCKER_CARGO) cargo fmt --all

.PHONY: docker-fmt-check
docker-fmt-check: ## Check formatting via Docker
	$(DOCKER_CARGO) cargo fmt --all -- --check

.PHONY: docker-lint
docker-lint: docker-fmt-check docker-clippy ## Run all linters via Docker

# ============================================================================
# Docker Run Targets
# ============================================================================
.PHONY: docker-run
docker-run: docker-build ## Run CLI via Docker (use ARGS='...')
	@echo "$(CYAN)Running via Docker...$(RESET)"
	docker run $(DOCKER_RUN_OPTS) rust:$(RUST_VERSION)-bookworm \
		./target/release/$(BINARY_NAME) $(ARGS)

.PHONY: docker-analyze-template
docker-analyze-template: docker-build ## Analyze template site via Docker
	docker run $(DOCKER_RUN_OPTS) rust:$(RUST_VERSION)-bookworm \
		./target/release/$(BINARY_NAME) analyze /workspace/site-templates/template-site

.PHONY: docker-shell
docker-shell: ## Open shell in Docker build environment
	docker run -it $(DOCKER_RUN_OPTS) rust:$(RUST_VERSION)-bookworm bash

# ============================================================================
# Docker Image Targets
# ============================================================================
.PHONY: docker-image
docker-image: ## Build production Docker image
	@echo "$(CYAN)Building Docker image...$(RESET)"
	docker build -t $(IMAGE_NAME):$(IMAGE_TAG) .
	@echo "$(GREEN)✓ Image built: $(IMAGE_NAME):$(IMAGE_TAG)$(RESET)"

.PHONY: docker-image-dev
docker-image-dev: ## Build development Docker image
	docker build --target builder -t $(IMAGE_NAME):dev .

.PHONY: docker-push
docker-push: docker-image ## Push image to registry
	docker push $(IMAGE_NAME):$(IMAGE_TAG)

# ============================================================================
# Distribution Targets
# ============================================================================
.PHONY: dist
dist: build ## Create distribution package
	@echo "$(CYAN)Creating distribution package...$(RESET)"
	@mkdir -p $(DIST_DIR)
	@cp $(RELEASE_DIR)/$(BINARY_NAME) $(DIST_DIR)/
	@cp README.md QUICKSTART.md $(DIST_DIR)/
	@tar -czvf $(DIST_DIR)/$(BINARY_NAME)-$(shell uname -s)-$(shell uname -m).tar.gz \
		-C $(DIST_DIR) $(BINARY_NAME) README.md QUICKSTART.md
	@echo "$(GREEN)✓ Distribution package created in $(DIST_DIR)/$(RESET)"

.PHONY: dist-all
dist-all: ## Create multi-platform distributions (requires cross)
	@echo "$(CYAN)Building for multiple platforms...$(RESET)"
	@mkdir -p $(DIST_DIR)
	# Linux x86_64
	cross build --release --target x86_64-unknown-linux-gnu --package $(PACKAGE_NAME)
	@cp target/x86_64-unknown-linux-gnu/release/$(BINARY_NAME) $(DIST_DIR)/$(BINARY_NAME)-linux-x86_64
	# Linux ARM64
	cross build --release --target aarch64-unknown-linux-gnu --package $(PACKAGE_NAME)
	@cp target/aarch64-unknown-linux-gnu/release/$(BINARY_NAME) $(DIST_DIR)/$(BINARY_NAME)-linux-arm64
	# macOS x86_64
	cross build --release --target x86_64-apple-darwin --package $(PACKAGE_NAME)
	@cp target/x86_64-apple-darwin/release/$(BINARY_NAME) $(DIST_DIR)/$(BINARY_NAME)-darwin-x86_64
	# macOS ARM64
	cross build --release --target aarch64-apple-darwin --package $(PACKAGE_NAME)
	@cp target/aarch64-apple-darwin/release/$(BINARY_NAME) $(DIST_DIR)/$(BINARY_NAME)-darwin-arm64
	@echo "$(GREEN)✓ Multi-platform builds complete$(RESET)"

# ============================================================================
# Documentation Targets
# ============================================================================
.PHONY: docs
docs: ## Generate documentation
	@echo "$(CYAN)Generating documentation...$(RESET)"
	cargo doc --workspace --no-deps --open

.PHONY: docs-build
docs-build: ## Build documentation (no open)
	cargo doc --workspace --no-deps

.PHONY: docker-docs
docker-docs: ## Generate documentation via Docker
	$(DOCKER_CARGO) cargo doc --workspace --no-deps

# ============================================================================
# Benchmark Targets
# ============================================================================
.PHONY: bench
bench: ## Run benchmarks
	cargo bench --workspace

.PHONY: docker-bench
docker-bench: ## Run benchmarks via Docker
	$(DOCKER_CARGO) cargo bench --workspace

# ============================================================================
# Clean Targets
# ============================================================================
.PHONY: clean
clean: ## Clean build artifacts
	@echo "$(CYAN)Cleaning build artifacts...$(RESET)"
	cargo clean
	rm -rf $(DIST_DIR) $(TEST_SITE_DIR)
	@echo "$(GREEN)✓ Clean complete$(RESET)"

.PHONY: clean-docker
clean-docker: ## Clean Docker build cache
	@echo "$(CYAN)Cleaning Docker cache...$(RESET)"
	docker builder prune -f
	docker image rm -f $(IMAGE_NAME):$(IMAGE_TAG) $(IMAGE_NAME):dev 2>/dev/null || true
	@echo "$(GREEN)✓ Docker cache cleaned$(RESET)"

.PHONY: clean-all
clean-all: clean clean-docker ## Clean everything

# ============================================================================
# Development Targets
# ============================================================================
.PHONY: dev
dev: ## Start development (watch mode - requires cargo-watch)
	cargo watch -x 'check --workspace' -x 'test --workspace' -x 'clippy --workspace'

.PHONY: dev-run
dev-run: ## Watch and run on changes
	cargo watch -x 'run --package $(PACKAGE_NAME) -- analyze ./site-templates/template-site'

.PHONY: setup
setup: ## Install development dependencies
	@echo "$(CYAN)Installing development tools...$(RESET)"
	rustup component add clippy rustfmt
	cargo install cargo-watch cargo-audit cargo-outdated
	@echo "$(GREEN)✓ Development tools installed$(RESET)"

.PHONY: update
update: ## Update dependencies
	@echo "$(CYAN)Updating dependencies...$(RESET)"
	cargo update
	@echo "$(GREEN)✓ Dependencies updated$(RESET)"

.PHONY: outdated
outdated: ## Check for outdated dependencies
	cargo outdated

# ============================================================================
# CI/CD Targets
# ============================================================================
.PHONY: ci
ci: fmt-check clippy test ## Run CI pipeline
	@echo "$(GREEN)✓ CI pipeline passed$(RESET)"

.PHONY: docker-ci
docker-ci: docker-fmt-check docker-clippy docker-test ## Run CI pipeline via Docker
	@echo "$(GREEN)✓ Docker CI pipeline passed$(RESET)"

.PHONY: release
release: lint test dist ## Prepare release
	@echo "$(GREEN)✓ Release ready$(RESET)"

# ============================================================================
# Version Info
# ============================================================================
.PHONY: version
version: ## Show version info
	@echo "$(BOLD)site-ranker-rs$(RESET)"
	@echo "  Rust: $(shell rustc --version 2>/dev/null || echo 'not installed')"
	@echo "  Cargo: $(shell cargo --version 2>/dev/null || echo 'not installed')"
	@echo "  Docker: $(shell docker --version 2>/dev/null || echo 'not installed')"
	@echo "  Image: $(IMAGE_NAME):$(IMAGE_TAG)"

# ============================================================================
# Phony declarations for make tab completion
# ============================================================================
.PHONY: all
all: build test lint docs ## Build, test, lint, and generate docs