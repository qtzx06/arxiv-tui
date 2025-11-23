use arxiv_tui::arxiv::client::ArxivClient;
use arxiv_tui::arxiv::models::Category;

#[tokio::test]
async fn test_search_papers() {
    // Create client with reasonable rate limit
    let client = ArxivClient::new(3000, 10);

    // Search for machine learning papers
    let results = client
        .search("machine learning", Some(5))
        .await
        .expect("Search should succeed");

    println!("\nSearch Results for 'machine learning':");
    println!("Found {} papers\n", results.len());

    for (i, paper) in results.iter().enumerate() {
        println!("{}. {}", i + 1, paper.title);
        println!("   arXiv ID: {}", paper.arxiv_id);
        println!("   Authors: {}", paper.authors_string());
        println!("   Categories: {}", paper.categories_string());
        println!("   Published: {}", paper.published);
        println!("   Abstract preview: {}", paper.abstract_preview(100));
        println!();
    }

    assert!(!results.is_empty(), "Should find some papers");
    assert!(results.len() <= 5, "Should respect max_results limit");
}

#[tokio::test]
async fn test_get_paper_by_id() {
    let client = ArxivClient::new(3000, 10);

    // Get a specific paper (Attention is All You Need)
    let paper = client
        .get_by_id("1706.03762")
        .await
        .expect("Should fetch paper by ID");

    println!("\nFetched Paper by ID:");
    println!("Title: {}", paper.title);
    println!("arXiv ID: {}", paper.arxiv_id);
    println!("Authors: {}", paper.authors_string());
    println!("Abstract: {}", paper.abstract_text);
    println!();

    assert_eq!(paper.arxiv_id, "1706.03762");
    assert!(paper.title.contains("Attention"));
}

#[tokio::test]
async fn test_get_latest_papers() {
    let client = ArxivClient::new(3000, 10);

    // Get latest AI papers
    let results = client
        .get_latest(Category::ArtificialIntelligence, Some(5))
        .await
        .expect("Should fetch latest papers");

    println!("\nLatest AI Papers:");
    println!("Found {} papers\n", results.len());

    for (i, paper) in results.iter().enumerate() {
        println!("{}. {}", i + 1, paper.title);
        println!("   Published: {}", paper.published);
        println!("   Categories: {}", paper.categories_string());
        println!();
    }

    assert!(!results.is_empty(), "Should find some papers");
    assert!(results.len() <= 5, "Should respect max_results limit");
}

#[tokio::test]
async fn test_category_search() {
    let client = ArxivClient::new(3000, 10);

    // Search in specific category
    let results = client
        .search("cat:cs.AI", Some(3))
        .await
        .expect("Category search should succeed");

    println!("\nCategory Search (cs.AI):");
    println!("Found {} papers\n", results.len());

    for paper in &results {
        println!("- {}", paper.title);
        println!("  Categories: {}", paper.categories_string());
    }

    assert!(!results.is_empty(), "Should find AI papers");
}

#[tokio::test]
async fn test_text_for_embedding() {
    let client = ArxivClient::new(3000, 10);

    let paper = client
        .get_by_id("1706.03762")
        .await
        .expect("Should fetch paper");

    let embedding_text = paper.text_for_embedding();

    println!("\nEmbedding Text Preview:");
    println!("{}", &embedding_text[..200.min(embedding_text.len())]);

    assert!(embedding_text.contains(&paper.title));
    assert!(embedding_text.contains(&paper.abstract_text));
}
