//! Example using a concurrent lister for listing 20 pages

// Importing traits from blockfrost::stream is necessary to iterate throught pages
use blockfrost::{env, stream::*, BlockFrostApi, Settings};

fn build_api() -> blockfrost::Result<BlockFrostApi> {
    let project_id = env::load_project_id()?.expect("BLOCKFROST_PROJECT_ID not found.");
    let settings = Settings::new().configure(|query| {
        // Show 2 elements per page to make the output cleaner, as this is just an usage example
        query.set_count(2);
    });

    let api = BlockFrostApi::new(project_id, settings);
    Ok(api)
}

#[tokio::main]
async fn main() -> blockfrost::Result<()> {
    let api = build_api()?;

    let block_number = "6316012";
    let mut block_lister = api.blocks_previous_all(block_number);

    while let Some(page) = block_lister.next().await {
        let page = page?;
        dbg!(page);
    }

    Ok(())
}
