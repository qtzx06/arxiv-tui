use anyhow::Result;
use helix_rs::{HelixDB, HelixDBClient};
use serde_json::{json, Value};

use crate::config::HelixDbConfig;
use crate::arxiv::models::Paper;

pub struct DbClient {
    client: HelixDB,
}

impl DbClient {
    pub fn new(config: &HelixDbConfig) -> Result<Self> {
        let api_key = if config.api_key.is_empty() {
            None
        } else {
            Some(config.api_key.as_str())
        };

        let client = HelixDB::new(
            Some(&config.endpoint),
            Some(config.port),
            api_key,
        );

        Ok(Self { client })
    }

    pub async fn add_paper(&self, paper: &Paper, embedding: Vec<f32>) -> Result<Value> {
        let payload = json!({
            "arxiv_id": paper.arxiv_id,
            "title": paper.title,
            "authors": paper.authors_string(),
            "published": paper.published.to_rfc3339(),
            "categories": paper.categories_string(),
            "abstract_preview": paper.abstract_preview(200),
            "embedding": embedding,
        });

        let result = self.client.query("AddPaper", &payload).await?;
        Ok(result)
    }

    pub async fn add_paper_metadata(&self, paper: &Paper) -> Result<Value> {
        let payload = json!({
            "arxiv_id": paper.arxiv_id,
            "abstract": paper.abstract_text,
            "comment": paper.comment.as_deref().unwrap_or(""),
            "journal_ref": paper.journal_ref.as_deref().unwrap_or(""),
            "doi": paper.doi.as_deref().unwrap_or(""),
            "pdf_url": paper.pdf_url,
            "created_at": chrono::Utc::now().to_rfc3339(),
        });

        let result = self.client.query("AddPaperMetadata", &payload).await?;
        Ok(result)
    }

    pub async fn search_papers(
        &self,
        query_embedding: Vec<f32>,
        min_similarity: f32,
        limit: u32,
    ) -> Result<Value> {
        let payload = json!({
            "query_embedding": query_embedding,
            "min_similarity": min_similarity,
            "limit": limit,
        });

        let result = self.client.query("SearchPapers", &payload).await?;
        Ok(result)
    }

    pub async fn get_paper_by_arxiv_id(&self, arxiv_id: &str) -> Result<Value> {
        let payload = json!({
            "arxiv_id": arxiv_id,
        });

        let result = self.client.query("GetPaperByArxivId", &payload).await?;
        Ok(result)
    }

    pub async fn find_similar_papers(&self, arxiv_id: &str, limit: u32) -> Result<Value> {
        let payload = json!({
            "arxiv_id": arxiv_id,
            "limit": limit,
        });

        let result = self.client.query("FindSimilarPapers", &payload).await?;
        Ok(result)
    }

    pub async fn create_collection(&self, name: &str, description: &str) -> Result<Value> {
        let payload = json!({
            "name": name,
            "description": description,
            "created_at": chrono::Utc::now().to_rfc3339(),
        });

        let result = self.client.query("CreateCollection", &payload).await?;
        Ok(result)
    }

    pub async fn mark_paper_saved(&self, arxiv_id: &str, saved: bool) -> Result<Value> {
        let payload = json!({
            "arxiv_id": arxiv_id,
            "saved": saved,
        });

        let result = self.client.query("MarkPaperSaved", &payload).await?;
        Ok(result)
    }
}
