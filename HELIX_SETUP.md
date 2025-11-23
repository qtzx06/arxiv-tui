# HelixDB Setup Guide

This guide will help you set up HelixDB and run the full RAG pipeline.

## Prerequisites

1. **HelixDB installed** - Follow [HelixDB installation guide](https://docs.helix-db.com/getting-started)
2. **Rust 1.88.0+** - Already installed ✓
3. **arXiv TUI built** - `cargo build` ✓

## Step-by-Step Setup

### 1. Start HelixDB Server

```bash
# Start HelixDB on default port 6969
helix serve

# Or specify custom port
helix serve --port 6969
```

**Expected output:**
```
HelixDB Server starting on localhost:6969
```

Keep this terminal open - the server needs to stay running.

### 2. Load Database Schema

In a new terminal:

```bash
cd helix

# Load the schema (defines nodes, vectors, edges)
helix load schema.hx

# Load the queries (defines all HQL operations)
helix load queries.hx
```

**Expected output:**
```
✓ Schema loaded successfully
✓ Queries loaded successfully
```

You can verify by checking:
```bash
helix show schema
helix show queries
```

### 3. Test HelixDB Connection

```bash
# Quick test of the integration
cargo run --example helix_integration
```

This will:
- Fetch 5 papers from arXiv about "transformer neural networks"
- Generate embeddings for each paper
- Store them in HelixDB
- Perform a semantic search query

**Expected output:**
```
=== arXiv + HelixDB RAG Pipeline ===

1. Initializing clients...
   ✓ Connected to HelixDB at http://localhost:6969
   ✓ Embedding generator ready (placeholder mode)

2. Fetching papers from arXiv...
   ✓ Found 5 papers

3. Processing papers and storing in HelixDB...
   Processing [1/5]: Attention Is All You Need
     ✓ Generated 384-dimensional embedding
     ✓ Stored paper vector in HelixDB
     ✓ Stored paper metadata

   [... more papers ...]

4. Performing semantic search...
   Query: "attention mechanism in deep learning"
   ✓ Generated query embedding
   ✓ Semantic search results:
   [JSON with similar papers]

=== Pipeline Complete ===
```

## Main Examples

### Example 1: Full Integration Test

```bash
cargo run --example helix_integration
```

**What it does:**
- Fetches papers from arXiv
- Generates embeddings
- Stores in HelixDB
- Runs semantic search

**Use case:** Testing the full pipeline

### Example 2: Ingest Papers in Bulk

```bash
cargo run --example ingest_papers
```

**What it does:**
- Fetches 10 latest papers from each category (AI, ML, NLP, Vision)
- Generates embeddings for all
- Stores in HelixDB

**Use case:** Building your paper database

### Example 3: Interactive Semantic Search

```bash
cargo run --example semantic_search
```

**What it does:**
- Interactive search CLI
- Type queries in natural language
- Get semantically similar papers

**Use case:** Testing search quality

## Verify It's Working

### Check HelixDB has data

```bash
# Using helix CLI
helix query "CountPapers"
```

Should return the count of papers stored.

### Check a specific paper

```bash
helix query "GetPaperByArxivId" '{"arxiv_id": "1706.03762"}'
```

Should return the "Attention is All You Need" paper if it was ingested.

## Troubleshooting

### "Connection refused" error

**Problem:** HelixDB is not running

**Solution:**
```bash
# Start HelixDB in a separate terminal
helix serve
```

### "Schema not found" or "Query not found" error

**Problem:** Schema/queries not loaded

**Solution:**
```bash
cd helix
helix load schema.hx
helix load queries.hx
```

### "Failed to store paper" error

**Problem:** Schema might not match queries

**Solution:**
```bash
# Reload both
helix load schema.hx
helix load queries.hx

# Or restart HelixDB
# Ctrl+C to stop, then:
helix serve
```

### Embeddings are placeholder

**Problem:** ONNX model not downloaded (this is OK for testing!)

**Current:** Using deterministic hash-based "embeddings"
**Future:** Download real model (see `models/README.md`)

The placeholder embeddings still work for testing the pipeline!

## What's Stored in HelixDB?

### Vectors (V::Paper)
- arXiv ID
- Title
- Authors
- Categories
- Abstract preview (200 chars)
- **384-dimensional embedding vector**

### Nodes (N::PaperMetadata)
- Full abstract
- Journal references
- DOI
- PDF URL
- Saved/read status
- Tags

### Edges
- `HasMetadata` - Links vector to full metadata
- `SimilarTo` - Links similar papers (computed later)
- `InCollection` - Papers in user collections
- `Cites` - Citation relationships

## Full Workflow Example

```bash
# Terminal 1: Start HelixDB
helix serve

# Terminal 2: Load schema
cd helix
helix load schema.hx
helix load queries.hx

# Terminal 3: Ingest papers
cargo run --example ingest_papers
# Wait ~2 minutes (40 papers × 3 sec delay)

# Terminal 3: Try semantic search
cargo run --example semantic_search
# Type: "transformers for machine translation"
```

## Next Steps

Once HelixDB integration works:

1. **Build TUI Search View** - Wire up UI to semantic search
2. **Download Real ONNX Model** - Better embeddings
3. **Implement Collections** - Save favorite papers
4. **Add Similar Papers** - Graph traversal
5. **PDF Downloads** - Local storage

## Performance Notes

- **Rate Limiting:** arXiv requires 3 seconds between requests
- **Ingestion Time:** 10 papers = ~30 seconds
- **Search Speed:** Sub-second after embeddings generated
- **Storage:** ~1KB per paper vector + metadata

## Resources

- [HelixDB Docs](https://docs.helix-db.com)
- [HQL Reference](https://docs.helix-db.com/hql)
- [arXiv API](https://arxiv.org/help/api)
- Schema: `helix/schema.hx`
- Queries: `helix/queries.hx`
