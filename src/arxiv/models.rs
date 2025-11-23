use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paper {
    pub arxiv_id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub abstract_text: String,
    pub published: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub categories: Vec<String>,
    pub pdf_url: String,
    pub comment: Option<String>,
    pub journal_ref: Option<String>,
    pub doi: Option<String>,
    pub primary_category: String,
}

impl Paper {
    pub fn authors_string(&self) -> String {
        self.authors.join(", ")
    }

    pub fn categories_string(&self) -> String {
        self.categories.join(", ")
    }

    pub fn abstract_preview(&self, max_len: usize) -> String {
        if self.abstract_text.len() <= max_len {
            self.abstract_text.clone()
        } else {
            format!("{}...", &self.abstract_text[..max_len])
        }
    }

    pub fn text_for_embedding(&self) -> String {
        format!("{} {}", self.title, self.abstract_text)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Category {
    ArtificialIntelligence,
    MachineLearning,
    ComputationAndLanguage,
    ComputerVision,
    Other,
}

impl Category {
    pub fn as_str(&self) -> &str {
        match self {
            Category::ArtificialIntelligence => "cs.AI",
            Category::MachineLearning => "cs.LG",
            Category::ComputationAndLanguage => "cs.CL",
            Category::ComputerVision => "cs.CV",
            Category::Other => "",
        }
    }
}
