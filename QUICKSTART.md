# Quick Start - Full RAG Pipeline

Get the arXiv TUI with semantic search running in 5 minutes!

## What You'll Build

A working system that:
1. **Fetches** papers from arXiv
2. **Generates** semantic embeddings
3. **Stores** them in HelixDB
4. **Searches** semantically (find papers by meaning, not just keywords)

## Prerequisites Check

```bash
# 1. Rust installed?
rustc --version  # Should be 1.88.0+

# 2. HelixDB installed?
helix --version  # Install from: https://docs.helix-db.com

# 3. Project built?
cargo build
```

## 5-Minute Setup

### Step 1: Start HelixDB (Terminal 1)

```bash
helix serve
```

**Expected:**
```
🚀 HelixDB Server running on http://localhost:6969
```

**Keep this terminal open!**

### Step 2: Load Schema (Terminal 2)

```bash
cd helix
helix load schema.hx
helix load queries.hx
```

**Expected:**
```
✓ Schema loaded successfully
✓ Queries loaded successfully
```

### Step 3: Check Connection

```bash
cargo run --example check_helix
```

**Expected:**
```
✓ HelixDB is accessible!
```

### Step 4: Run Full Pipeline

```bash
cargo run --example helix_integration
```

**Expected:**
```
=== arXiv + HelixDB RAG Pipeline ===

1. Initializing clients...
   ✓ Connected to HelixDB at http://localhost:6969
   ✓ Embedding generator ready

2. Fetching papers from arXiv...
   ✓ Found 5 papers

3. Processing papers and storing in HelixDB...
   [1/5] Processing: Attention Is All You Need
     ✓ Generated 384-dimensional embedding
     ✓ Stored paper vector in HelixDB
     ✓ Stored paper metadata

   [... 4 more papers ...]

4. Performing semantic search...
   Query: "attention mechanism in deep learning"
   ✓ Semantic search results:
   {
     "results": [
       {
         "arxiv_id": "1706.03762",
         "title": "Attention Is All You Need",
         "similarity": 0.89
       },
       ...
     ]
   }

=== Pipeline Complete ===
```

## 🎉 It Works!

You now have a semantic paper search system!

## Try It Out

### 1. Ingest More Papers

```bash
# Add 40 papers (10 from each major category)
cargo run --example ingest_papers
```

**Time:** ~2 minutes (arXiv rate limits)

### 2. Interactive Search

```bash
cargo run --example semantic_search
```

**Try queries like:**
- "transformers for machine translation"
- "computer vision for autonomous driving"
- "reinforcement learning in robotics"
- "natural language understanding"

**Expected:**
```
=== Semantic Paper Search ===

Connected to HelixDB
Ready for semantic search!

> transformers for machine translation

Searching for: "transformers for machine translation"
Generated embedding (384 dimensions)

📄 Search Results:
[Papers semantically related to translation...]
```

### 3. Simple arXiv Search (No DB)

```bash
cargo run --example arxiv_search
```

Fetches papers directly from arXiv (no HelixDB needed).

## What Just Happened?

### The Pipeline

```
arXiv API → Paper → Embedding Generator → HelixDB
                         ↓
              384-dim vector stored
                         ↓
          Query → Embedding → Vector Search → Results
```

### The Stack

- **arXiv Client**: Fetches papers via REST API
- **Embedding Generator**: Converts text → vectors (currently placeholder)
- **HelixDB**: Stores vectors + metadata, performs similarity search
- **Queries**: HQL (Helix Query Language) for graph-vector operations

### What's Stored

**Per Paper:**
- **Vector** (384 floats): Semantic representation
- **Metadata**: Title, authors, abstract, PDF URL, etc.
- **Edges**: Links to similar papers, collections, citations

## Next Steps

### Improve Embeddings (Optional)

Currently using deterministic hash-based embeddings. For production:

1. Download ONNX model (see `models/README.md`)
2. Replace placeholder in `src/embeddings/generator.rs`
3. Re-ingest papers

### Build the TUI

```bash
# Start the TUI (basic version)
cargo run

# Press 'q' to quit
```

Currently shows placeholder views. Next: wire up search!

### Add Features

1. **Collections** - Save favorite papers
2. **Similar Papers** - Graph traversal
3. **PDF Downloads** - Local storage
4. **Citation Graph** - Explore relationships

## Troubleshooting

### HelixDB not running

**Error:** `Connection refused`

**Fix:**
```bash
# Terminal 1
helix serve
```

### Schema not loaded

**Error:** `Query 'AddPaper' not found`

**Fix:**
```bash
cd helix
helix load schema.hx
helix load queries.hx
```

### Rate limit hit

**Error:** `429 Too Many Requests`

**Why:** arXiv requires 3 seconds between requests

**Fix:** Wait and try again (built-in delay handles this)

### No results from search

**Why:** No papers ingested yet

**Fix:**
```bash
cargo run --example ingest_papers
```

## Commands Summary

```bash
# Check everything works
cargo run --example check_helix

# Quick test (5 papers)
cargo run --example helix_integration

# Ingest papers (40 papers)
cargo run --example ingest_papers

# Interactive search
cargo run --example semantic_search

# arXiv only (no DB)
cargo run --example arxiv_search

# Start TUI
cargo run
```

## Architecture

See `ARCHITECTURE.md` for detailed design.

Key files:
- `src/arxiv/` - arXiv API client
- `src/db/` - HelixDB integration
- `src/embeddings/` - Embedding generation
- `helix/schema.hx` - Database schema
- `helix/queries.hx` - HQL queries

## Resources

- [HelixDB Setup Guide](./HELIX_SETUP.md) - Detailed setup
- [Architecture](./ARCHITECTURE.md) - System design
- [Getting Started](./GETTING_STARTED.md) - Development guide

Ready to search papers semantically! 🚀
