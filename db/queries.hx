// Minimal HelixDB Queries for arXiv TUI

// Add a paper with embedding
QUERY AddPaper (
    arxiv_id: String,
    title: String,
    authors: String,
    published: String,
    categories: String,
    abstract: String,
    embedding: [F32]
) =>
    paper <- AddN<Paper>({
        arxiv_id: arxiv_id,
        title: title,
        authors: authors,
        published: published,
        categories: categories,
        abstract: abstract,
        embedding: embedding
    })
    RETURN paper

// Get a paper by arXiv ID
QUERY GetPaperByArxivId (arxiv_id: String) =>
    paper <- N<Paper>({arxiv_id: arxiv_id})
    RETURN paper

// Create a collection
QUERY CreateCollection (name: String, description: String, created_at: String) =>
    collection <- AddN<Collection>({
        name: name,
        description: description,
        created_at: created_at
    })
    RETURN collection
