//! Example using a concurrent lister for listing 20 pages

// NOTE: StreamExt helps treating listers like iterators (actually, they are Streams)
use blockfrost::{env, stream::StreamExt, BlockFrostApi, Settings};

fn build_api() -> blockfrost::Result<BlockFrostApi> {
    let project_id = env::load_project_id()?.expect("BLOCKFROST_PROJECT_ID not found.");
    let settings = Settings::new().configure(|query| {
        // Show 3 elements per page (just for this example)
        query.set_count(3);
    });

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
