# arXiv TUI

A high-performance terminal user interface for searching, browsing, and managing arXiv papers with semantic search capabilities powered by HelixDB.

## Features

- **Semantic Search**: Find papers by meaning, not just keywords
- **Fast Vector Search**: Powered by HelixDB graph-vector database
- **Terminal UI**: Vim-like keybindings for efficient navigation
- **Paper Management**: Save, organize, and annotate papers
- **Citation Graph**: Explore relationships between papers
- **PDF Downloads**: Download and manage papers locally

## Quick Start

**Get running in 5 minutes!** See [QUICKSTART.md](./QUICKSTART.md)

```bash
# 1. Start HelixDB
helix serve

# 2. Load schema
cd helix && helix load schema.hx && helix load queries.hx

# 3. Run full pipeline
cargo run --example helix_integration
```

## Examples

### Check HelixDB Connection
```bash
cargo run --example check_helix
```

### Full RAG Pipeline
```bash
# Fetch papers, generate embeddings, store in HelixDB, semantic search
cargo run --example helix_integration
```

### Ingest Papers
```bash
# Add 40 papers to database (AI, ML, NLP, Vision)
cargo run --example ingest_papers
```

### Interactive Semantic Search
```bash
# Search papers by natural language queries
cargo run --example semantic_search
```

### arXiv API Only
```bash
# Test arXiv integration without database
cargo run --example arxiv_search
```

## Prerequisites

- Rust 1.88.0 or higher (`rustup update`)
- HelixDB instance running (see [HELIX_SETUP.md](./HELIX_SETUP.md))
- ONNX model for embeddings (optional, placeholder works for testing)

## Detailed Setup

### 1. Install HelixDB

Follow the [HelixDB installation guide](https://docs.helix-db.com/getting-started).

Start HelixDB:
```bash
helix serve
```

### 2. Download Embedding Model

Download the all-MiniLM-L6-v2 ONNX model and tokenizer:

```bash
# Create models directory
mkdir -p models

# Download model (placeholder - you'll need to get this from HuggingFace)
# Visit: https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2
# Download: model.onnx -> models/all-MiniLM-L6-v2.onnx
# Download: tokenizer.json -> models/tokenizer.json
```

### 3. Initialize HelixDB Schema

```bash
# Navigate to helix directory
cd helix

# Load schema and queries into HelixDB
helix load schema.hx
helix load queries.hx
```

### 4. Build and Run

```bash
# Build the project
cargo build --release

# Run the application
cargo run --release
```

## Configuration

Edit `config/default.toml` to customize:

- HelixDB connection settings
- Embedding model settings
- UI preferences (theme, keybindings)
- Cache and storage locations

## Usage

### Keybindings (Vim mode)

- `q` - Quit
- `/` - Search
- `j/k` - Navigate up/down
- `Enter` - View paper details
- `s` - Save paper
- `d` - Download PDF
- `?` - Help

### Search Modes

1. **Semantic Search** - Type your query naturally
2. **Category Browse** - Browse by arXiv category
3. **Library** - View saved papers

## Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed system design.

```
├── src/
│   ├── arxiv/        # arXiv API client
│   ├── db/           # HelixDB integration
│   ├── embeddings/   # ONNX embedding generation
│   ├── ui/           # TUI views and components
│   └── core/         # Application logic
├── helix/            # HelixDB schema and queries
├── config/           # Configuration files
└── models/           # ONNX models
```

## Development

```bash
# Run with debug logging
RUST_LOG=debug cargo run

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

## Roadmap

- [x] Basic project structure
- [ ] arXiv API integration
- [ ] HelixDB vector search
- [ ] Embedding generation
- [ ] Search UI
- [ ] Paper detail view
- [ ] Collections/library
- [ ] PDF download
- [ ] Citation graph visualization

## Contributing

Contributions welcome! Please check out the [architecture docs](ARCHITECTURE.md) before starting.

## License

MIT

## Credits

Built with:
- [HelixDB](https://github.com/HelixDB/helix-db) - Graph-vector database
- [ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI framework
- [ONNX Runtime](https://onnxruntime.ai/) - ML inference
