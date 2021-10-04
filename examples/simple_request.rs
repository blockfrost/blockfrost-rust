use blockfrost::{env, BlockFrostApi, Settings};

fn build_api() -> blockfrost::Result<BlockFrostApi> {
    let project_id = env::load_project_id()?.expect("BLOCKFROST_PROJECT_ID not found.");
    let settings = Settings::new();
    let api = BlockFrostApi::new(project_id, settings);

    Ok(api)
}

#[tokio::main]
async fn main() -> blockfrost::Result<()> {
    let api = build_api()?;

    let health = api.health().await?;
    println!("{:#?}", health);

    Ok(())
}
