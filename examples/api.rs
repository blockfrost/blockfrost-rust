use blockfrost::{BlockFrostApi, Settings};

type MainResult = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> MainResult {
    let project_id = std::fs::read_to_string(".env").unwrap();
    let settings = Settings::new().set_test_network(true);
    let api = BlockFrostApi::new(project_id.trim(), settings);

    let health = api.health().await;
    println!("{:?}", health);

    Ok(())
}
