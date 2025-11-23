// Semantic search example using HelixDB

use anyhow::Result;
use arxiv_tui::db::client::DbClient;
use arxiv_tui::embeddings::generator::EmbeddingGenerator;
use arxiv_tui::config::{HelixDbConfig, EmbeddingsConfig};
use std::path::PathBuf;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    println!("=== Semantic Paper Search ===\n");

    // Initialize
    let db_client = DbClient::new(&HelixDbConfig {
        endpoint: "http://localhost".to_string(),
        port: 6969,
        api_key: String::new(),
    })?;

    let embedding_gen = EmbeddingGenerator::new(&EmbeddingsConfig {
        model_path: PathBuf::from("./models/all-MiniLM-L6-v2.onnx"),
        batch_size: 32,
        device: "cpu".to_string(),
        dimension: 384,
    })?;

    println!("Connected to HelixDB");
    println!("Ready for semantic search!\n");

    // Example queries
    let example_queries = vec![
        "attention mechanisms in transformers",
        "computer vision for autonomous driving",
        "natural language understanding with large models",
        "reinforcement learning for robotics",
    ];

    println!("Example queries:");
    for (i, query) in example_queries.iter().enumerate() {
        println!("  {}. {}", i + 1, query);
    }

    println!("\nEnter your search query (or use 1-4 for examples, 'q' to quit):");

    loop {
        print!("\n> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "q" || input == "quit" {
            println!("Goodbye!");
            break;
        }

        // Handle example selection
        let query = if let Ok(idx) = input.parse::<usize>() {
            if idx > 0 && idx <= example_queries.len() {
                example_queries[idx - 1]
            } else {
                println!("Invalid example number");
                continue;
            }
        } else {
            input
        };

        println!("\nSearching for: \"{}\"", query);

        // Generate query embedding
        let embedding = embedding_gen.generate_embedding(query)?;
        println!("Generated embedding ({} dimensions)", embedding.len());

        // Search HelixDB
        match db_client.search_papers(embedding, 0.5, 10).await {
            Ok(results) => {
                println!("\n📄 Search Results:");
                println!("{}\n", serde_json::to_string_pretty(&results)?);
            }
            Err(e) => {
                println!("\n❌ Search failed: {}", e);
                println!("Make sure papers have been ingested first!");
                println!("Run: cargo run --example ingest_papers");
            }
        }
    }

    Ok(())
}
