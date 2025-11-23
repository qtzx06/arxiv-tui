use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub arxiv: ArxivConfig,
    pub helixdb: HelixDbConfig,
    pub embeddings: EmbeddingsConfig,
    pub ui: UiConfig,
    pub storage: StorageConfig,
    pub search: SearchConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArxivConfig {
    pub rate_limit_delay_ms: u64,
    pub max_results: u32,
    pub default_categories: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelixDbConfig {
    pub endpoint: String,
    pub port: u16,
    pub api_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingsConfig {
    pub model_path: PathBuf,
    pub batch_size: usize,
    pub device: String,
    pub dimension: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub theme: String,
    pub keybindings: String,
    pub papers_per_page: usize,
    pub show_line_numbers: bool,
    pub tick_rate_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub cache_dir: PathBuf,
    pub download_dir: PathBuf,
    pub max_cache_size_mb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    pub min_similarity: f32,
    pub max_results: usize,
    pub enable_fuzzy: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub log_to_file: bool,
    pub log_file: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config = config::Config::builder()
            .add_source(config::File::with_name("config/default"))
            .add_source(config::Environment::with_prefix("ARXIV_TUI"))
            .build()?;

        let mut cfg: Config = config.try_deserialize()?;

        // Expand home directory paths
        cfg.storage.cache_dir = expand_tilde(&cfg.storage.cache_dir);
        cfg.storage.download_dir = expand_tilde(&cfg.storage.download_dir);
        cfg.logging.log_file = expand_tilde(&cfg.logging.log_file);

        // Create directories if they don't exist
        std::fs::create_dir_all(&cfg.storage.cache_dir)?;
        std::fs::create_dir_all(&cfg.storage.download_dir)?;

        Ok(cfg)
    }
}

fn expand_tilde(path: &PathBuf) -> PathBuf {
    if let Some(path_str) = path.to_str() {
        if path_str.starts_with("~/") {
            if let Some(home) = dirs::home_dir() {
                return home.join(&path_str[2..]);
            }
        }
    }
    path.clone()
}
