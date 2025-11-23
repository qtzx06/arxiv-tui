use anyhow::Result;
use chrono::DateTime;
use quick_xml::de::from_str;
use serde::Deserialize;

use super::models::Paper;

#[derive(Debug, Deserialize)]
struct Feed {
    entry: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
struct Entry {
    id: String,
    title: String,
    summary: String,
    published: String,
    updated: String,
    author: Vec<Author>,
    category: Vec<CategoryAttr>,
    link: Vec<Link>,
    #[serde(rename = "arxiv:comment")]
    comment: Option<String>,
    #[serde(rename = "arxiv:journal_ref")]
    journal_ref: Option<String>,
    #[serde(rename = "arxiv:doi")]
    doi: Option<String>,
    #[serde(rename = "arxiv:primary_category")]
    primary_category: PrimaryCategory,
}

#[derive(Debug, Deserialize)]
struct Author {
    name: String,
}

#[derive(Debug, Deserialize)]
struct CategoryAttr {
    #[serde(rename = "@term")]
    term: String,
}

#[derive(Debug, Deserialize)]
struct Link {
    #[serde(rename = "@href")]
    href: String,
    #[serde(rename = "@type")]
    link_type: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PrimaryCategory {
    #[serde(rename = "@term")]
    term: String,
}

pub fn parse_arxiv_response(xml: &str) -> Result<Vec<Paper>> {
    let feed: Feed = from_str(xml)?;

    let papers = feed
        .entry
        .into_iter()
        .map(|entry| -> Result<Paper> {
            let arxiv_id = extract_arxiv_id(&entry.id);
            let authors = entry.author.into_iter().map(|a| a.name).collect();
            let categories = entry.category.into_iter().map(|c| c.term).collect();
            let pdf_url = entry
                .link
                .iter()
                .find(|l| l.link_type.as_deref() == Some("application/pdf"))
                .map(|l| l.href.clone())
                .unwrap_or_else(|| format!("https://arxiv.org/pdf/{}.pdf", arxiv_id));

            Ok(Paper {
                arxiv_id,
                title: entry.title.trim().to_string(),
                authors,
                abstract_text: entry.summary.trim().to_string(),
                published: DateTime::parse_from_rfc3339(&entry.published)?.with_timezone(&chrono::Utc),
                updated: DateTime::parse_from_rfc3339(&entry.updated)?.with_timezone(&chrono::Utc),
                categories,
                pdf_url,
                comment: entry.comment,
                journal_ref: entry.journal_ref,
                doi: entry.doi,
                primary_category: entry.primary_category.term,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(papers)
}

fn extract_arxiv_id(url: &str) -> String {
    url.split('/')
        .last()
        .unwrap_or(url)
        .trim_start_matches("abs/")
        .to_string()
}
