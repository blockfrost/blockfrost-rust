use blockfrost::{BlockFrostApi, Settings};

type MainResult = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> MainResult {
    let project_id = "p76Qpek6YNZsYda0DAXQ0LQfNbX7cxVz";
    let api = BlockFrostApi::new(project_id, Settings::default());
    Ok(())
}
