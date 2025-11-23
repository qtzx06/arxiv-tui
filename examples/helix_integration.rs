// Full integration example: arXiv + Embeddings + HelixDB
// This demonstrates the complete RAG pipeline

use anyhow::Result;
use arxiv_tui::arxiv::client::ArxivClient;
use arxiv_tui::db::client::DbClient;
use arxiv_tui::embeddings::generator::EmbeddingGenerator;
use arxiv_tui::config::{Config, HelixDbConfig, EmbeddingsConfig};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("=== arXiv + HelixDB RAG Pipeline ===\n");

    // 1. Initialize clients
    println!("1. Initializing clients...");

    let arxiv_client = ArxivClient::new(3000, 10);

    let helix_config = HelixDbConfig {
        endpoint: "http://localhost".to_string(),
        port: 6969,
        api_key: String::new(),
    };
    let db_client = DbClient::new(&helix_config)?;
    println!("   ✓ Connected to HelixDB at {}:{}", helix_config.endpoint, helix_config.port);

    let embeddings_config = EmbeddingsConfig {
        model_path: PathBuf::from("./models/all-MiniLM-L6-v2.onnx"),
        batch_size: 32,
        device: "cpu".to_string(),
        dimension: 384,
    };
    let embedding_gen = EmbeddingGenerator::new(&embeddings_config)?;
    println!("   ✓ Embedding generator ready (placeholder mode)");

    // 2. Fetch papers from arXiv
    println!("\n2. Fetching papers from arXiv...");
    let search_query = "transformer neural networks";
    let papers = arxiv_client.search(search_query, Some(5)).await?;
    println!("   ✓ Found {} papers", papers.len());

    // 3. Generate embeddings and store in HelixDB
    println!("\n3. Processing papers and storing in HelixDB...");
    for (i, paper) in papers.iter().enumerate() {
        println!("\n   Processing [{}/{}]: {}", i + 1, papers.len(), paper.title);

        // Generate embedding from title + abstract
        let text = paper.text_for_embedding();
        let embedding = embedding_gen.generate_embedding(&text)?;
        println!("     ✓ Generated {}-dimensional embedding", embedding.len());

        // Store paper vector in HelixDB
        match db_client.add_paper(paper, embedding).await {
            Ok(result) => {
                println!("     ✓ Stored paper vector in HelixDB");
                println!("       Response: {}", serde_json::to_string_pretty(&result)?);
            }
            Err(e) => {
                println!("     ✗ Failed to store paper: {}", e);
                println!("       (This is expected if HelixDB is not running or schema not loaded)");
            }
        }

        // Store paper metadata
        match db_client.add_paper_metadata(paper).await {
            Ok(result) => {
                println!("     ✓ Stored paper metadata");
                println!("       Response: {}", serde_json::to_string_pretty(&result)?);
            }
            Err(e) => {
                println!("     ✗ Failed to store metadata: {}", e);
            }
        }
    }

    // 4. Perform semantic search
    println!("\n4. Performing semantic search...");
    let search_text = "attention mechanism in deep learning";
    println!("   Query: \"{}\"", search_text);

    let query_embedding = embedding_gen.generate_embedding(search_text)?;
    println!("   ✓ Generated query embedding");

    match db_client.search_papers(query_embedding, 0.5, 10).await {
        Ok(results) => {
            println!("\n   ✓ Semantic search results:");
            println!("{}", serde_json::to_string_pretty(&results)?);
        }
        Err(e) => {
            println!("\n   ✗ Search failed: {}", e);
            println!("   Note: Make sure HelixDB is running with schema loaded");
        }
    }

    println!("\n=== Pipeline Complete ===");
    println!("\nTo run this successfully:");
    println!("1. Start HelixDB: helix serve");
    println!("2. Load schema: helix load helix/schema.hx");
    println!("3. Load queries: helix load helix/queries.hx");
    println!("4. Run this example: cargo run --example helix_integration");

    Ok(())
}
