# Helper Scripts

Three scripts to make your life easier:

## 🚀 setup.sh - Initial Setup

**What it does:**
- Checks Rust, Docker, HelixDB CLI
- Installs HelixDB CLI if needed
- Builds the project

**Run once:**
```bash
./setup.sh
```

**Requirements:**
- Rust 1.88.0+
- Docker Desktop (installed and running)

---

## ⚡ quick_test.sh - Test Without Docker

**What it does:**
- Tests arXiv integration only
- No HelixDB/Docker needed
- Shows that arXiv fetching works

**Run anytime:**
```bash
./quick_test.sh
```

**Perfect for:**
- Testing arXiv API
- Verifying project builds
- When Docker isn't available

---

## 🎯 run_pipeline.sh - Full RAG Pipeline

**What it does:**
1. Checks Docker is running
2. Deploys HelixDB (`helix push dev`)
3. Tests connection
4. Runs full pipeline:
   - Fetches papers from arXiv
   - Generates embeddings
   - Stores in HelixDB
   - Performs semantic search

**Run after setup:**
```bash
./run_pipeline.sh
```

**This is the main event!**

---

## Quick Reference

### First Time Setup
```bash
# 1. Install Docker Desktop (if not installed)
# Download from: https://www.docker.com/products/docker-desktop/

# 2. Start Docker Desktop app

# 3. Run setup
./setup.sh

# 4. Run full pipeline
./run_pipeline.sh
```

### Daily Usage
```bash
# Start HelixDB
helix push dev

# Run examples
cargo run --example helix_integration
cargo run --example semantic_search
cargo run --example ingest_papers

# Stop HelixDB
helix stop dev
```

### Testing Without Docker
```bash
./quick_test.sh
```

---

## Troubleshooting

### "Docker not found"
**Fix:** Install Docker Desktop, then run `./setup.sh`

### "Docker is not running"
**Fix:** Start Docker Desktop app, wait for it to fully start

### "HelixDB CLI not found"
**Fix:** Script will auto-install, or run manually:
```bash
curl -sSL https://install.helix-db.com | bash
```

### "Build failed"
**Fix:**
```bash
cargo clean
cargo build
```

---

## What Each Script Checks

### setup.sh
- ✓ Rust installed and version
- ✓ Docker installed and running
- ✓ HelixDB CLI installed
- ✓ Project builds

### run_pipeline.sh
- ✓ Docker running
- ✓ HelixDB CLI available
- ✓ Deploys HelixDB
- ✓ Tests connection
- ✓ Runs full integration

### quick_test.sh
- ✓ Project builds
- ✓ arXiv API works

---

## Manual Commands

If you prefer not to use scripts:

### Deploy HelixDB
```bash
helix push dev
```

### Check it's running
```bash
cargo run --example check_helix
```

### Test arXiv only
```bash
cargo run --example arxiv_search
```

### Full pipeline
```bash
cargo run --example helix_integration
```

### Interactive search
```bash
cargo run --example semantic_search
```

### Stop HelixDB
```bash
helix stop dev
```

---

## Output Examples

### Successful run_pipeline.sh
```
═══════════════════════════════════════════════════
   arXiv TUI - Full RAG Pipeline Test
═══════════════════════════════════════════════════

[1/5] Checking Docker...
✓ Docker is running

[2/5] Checking HelixDB CLI...
✓ Helix CLI 2.1.3

[3/5] Deploying HelixDB...
✓ HelixDB deployed successfully

[4/5] Testing HelixDB connection...
✓ Connection successful

[5/5] Running full RAG pipeline...
[Pipeline output...]

═══════════════════════════════════════════════════
   Pipeline Complete! ✓
═══════════════════════════════════════════════════
```

---

## Pro Tips

1. **First time?** Run `./setup.sh` then `./run_pipeline.sh`
2. **No Docker yet?** Run `./quick_test.sh` to verify arXiv works
3. **Daily dev?** Just `helix push dev` and use cargo examples
4. **Debugging?** Check each step manually instead of using scripts

---

## Next Steps After Success

Once `run_pipeline.sh` succeeds:

1. **Ingest more papers:**
   ```bash
   cargo run --example ingest_papers
   ```

2. **Interactive search:**
   ```bash
   cargo run --example semantic_search
   ```

3. **Start the TUI:**
   ```bash
   cargo run
   ```

4. **Build features:**
   - Implement search UI
   - Add paper detail view
   - Create collections
   - Download PDFs

Happy hacking! 🚀
