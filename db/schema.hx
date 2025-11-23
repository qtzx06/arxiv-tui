// Minimal HelixDB Schema for arXiv TUI

// Paper node with embedding for semantic search
N::Paper {
    INDEX arxiv_id: String,
    title: String,
    authors: String,
    published: String,
    categories: String,
    abstract: String,
    embedding: [F32]
}

// User collections
N::Collection {
    INDEX name: String,
    description: String,
    created_at: String
}

// Papers in a collection
E::InCollection {
    From: Paper,
    To: Collection,
    Properties: {
        added_at: String
    }
}
