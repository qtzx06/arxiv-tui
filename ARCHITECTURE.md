# arXiv TUI Architecture

## Overview
A high-performance terminal user interface for searching, browsing, and managing arXiv papers with semantic search capabilities powered by HelixDB's graph-vector database.

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         TUI Layer                           │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │  Search  │  │  Browse  │  │  Paper   │  │ Library  │   │
│  │   View   │  │   View   │  │  Detail  │  │   View   │   │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                    Application Core                         │
│  ┌────────────────┐  ┌──────────────┐  ┌────────────────┐  │
│  │  State Manager │  │ Event Handler│  │  Config Manager│  │
│  └────────────────┘  └──────────────┘  └────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                            │
        ┌───────────────────┼───────────────────┐
        ▼                   ▼                   ▼
┌───────────────┐  ┌─────────────────┐  ┌──────────────┐
│  arXiv Client │  │  HelixDB Client │  │   Embedding  │
│               │  │                 │  │   Generator  │
│  - Fetch      │  │  - Vector Store │  │              │
│  - Search     │  │  - Graph Queries│  │  - Generate  │
│  - Download   │  │  - RAG Search   │  │  - Batch     │
└───────────────┘  └─────────────────┘  └──────────────┘
        │                   │                   │
        ▼                   ▼                   ▼
┌───────────────┐  ┌─────────────────┐  ┌──────────────┐
│  arXiv API    │  │    HelixDB      │  │ ONNX Runtime │
│  (REST)       │  │   Instance      │  │ (Model)      │
└───────────────┘  └─────────────────┘  └──────────────┘
```

## Component Breakdown

### 1. TUI Layer (Presentation)
**Framework**: `ratatui`

**Views**:
- **Search View**: Semantic search input with results
- **Browse View**: Category/subject browsing, latest papers
- **Paper Detail View**: Abstract, metadata, similar papers
- **Library View**: Saved/starred papers, collections
- **Settings View**: Config, HelixDB connection, model settings

**Features**:
- Vim-like keybindings
- Real-time search suggestions
- Syntax highlighting for abstracts
- Progress indicators for async operations

### 2. Application Core

#### State Manager
- Global application state
- Active view tracking
- Paper cache
- Search history

#### Event Handler
- Keyboard input processing
- Async event dispatch
- Command palette

#### Config Manager
- TOML-based configuration
- HelixDB connection settings
- API keys/credentials
- UI preferences

### 3. Data Layer

#### arXiv Client
**Responsibilities**:
- Query arXiv API (Atom/RSS feeds)
- Parse paper metadata
- Download PDFs
- Rate limiting

**API Operations**:
```rust
- search(query: &str, max_results: u32) -> Vec<Paper>
- get_by_id(arxiv_id: &str) -> Paper
- download_pdf(arxiv_id: &str) -> Result<PathBuf>
- get_latest(category: Category) -> Vec<Paper>
```

#### HelixDB Client
**Responsibilities**:
- Store paper embeddings as vectors
- Store paper metadata as nodes
- Create relationship edges (citations, similar papers)
- Semantic search via vector similarity

**Client Usage**:
```rust
use helix_rs::{HelixDB, HelixDBClient};
use serde_json::json;

// Initialize client
let client = HelixDB::new(
    Some("http://localhost"),
    Some(6969),
    None  // API key
);

// Execute query
let payload = json!({
    "arxiv_id": "2301.00001",
    "title": "Example Paper",
    "embedding": vec![0.1, 0.2, ...]
});

let result: serde_json::Value = client
    .query("AddPaper", &payload)
    .await?;
```

**Schema Design** (defined in `helix/schema.hx`):
```hql
// Paper vector with metadata
V::Paper {
    arxiv_id: String,
    title: String,
    authors: String,
    published: String,
    categories: String,
    abstract_preview: String  // First 200 chars
}

// Full paper metadata node
N::PaperMetadata {
    arxiv_id: String,
    abstract: String,
    comment: String,
    journal_ref: String,
    doi: String,
    pdf_url: String,
    saved: Bool,
    read: Bool,
    tags: String
}

// User collections
N::Collection {
    name: String,
    description: String,
    created_at: String
}

// Relationships
E::HasMetadata {
    From: String,  // Paper vector ID
    To: String,    // PaperMetadata node ID
    Properties: {}
}

E::SimilarTo {
    From: String,
    To: String,
    Properties: {
        similarity_score: F32
    }
}

E::InCollection {
    From: String,  // Paper vector ID
    To: String,    // Collection node ID
    Properties: {
        added_at: String
    }
}

E::Cites {
    From: String,
    To: String,
    Properties: {
        context: String  // Optional citation context
    }
}
```

**Query Operations** (defined in `helix/queries.hx`):
```hql
// Semantic search for papers
QUERY SearchPapers (query_embedding: Vec<F32>, min_similarity: F32, limit: U32) =>
    results <- SEARCH V::Paper
    WHERE SIMILARITY(embedding, query_embedding) > min_similarity
    LIMIT limit
    RETURN results

// Add a new paper with embedding
QUERY AddPaper (
    arxiv_id: String,
    title: String,
    authors: String,
    published: String,
    categories: String,
    abstract_preview: String,
    embedding: Vec<F32>
) =>
    paper <- AddV<Paper>({
        arxiv_id: arxiv_id,
        title: title,
        authors: authors,
        published: published,
        categories: categories,
        abstract_preview: abstract_preview,
        embedding: embedding
    })
    RETURN paper

// Get paper with full metadata
QUERY GetPaperWithMetadata (arxiv_id: String) =>
    result <- MATCH (v:Paper)-[:HasMetadata]->(m:PaperMetadata)
    WHERE v.arxiv_id = arxiv_id
    RETURN v, m

// Find similar papers
QUERY FindSimilarPapers (arxiv_id: String, limit: U32) =>
    similar <- MATCH (p1:Paper)-[:SimilarTo]->(p2:Paper)
    WHERE p1.arxiv_id = arxiv_id
    ORDER BY SimilarTo.similarity_score DESC
    LIMIT limit
    RETURN p2

// Get papers in a collection
QUERY GetCollectionPapers (collection_name: String) =>
    papers <- MATCH (p:Paper)-[:InCollection]->(c:Collection)
    WHERE c.name = collection_name
    RETURN p

// Add paper to collection
QUERY AddPaperToCollection (paper_id: String, collection_id: String, added_at: String) =>
    edge <- AddE<InCollection>({
        from: paper_id,
        to: collection_id,
        properties: {
            added_at: added_at
        }
    })
    RETURN edge
```

**Rust Client Examples**:
```rust
// Search for papers semantically
let payload = json!({
    "query_embedding": embedding_vec,
    "min_similarity": 0.7,
    "limit": 20
});
let results: serde_json::Value = client.query("SearchPapers", &payload).await?;

// Get paper with metadata
let payload = json!({
    "arxiv_id": "2301.00001"
});
let paper: serde_json::Value = client.query("GetPaperWithMetadata", &payload).await?;

// Find similar papers
let payload = json!({
    "arxiv_id": "2301.00001",
    "limit": 10
});
let similar: serde_json::Value = client.query("FindSimilarPapers", &payload).await?;
```

#### Embedding Generator
**Model**: `all-MiniLM-L6-v2` (384 dimensions, fast inference)

**Responsibilities**:
- Generate embeddings for paper abstracts + titles
- Batch processing for efficiency
- ONNX runtime for fast CPU inference

**Operations**:
```rust
- generate_embedding(text: &str) -> Vec<f32>
- batch_generate(texts: Vec<&str>) -> Vec<Vec<f32>>
```

## Data Flow

### Paper Ingestion Flow
```
1. User searches arXiv API
2. arXiv Client fetches papers
3. For each paper:
   a. Generate embedding from title + abstract
   b. Create V::Paper vector in HelixDB
   c. Create N::PaperMetadata node
   d. Create HasMetadata edge
4. Calculate similarities (background job)
5. Create SimilarTo edges for top matches
```

### Semantic Search Flow
```
1. User enters search query
2. Generate query embedding
3. HelixDB vector similarity search
4. Fetch associated metadata via graph traversal
5. Rank and display results
6. Cache results in application state
```

### Paper Detail Flow
```
1. User selects paper
2. Fetch paper vector + metadata (single graph query)
3. Fetch similar papers via SimilarTo edges
4. Display in detail view
5. Optional: Download PDF on demand
```

## Technology Stack

### Core Dependencies
```toml
[dependencies]
# TUI
ratatui = "0.26"
crossterm = "0.27"

# Async runtime
tokio = { version = "1.36", features = ["full"] }

# HelixDB client
helix-rs = "0.1.9"

# HTTP client for arXiv API
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Embedding generation
ort = "2.0"  # ONNX Runtime
ndarray = "0.15"
tokenizers = "0.15"

# XML parsing (arXiv Atom feeds)
quick-xml = "0.31"

# Configuration
config = "0.14"
toml = "0.8"

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

# Utilities
chrono = "0.4"
dirs = "5.0"
```

## Project Structure

```
arxiv-tui/
├── Cargo.toml
├── src/
│   ├── main.rs                 # Entry point
│   ├── app.rs                  # Application state
│   ├── config.rs               # Configuration management
│   │
│   ├── ui/                     # TUI layer
│   │   ├── mod.rs
│   │   ├── app.rs              # Main UI coordinator
│   │   ├── views/
│   │   │   ├── search.rs
│   │   │   ├── browse.rs
│   │   │   ├── detail.rs
│   │   │   └── library.rs
│   │   └── components/         # Reusable UI components
│   │       ├── paper_list.rs
│   │       ├── search_bar.rs
│   │       └── status_bar.rs
│   │
│   ├── core/                   # Business logic
│   │   ├── mod.rs
│   │   ├── state.rs            # State manager
│   │   ├── events.rs           # Event handler
│   │   └── commands.rs         # Command palette
│   │
│   ├── arxiv/                  # arXiv client
│   │   ├── mod.rs
│   │   ├── client.rs           # API client
│   │   ├── models.rs           # Paper models
│   │   └── parser.rs           # XML parser
│   │
│   ├── db/                     # HelixDB integration
│   │   ├── mod.rs
│   │   ├── client.rs           # HelixDB client wrapper
│   │   └── models.rs           # Response models
│   │
│   ├── embeddings/             # Embedding generation
│   │   ├── mod.rs
│   │   ├── generator.rs        # ONNX inference
│   │   └── models.rs           # Model loading
│   │
│   └── utils/
│       ├── mod.rs
│       └── cache.rs            # Local caching
│
├── helix/                      # HelixDB schema and queries
│   ├── schema.hx               # Database schema definitions
│   └── queries.hx              # HQL query definitions
│
├── models/                     # ONNX models
│   └── all-MiniLM-L6-v2.onnx
│
├── config/
│   └── default.toml
│
└── tests/
    ├── integration/
    └── unit/
```

## Key Features (MVP)

### Phase 1: Core Functionality
- [x] Basic TUI with search view
- [x] arXiv API integration
- [x] HelixDB connection
- [x] Embedding generation
- [x] Semantic search

### Phase 2: Enhanced UX
- [ ] Paper collections/library
- [ ] Similar paper recommendations
- [ ] PDF download & local storage
- [ ] Citation graph visualization
- [ ] Search history

### Phase 3: Advanced Features
- [ ] Background indexing (latest papers)
- [ ] Full-text search (if PDFs downloaded)
- [ ] Export citations (BibTeX)
- [ ] Paper notes/annotations
- [ ] Multi-user support (via HelixDB)

## Performance Considerations

1. **Embedding Generation**: Batch process papers to amortize model loading cost
2. **Vector Search**: HelixDB handles this efficiently
3. **Caching**: Cache paper metadata locally to reduce API calls
4. **Async Operations**: All I/O operations are async (API calls, DB queries)
5. **Lazy Loading**: Only generate embeddings when needed (user searches/saves paper)

## Configuration Example

```toml
[arxiv]
rate_limit_delay_ms = 3000  # arXiv requires 3 second delay between requests
max_results = 100

[helixdb]
endpoint = "http://localhost"
port = 6969
api_key = ""  # Optional, for production deployments

[embeddings]
model_path = "./models/all-MiniLM-L6-v2.onnx"
batch_size = 32
device = "cpu"  # or "cuda" if available

[ui]
theme = "dark"
keybindings = "vim"  # or "emacs"
papers_per_page = 20

[storage]
cache_dir = "~/.cache/arxiv-tui"
download_dir = "~/Documents/papers"
```

## Security Considerations

- API keys stored in config file (permissions 600)
- HelixDB encryption at rest (if enabled)
- PDF downloads verified by hash
- No sensitive data in embeddings (only public arXiv content)

## Future Enhancements

- Web UI companion (read-only view)
- Mobile app integration
- Collaborative features (shared collections)
- Integration with reference managers (Zotero, Mendeley)
- AI-powered paper summaries
- Email alerts for new papers in areas of interest
