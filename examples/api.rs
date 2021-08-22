use blockfrost::{env, BlockFrostApi, Settings};

fn build_api() -> BlockFrostApi {
    let project_id = env::load_project_id().expect("Could not read key");
    let settings = Settings::new(project_id).set_test_network(true);
    BlockFrostApi::new(settings)
}

#[tokio::main]
async fn main() -> blockfrost::Result<()> {
    let api = build_api();

    let health = api.health().await;
    let root = api.root().await;

    println!("{:#?}", health);
    println!("{:#?}", root);

    Ok(())
}
