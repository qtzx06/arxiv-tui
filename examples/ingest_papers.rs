// Ingest papers from arXiv into HelixDB
// This is a practical tool for building your database

use anyhow::Result;
use arxiv_tui::arxiv::client::ArxivClient;
use arxiv_tui::arxiv::models::Category;
use arxiv_tui::db::client::DbClient;
use arxiv_tui::embeddings::generator::EmbeddingGenerator;
use arxiv_tui::config::{HelixDbConfig, EmbeddingsConfig};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    println!("=== arXiv Paper Ingestion Tool ===\n");

    // Initialize clients
    let arxiv_client = ArxivClient::new(3000, 50);

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

    // Categories to ingest
    let categories = vec![
        ("AI", Category::ArtificialIntelligence),
        ("ML", Category::MachineLearning),
        ("NLP", Category::ComputationAndLanguage),
        ("Vision", Category::ComputerVision),
    ];

    println!("Ingesting latest papers from {} categories\n", categories.len());

    let mut total_ingested = 0;
    let papers_per_category = 10;

    for (name, category) in categories {
        println!("\n📚 Category: {} ({})", name, category.as_str());
        println!("Fetching {} latest papers...", papers_per_category);

        let papers = arxiv_client
            .get_latest(category, Some(papers_per_category))
            .await?;

        println!("Found {} papers", papers.len());

        for (i, paper) in papers.iter().enumerate() {
            print!("  [{}/{}] Processing: {}... ", i + 1, papers.len(),
                   paper.title.chars().take(50).collect::<String>());

            // Generate embedding
            let text = paper.text_for_embedding();
            let embedding = embedding_gen.generate_embedding(&text)?;

            // Store in HelixDB
            match db_client.add_paper(paper, embedding).await {
                Ok(_) => {
                    // Also store metadata
                    let _ = db_client.add_paper_metadata(paper).await;
                    println!("✓");
                    total_ingested += 1;
                }
                Err(e) => {
                    println!("✗ ({})", e);
                }
            }
        }

        println!("  Completed {} papers", papers.len());
    }

    println!("\n=== Ingestion Complete ===");
    println!("Total papers ingested: {}", total_ingested);
    println!("\nYou can now perform semantic searches!");

    Ok(())
}
