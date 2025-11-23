// Quick check if HelixDB is running and accessible

use arxiv_tui::config::HelixDbConfig;
use arxiv_tui::db::client::DbClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("=== HelixDB Connection Check ===\n");

    let config = HelixDbConfig {
        endpoint: "http://localhost".to_string(),
        port: 6969,
        api_key: String::new(),
    };

    println!("Attempting to connect to: {}:{}", config.endpoint, config.port);

    match DbClient::new(&config) {
        Ok(_client) => {
            println!("✓ Client initialized successfully");
            println!("\n✓ HelixDB is accessible!");
            println!("\nNext steps:");
            println!("1. Load schema: cd helix && helix load schema.hx");
            println!("2. Load queries: helix load queries.hx");
            println!("3. Run integration: cargo run --example helix_integration");
            Ok(())
        }
        Err(e) => {
            println!("✗ Connection failed: {}", e);
            println!("\nTroubleshooting:");
            println!("1. Is HelixDB running? Start with: helix serve");
            println!("2. Check port: Default is 6969");
            println!("3. Check endpoint: Default is http://localhost");
            Err(e)
        }
    }
}
