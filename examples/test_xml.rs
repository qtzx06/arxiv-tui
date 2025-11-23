// Test to fetch raw XML from arXiv and see the structure

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let url = "http://export.arxiv.org/api/query?id_list=1706.03762";

    println!("Fetching from arXiv API...\n");
    let response = client.get(url).send().await?;
    let body = response.text().await?;

    println!("Raw XML Response:");
    println!("{}", body);

    Ok(())
}
