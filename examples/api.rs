use blockfrost::{BlockFrostApi, Settings, env::*};

type MainResult = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> MainResult {
    let project_id = project_id_from_env().expect("Could not read .env");
    let settings = Settings::new(project_id).set_test_network(true);
    let api = BlockFrostApi::new(settings);

    let health = api.health().await;
    println!("{:?}", health);

    Ok(())
}
