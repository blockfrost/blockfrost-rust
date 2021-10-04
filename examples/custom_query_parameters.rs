use blockfrost::{env, BlockFrostApi, QueryOrder, Settings};

fn build_api() -> blockfrost::Result<BlockFrostApi> {
    let project_id = env::load_project_id()?.expect("BLOCKFROST_PROJECT_ID not found.");
    let settings = Settings::new().configure(|query| {
        query.set_count(5).set_page(10).set_order(QueryOrder::Descending);
    });
    let api = BlockFrostApi::new(project_id, settings);

    Ok(api)
}

#[tokio::main]
async fn main() -> blockfrost::Result<()> {
    let api = build_api()?;

    let assets = api.assets().await;
    println!("{:#?}", assets);

    Ok(())
}
