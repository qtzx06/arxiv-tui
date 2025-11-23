use anyhow::Result;
use reqwest::Client;
use std::time::Duration;

use super::models::{Category, Paper};
use super::parser::parse_arxiv_response;

pub struct ArxivClient {
    client: Client,
    rate_limit_delay: Duration,
    max_results: u32,
}

impl ArxivClient {
    pub fn new(rate_limit_delay_ms: u64, max_results: u32) -> Self {
        Self {
            client: Client::new(),
            rate_limit_delay: Duration::from_millis(rate_limit_delay_ms),
            max_results,
        }
    }

    pub async fn search(&self, query: &str, max_results: Option<u32>) -> Result<Vec<Paper>> {
        let max_results = max_results.unwrap_or(self.max_results);

        let url = format!(
            "http://export.arxiv.org/api/query?search_query={}&max_results={}",
            urlencoding::encode(query),
            max_results
        );

        tracing::info!("Searching arXiv: {}", query);
        let response = self.client.get(&url).send().await?;
        let body = response.text().await?;

        let papers = parse_arxiv_response(&body)?;

        // Respect rate limiting
        tokio::time::sleep(self.rate_limit_delay).await;

        Ok(papers)
    }

    pub async fn get_by_id(&self, arxiv_id: &str) -> Result<Paper> {
        let url = format!(
            "http://export.arxiv.org/api/query?id_list={}",
            arxiv_id
        );

        tracing::info!("Fetching paper: {}", arxiv_id);
        let response = self.client.get(&url).send().await?;
        let body = response.text().await?;

        let papers = parse_arxiv_response(&body)?;

        // Respect rate limiting
        tokio::time::sleep(self.rate_limit_delay).await;

        papers
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("Paper not found: {}", arxiv_id))
    }

    pub async fn get_latest(&self, category: Category, max_results: Option<u32>) -> Result<Vec<Paper>> {
        let max_results = max_results.unwrap_or(self.max_results);
        let category_str = category.as_str();

        let url = format!(
            "http://export.arxiv.org/api/query?search_query=cat:{}&sortBy=submittedDate&sortOrder=descending&max_results={}",
            category_str,
            max_results
        );

        tracing::info!("Fetching latest papers for category: {}", category_str);
        let response = self.client.get(&url).send().await?;
        let body = response.text().await?;

        let papers = parse_arxiv_response(&body)?;

        // Respect rate limiting
        tokio::time::sleep(self.rate_limit_delay).await;

        Ok(papers)
    }

    pub async fn download_pdf(&self, arxiv_id: &str, output_path: &std::path::Path) -> Result<()> {
        let pdf_url = format!("https://arxiv.org/pdf/{}.pdf", arxiv_id);

        tracing::info!("Downloading PDF: {}", arxiv_id);
        let response = self.client.get(&pdf_url).send().await?;
        let bytes = response.bytes().await?;

        std::fs::write(output_path, bytes)?;

        // Respect rate limiting
        tokio::time::sleep(self.rate_limit_delay).await;

        Ok(())
    }
}
