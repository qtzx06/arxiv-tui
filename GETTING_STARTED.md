# Getting Started with arXiv TUI Development

This guide will help you get the project running and understand the next steps for development.

## Project Status

The project has been scaffolded with the complete architecture in place. Here's what's done:

- [x] Project structure and Cargo workspace
- [x] HelixDB schema and queries (HQL)
- [x] Configuration system
- [x] arXiv API client (with XML parsing)
- [x] HelixDB client wrapper
- [x] Embedding generator (placeholder implementation)
- [x] Basic TUI framework
- [x] Application state management
- [x] Module organization

## Quick Start

### 1. Verify Rust Installation

```bash
rustc --version  # Should be 1.88.0+
```

If not, update Rust:
```bash
rustup update
```

### 2. Build the Project

```bash
cargo build
```

This will download all dependencies (~360 crates) and compile the project.

### 3. Set Up HelixDB

You need a running HelixDB instance:

```bash
# Install HelixDB (follow official docs)
# https://docs.helix-db.com/getting-started

# Start HelixDB server
helix serve
```

### 4. Load Schema and Queries

```bash
cd helix

# Load the schema
helix load schema.hx

# Load the queries
helix load queries.hx
```

### 5. Run the Application

```bash
cargo run
```

You should see a basic TUI with placeholder views. Press 'q' to quit.

## Project Structure

```
arxiv-tui/
├── src/
│   ├── main.rs              # Entry point
│   ├── app.rs               # Main application loop
│   ├── config.rs            # Configuration loader
│   │
│   ├── arxiv/               # arXiv API client
│   │   ├── client.rs        # HTTP client for arXiv
│   │   ├── models.rs        # Paper data models
│   │   └── parser.rs        # XML response parser
│   │
│   ├── db/                  # HelixDB integration
│   │   ├── client.rs        # HelixDB client wrapper
│   │   └── models.rs        # DB response models
│   │
│   ├── embeddings/          # Embedding generation
│   │   ├── generator.rs     # ONNX model inference (placeholder)
│   │   └── models.rs        # Model utilities
│   │
│   ├── ui/                  # TUI components
│   │   ├── app.rs           # Main UI renderer
│   │   ├── views/           # Individual views
│   │   └── components/      # Reusable UI components
│   │
│   ├── core/                # Business logic
│   │   ├── state.rs         # Application state
│   │   ├── events.rs        # Event handling
│   │   └── commands.rs      # Command palette
│   │
│   └── utils/               # Utilities
│       └── cache.rs         # Caching system
│
├── helix/                   # HelixDB definitions
│   ├── schema.hx            # Database schema
│   └── queries.hx           # HQL queries
│
├── config/
│   └── default.toml         # Default configuration
│
└── models/                  # ONNX models (download separately)
    └── README.md
```

## Next Steps for Development

### Phase 1: Core Functionality (Recommended Order)

1. **Test arXiv Client**
   - Create `tests/integration/arxiv_test.rs`
   - Test search, get_by_id, get_latest functions
   - Verify XML parsing works correctly

2. **Test HelixDB Client**
   - Create simple integration test
   - Verify connection to HelixDB
   - Test adding papers and searching

3. **Implement Search View**
   - File: `src/ui/views/search.rs`
   - Add search input component
   - Display search results in a list
   - Wire up to arXiv API

4. **Connect Embedding Generator**
   - Download ONNX model (see `models/README.md`)
   - Implement real ONNX inference
   - Test embedding generation

5. **Implement Semantic Search**
   - Generate embeddings for papers
   - Store in HelixDB
   - Query by similarity
   - Display results

### Phase 2: Enhanced Features

6. **Paper Detail View**
   - File: `src/ui/views/detail.rs`
   - Show full abstract
   - Display metadata
   - Show similar papers

7. **Collections/Library**
   - File: `src/ui/views/library.rs`
   - Create collections
   - Add/remove papers
   - Organize saved papers

8. **PDF Downloads**
   - Implement download_pdf function
   - Progress indicators
   - Local storage management

### Phase 3: Polish

9. **Keybindings & Commands**
   - Implement vim-like navigation
   - Add command palette
   - Keyboard shortcuts

10. **Error Handling**
    - Better error messages
    - Retry logic for API calls
    - Graceful degradation

## Development Tips

### Running with Debug Logs

```bash
RUST_LOG=debug cargo run
```

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test arxiv_client

# Integration tests only
cargo test --test '*'
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Fix clippy warnings automatically
cargo clippy --fix
```

### Hot Reloading

Use `cargo-watch` for automatic recompilation:

```bash
cargo install cargo-watch
cargo watch -x run
```

## Common Tasks

### Adding a New Dependency

```bash
cargo add <crate-name>
```

### Adding a New HQL Query

1. Edit `helix/queries.hx`
2. Add query definition
3. Reload in HelixDB: `helix load queries.hx`
4. Add Rust wrapper in `src/db/client.rs`

### Adding a New UI View

1. Create file in `src/ui/views/`
2. Add to `src/ui/views/mod.rs`
3. Add enum variant to `View` in `src/core/state.rs`
4. Add render case in `src/ui/app.rs`

## Configuration

Edit `config/default.toml` to customize:

```toml
[helixdb]
endpoint = "http://localhost"
port = 6969

[embeddings]
model_path = "./models/all-MiniLM-L6-v2.onnx"
dimension = 384

[ui]
theme = "dark"
keybindings = "vim"
```

## Troubleshooting

### HelixDB Connection Error

- Ensure HelixDB is running: `helix serve`
- Check port configuration in `config/default.toml`
- Verify schema is loaded: `helix load helix/schema.hx`

### Embedding Model Not Found

- Download model (see `models/README.md`)
- Update `model_path` in config
- For now, placeholder implementation works

### Compilation Errors

```bash
# Clean and rebuild
cargo clean
cargo build
```

## Resources

- [HelixDB Docs](https://docs.helix-db.com)
- [Ratatui Docs](https://ratatui.rs)
- [arXiv API Docs](https://arxiv.org/help/api)
- [Architecture Guide](./ARCHITECTURE.md)

## Getting Help

1. Check architecture docs
2. Look at similar components
3. Run with debug logging
4. Check error messages

Happy coding!
