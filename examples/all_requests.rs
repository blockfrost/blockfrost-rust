use blockfrost::{env, BlockFrostApi, Settings};

fn build_api() -> blockfrost::Result<BlockFrostApi> {
    let project_id = env::load_project_id()?.expect("BLOCKFROST_PROJECT_ID not found.");
    let settings = Settings::new(project_id).set_test_network(true);
    let api = BlockFrostApi::new(settings);

    Ok(api)
}

#[tokio::main]
async fn main() -> blockfrost::Result<()> {
    let api = build_api()?;

    // let root = api.root().await;
    // let health = api.health().await;
    // let health_clock = api.health_clock().await;
    // let metrics = api.metrics().await;
    // let metrics_endpoints = api.metrics_endpoints().await;
    // let account = api.accounts("e1bc000424731d5cfdb41da8bbb2e8bfbdcf05becd055a3f831a0ccdfa").await;
    let account_rewards = api.accounts_rewards("e1bc000424731d5cfdb41da8bbb2e8bfbdcf05becd055a3f831a0ccdfa").await;

    // println!("{:#?}", root);
    // println!("{:#?}", health);
    // println!("{:#?}", health_clock);
    // println!("{:#?}", metrics);
    // println!("{:#?}", metrics_endpoints);
    // println!("{:#?}", account);
    println!("{:#?}", account_rewards);

    Ok(())
}
