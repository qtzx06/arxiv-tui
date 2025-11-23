// HelixDB Queries for arXiv TUI
// All HQL queries used by the application

// ===== Paper Management =====

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

// Add full metadata for a paper
QUERY AddPaperMetadata (
    arxiv_id: String,
    abstract: String,
    comment: String,
    journal_ref: String,
    doi: String,
    pdf_url: String,
    created_at: String
) =>
    metadata <- AddN<PaperMetadata>({
        arxiv_id: arxiv_id,
        abstract: abstract,
        comment: comment,
        journal_ref: journal_ref,
        doi: doi,
        pdf_url: pdf_url,
        saved: false,
        read: false,
        tags: "",
        created_at: created_at
    })
    RETURN metadata

// Link paper vector to its metadata
QUERY LinkPaperMetadata (paper_id: String, metadata_id: String) =>
    edge <- AddE<HasMetadata>({
        from: paper_id,
        to: metadata_id,
        properties: {}
    })
    RETURN edge

// ===== Search Operations =====

// Semantic search for papers by embedding similarity
QUERY SearchPapers (query_embedding: Vec<F32>, min_similarity: F32, limit: U32) =>
    results <- SEARCH V::Paper
    WHERE SIMILARITY(embedding, query_embedding) > min_similarity
    LIMIT limit
    RETURN results

// Get a single paper by arXiv ID
QUERY GetPaperByArxivId (arxiv_id: String) =>
    paper <- MATCH (v:Paper)
    WHERE v.arxiv_id = arxiv_id
    RETURN v

// Get paper with its full metadata
QUERY GetPaperWithMetadata (arxiv_id: String) =>
    result <- MATCH (v:Paper)-[:HasMetadata]->(m:PaperMetadata)
    WHERE v.arxiv_id = arxiv_id
    RETURN v, m

// ===== Similar Papers =====

// Find similar papers (via pre-computed similarity edges)
QUERY FindSimilarPapers (arxiv_id: String, limit: U32) =>
    similar <- MATCH (p1:Paper)-[:SimilarTo]->(p2:Paper)
    WHERE p1.arxiv_id = arxiv_id
    ORDER BY SimilarTo.similarity_score DESC
    LIMIT limit
    RETURN p2

// Create similarity edge between two papers
QUERY AddSimilarity (from_id: String, to_id: String, score: F32) =>
    edge <- AddE<SimilarTo>({
        from: from_id,
        to: to_id,
        properties: {
            similarity_score: score
        }
    })
    RETURN edge

// ===== Collections =====

// Create a new collection
QUERY CreateCollection (name: String, description: String, created_at: String) =>
    collection <- AddN<Collection>({
        name: name,
        description: description,
        created_at: created_at
    })
    RETURN collection

// Get all collections
QUERY GetAllCollections () =>
    collections <- MATCH (c:Collection)
    RETURN c

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

// Remove paper from collection
QUERY RemovePaperFromCollection (paper_id: String, collection_id: String) =>
    result <- MATCH (p:Paper)-[e:InCollection]->(c:Collection)
    WHERE p.id = paper_id AND c.id = collection_id
    DELETE e
    RETURN result

// ===== Paper Status =====

// Mark paper as saved
QUERY MarkPaperSaved (arxiv_id: String, saved: Bool) =>
    result <- MATCH (m:PaperMetadata)
    WHERE m.arxiv_id = arxiv_id
    SET m.saved = saved
    RETURN m

// Mark paper as read
QUERY MarkPaperRead (arxiv_id: String, read: Bool) =>
    result <- MATCH (m:PaperMetadata)
    WHERE m.arxiv_id = arxiv_id
    SET m.read = read
    RETURN m

// Update paper tags
QUERY UpdatePaperTags (arxiv_id: String, tags: String) =>
    result <- MATCH (m:PaperMetadata)
    WHERE m.arxiv_id = arxiv_id
    SET m.tags = tags
    RETURN m

// ===== Citations =====

// Add citation relationship
QUERY AddCitation (from_arxiv_id: String, to_arxiv_id: String, context: String) =>
    edge <- MATCH (p1:Paper), (p2:Paper)
    WHERE p1.arxiv_id = from_arxiv_id AND p2.arxiv_id = to_arxiv_id
    CREATE (p1)-[:Cites {context: context}]->(p2)
    RETURN edge

// Get papers cited by a paper
QUERY GetCitedPapers (arxiv_id: String) =>
    cited <- MATCH (p1:Paper)-[:Cites]->(p2:Paper)
    WHERE p1.arxiv_id = arxiv_id
    RETURN p2

// Get papers that cite a paper
QUERY GetCitingPapers (arxiv_id: String) =>
    citing <- MATCH (p1:Paper)-[:Cites]->(p2:Paper)
    WHERE p2.arxiv_id = arxiv_id
    RETURN p1

// ===== Statistics =====

// Count total papers
QUERY CountPapers () =>
    count <- MATCH (p:Paper)
    RETURN COUNT(p)

// Count saved papers
QUERY CountSavedPapers () =>
    count <- MATCH (m:PaperMetadata)
    WHERE m.saved = true
    RETURN COUNT(m)
