use blockfrost::{load, BlockFrostApi, BlockFrostSettings, QueryOrder};

fn build_api() -> blockfrost::Result<BlockFrostApi> {
    let configurations = load::configurations_from_env()?;
    let project_id = configurations["project_id"].as_str().unwrap();
    let settings = {
        let mut settings = BlockFrostSettings::new();
        settings
            .query_parameters
            .set_count(5)
            .set_page(10)
            .set_order(QueryOrder::Descending);
        settings
    };
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
