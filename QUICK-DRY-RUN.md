## QUICK TEST PLAN

```shell
# 1. Extract and build
tar -xzf site-ranker-rs.tar.gz
cd site-ranker-rs
make build   # or make docker-build if no local Rust

# 2. Analyze enginevector.io (we'll need to fetch it first)
# Option A: curl the page locally
mkdir -p test-site
curl -o test-site/index.html https://www.enginevector.io
./target/release/site-ranker analyze ./test-site

# Option B: Add web fetch capability (future feature)

# 3. Full dry-run optimization
./target/release/site-ranker run ./test-site \
  --site-name "EngineVector" \
  --site-url "https://enginevector.io" \
  --twitter "enginevector" \
  --email "info@enginevector.io" \
  --output ./optimized-enginevector

# 4. Compare before/after
diff test-site/index.html optimized-enginevector/index.html
```
