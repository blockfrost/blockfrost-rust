use blockfrost::{load, BlockFrostApi};

fn build_api() -> blockfrost::Result<BlockFrostApi> {
    let configurations = load::configurations_from_env()?;
    let project_id = configurations["project_id"].as_str().unwrap();
    let api = BlockFrostApi::new(project_id, Default::default());
    Ok(api)
}

#[tokio::main]
async fn main() -> blockfrost::Result<()> {
    let mut api = build_api()?;
    api.settings = api.settings.use_preview();
    let genesis = api.genesis().await?;
    println!("{:#?}", genesis);
    Ok(())
}
