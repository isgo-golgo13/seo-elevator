# Quick Start Guide

Get site-ranker-rs running in under 2 minutes.

## Prerequisites

- Rust 1.75+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- A website to optimize (or use our template)

## 1. Install

```bash
# Clone
git clone https://github.com/enginevector/site-ranker-rs.git
cd site-ranker-rs

# Build
cargo build --release

# Add to PATH (optional)
cp target/release/site-ranker ~/.local/bin/
```

## 2. Test with Template Site

```bash
# Analyze the included template
./target/release/site-ranker analyze ./site-templates/template-site

# You'll see output like:
# SEO Score: 15/100
# Optimization Score: 35/100
```

## 3. Optimize the Template

```bash
./target/release/site-ranker run ./site-templates/template-site \
  --site-name "Acme Corp" \
  --site-url "https://acme.com" \
  --twitter "acmecorp" \
  --output ./optimized-site
```

## 4. Check the Results

```bash
# View the optimized HTML
cat ./optimized-site/index.html

# You'll see injected:
# - Meta tags (title, description, keywords)
# - Open Graph tags
# - Twitter Cards  
# - Schema.org JSON-LD
```

## 5. Use on Your Project

```bash
# Point to your website directory
site-ranker run ./path/to/your/website \
  --site-name "Your Site" \
  --site-url "https://yoursite.com" \
  --output ./optimized
```

## Common Workflows

### Just Analyze (No Changes)

```bash
site-ranker analyze ./my-site
site-ranker analyze ./my-site --output report.json --format json
```

### Dry Run (Preview Changes)

```bash
site-ranker inject ./my-site --dry-run
```

### Generate Report

```bash
site-ranker report ./my-site --output seo-report.md
```

### Docker Usage

```bash
# Build image
docker build -t site-ranker .

# Run analysis
docker run -v $(pwd)/my-site:/workspace site-ranker analyze /workspace

# Run optimization
docker run -v $(pwd)/my-site:/workspace -v $(pwd)/output:/output \
  site-ranker run /workspace --output /output \
  --site-name "My Site" --site-url "https://mysite.com"
```

## What Gets Injected?

### Meta Tags
```html
<title>Your Optimized Title | Site Name</title>
<meta name="description" content="AI-generated compelling description...">
<meta name="keywords" content="keyword1, keyword2, keyword3">
<meta name="robots" content="index, follow">
<link rel="canonical" href="https://yoursite.com">
```

### Open Graph
```html
<meta property="og:type" content="website">
<meta property="og:title" content="Your Title">
<meta property="og:description" content="Your description">
<meta property="og:url" content="https://yoursite.com">
<meta property="og:image" content="https://yoursite.com/og-image.png">
```

### Twitter Cards
```html
<meta name="twitter:card" content="summary_large_image">
<meta name="twitter:title" content="Your Title">
<meta name="twitter:description" content="Your description">
<meta name="twitter:site" content="@yourhandle">
```

### Schema.org JSON-LD
```html
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@graph": [
    { "@type": "Organization", ... },
    { "@type": "WebSite", ... },
    { "@type": "BreadcrumbList", ... }
  ]
}
</script>
```

## Next Steps

1. **Read the full [README](README.md)** for detailed configuration
2. **Explore the ML features** with `--verbose` flag
3. **Integrate into CI/CD** for automatic SEO optimization
4. **Star the repo** ⭐ if you find it useful!

---

**Questions?** Open an issue on GitHub or reach out at [enginevector.io](https://enginevector.io)
