use arxiv_tui::arxiv::client::ArxivClient;
use arxiv_tui::arxiv::models::Category;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("=== arXiv Client Demo ===\n");

    // Create client (3 second delay between requests, max 10 results)
    let client = ArxivClient::new(3000, 10);

    // Example 1: Search for papers
    println!("1. Searching for 'large language models'...\n");
    let search_results = client.search("large language models", Some(5)).await?;

    for (i, paper) in search_results.iter().enumerate() {
        println!("{}. {}", i + 1, paper.title);
        println!("   ID: {}", paper.arxiv_id);
        println!("   Authors: {}", paper.authors_string());
        println!("   Published: {}", paper.published.format("%Y-%m-%d"));
        println!("   Categories: {}", paper.categories_string());
        println!("   Abstract: {}...\n", paper.abstract_preview(150));
    }

    // Example 2: Get a specific paper by ID
    println!("\n2. Fetching specific paper (Attention is All You Need - 1706.03762)...\n");
    let paper = client.get_by_id("1706.03762").await?;

    println!("Title: {}", paper.title);
    println!("Authors: {}", paper.authors_string());
    println!("Published: {}", paper.published.format("%Y-%m-%d"));
    println!("PDF URL: {}", paper.pdf_url);
    println!("\nAbstract:\n{}\n", paper.abstract_text);

    // Example 3: Get latest papers from a category
    println!("\n3. Getting latest papers from cs.AI...\n");
    let latest = client
        .get_latest(Category::ArtificialIntelligence, Some(5))
        .await?;

    for (i, paper) in latest.iter().enumerate() {
        println!("{}. {}", i + 1, paper.title);
        println!("   Published: {}", paper.published.format("%Y-%m-%d %H:%M"));
        println!("   ID: {}\n", paper.arxiv_id);
    }

    // Example 4: Advanced search with arXiv query syntax
    println!("\n4. Advanced search: Recent computer vision papers...\n");
    let advanced_results = client
        .search("cat:cs.CV AND submittedDate:[202401* TO *]", Some(5))
        .await?;

    for (i, paper) in advanced_results.iter().enumerate() {
        println!("{}. {}", i + 1, paper.title);
        println!("   Submitted: {}", paper.published.format("%Y-%m-%d"));
        println!();
    }

    println!("\n=== Demo Complete ===");
    println!("Note: arXiv API requires 3 second delay between requests");

    Ok(())
}
