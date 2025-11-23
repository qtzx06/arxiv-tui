// src/arxiv.rs

use serde::{Deserialize, Serialize};
use quick_xml::de::from_str;
use anyhow::{Result, Context}; // Add anyhow::Context for better error messages
use reqwest::Client; // Import reqwest client

// Represents the top-level Atom feed structure
#[derive(Debug, Deserialize, Serialize)]
pub struct Feed {
    pub updated: String,
    pub id: String,
    pub title: String,
    pub link: Vec<Link>,
    #[serde(default)] // Ensure entries are optional in feed
    pub entry: Vec<Entry>,
}

// Represents an individual arXiv article entry
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Entry {
    pub id: String,
    pub updated: String,
    pub published: String,
    pub title: String,
    pub summary: String,
    #[serde(default)] // Make author optional, in case it's missing
    pub author: Vec<Author>,
    pub link: Vec<Link>,
    #[serde(rename = "category", default)] // Rename 'category' to 'tag' for clarity
    pub tags: Vec<Tag>,
    // Add other fields as needed, e.g., 'comment', 'journal_ref', 'doi'
}

// Represents an author
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Author {
    pub name: String,
    // Optional: add affiliation if available in the XML
    // #[serde(rename = "arxiv:affiliation")]
    // pub affiliation: Option<String>,
}

// Represents a link
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Link {
    pub href: String,
    #[serde(rename = "type", default)] // 'type' is a reserved keyword in Rust
    pub mime_type: Option<String>,
    pub rel: Option<String>,
    pub title: Option<String>,
}

// Represents an arXiv category/tag
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tag {
    pub term: String,
    pub scheme: Option<String>,
}

// Base URL for the arXiv API
const ARXIV_API_BASE_URL: &str = "http://export.arxiv.org/api/query";

/// Searches the arXiv API for papers matching the given query.
pub async fn search(query: &str) -> Result<Feed> {
    let client = Client::new();
    let search_url = format!(
        "{}?search_query=all:{}&start=0&max_results=10",
        ARXIV_API_BASE_URL,
        urlencoding::encode(query)
    );

    println!("Searching arXiv: {}", search_url);

    let response = client.get(&search_url)
        .send()
        .await
        .context("Failed to send request to arXiv API")?;

    let response_text = response.text()
        .await
        .context("Failed to get response text from arXiv API")?;

    println!("Received response from arXiv (first 500 chars): {}", &response_text[..std::cmp::min(response_text.len(), 500)]);

    let feed: Feed = from_str(&response_text)
        .context("Failed to parse XML response from arXiv API")?;

    Ok(feed)
}
