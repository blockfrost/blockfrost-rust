//! Example using a concurrent lister for listing 20 pages
//!
//! You shall use `StreamExt` to iterate on a `Lister` (implements `Stream`).

use blockfrost::{load, stream::StreamExt, BlockFrostApi, BlockFrostSettings};

fn build_api() -> blockfrost::Result<BlockFrostApi> {
    let configurations = load::configurations_from_env()?;
    let project_id = configurations["project_id"].as_str().unwrap();

    let mut settings = BlockFrostSettings::new();
    // Show 3 elements per page (just for this example)
    settings.query_parameters.set_count(3);

    let api = BlockFrostApi::new(project_id, settings);
    Ok(api)
}

#[tokio::main]
async fn main() -> blockfrost::Result<()> {
    let api = build_api()?;

    let latest_block_number = "5000000";
    // If not limited by `.take()`, will run until first Cardano block
    let mut block_lister = api.blocks_previous_all(latest_block_number).take(5);

    while let Some(page) = block_lister.next().await {
        let page = page?;
        dbg!(page);
    }

    Ok(())
}
