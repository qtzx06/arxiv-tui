// HelixDB Schema for arXiv TUI
// Defines the data model for papers, metadata, and relationships

// Paper vector - stores embeddings for semantic search
V::Paper {
    arxiv_id: String,
    title: String,
    authors: String,
    published: String,
    categories: String,
    abstract_preview: String
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
    tags: String,
    created_at: String
}

// User collections for organizing papers
N::Collection {
    name: String,
    description: String,
    created_at: String
}

// Links paper vector to its full metadata
E::HasMetadata {
    From: String,
    To: String,
    Properties: {}
}

// Represents similarity between papers (from vector search)
E::SimilarTo {
    From: String,
    To: String,
    Properties: {
        similarity_score: F32
    }
}

// Papers in a collection
E::InCollection {
    From: String,
    To: String,
    Properties: {
        added_at: String
    }
}

// Citation relationships between papers
E::Cites {
    From: String,
    To: String,
    Properties: {
        context: String
    }
}
