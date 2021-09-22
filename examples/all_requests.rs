use blockfrost::{env, BlockFrostApi, Settings};

fn build_api() -> blockfrost::Result<BlockFrostApi> {
    let project_id = env::load_project_id()?.expect("BLOCKFROST_PROJECT_ID not found.");
    let settings = Settings::new(project_id);
    let api = BlockFrostApi::new(settings);

    Ok(api)
}

#[tokio::main]
async fn main() -> blockfrost::Result<()> {
    let api = build_api()?;

    // Arbitrary example text used in requisitions that require id or address
    let address = "example-address-01234567890123456789";
    let pool = "example-pool-01234567890123456789";

    let root = api.root().await;
    let health = api.health().await;
    let health_clock = api.health_clock().await;
    let metrics = api.metrics().await;
    let metrics_endpoints = api.metrics_endpoints().await;
    let accounts = api.accounts(address).await;
    let accounts_rewards = api.accounts_rewards(address).await;
    let accounts_history = api.accounts_history(address).await;
    let accounts_delegations = api.accounts_delegations(address).await;
    let accounts_registrations = api.accounts_registrations(address).await;
    let accounts_withdrawals = api.accounts_withdrawals(address).await;
    let accounts_mirs = api.accounts_mirs(address).await;
    let accounts_addresses = api.accounts_addresses(address).await;
    let accounts_addresses_assets = api.accounts_addresses_assets(address).await;
    let addresses = api.addresses(address).await;
    let addresses_total = api.addresses_total(address).await;
    let addresses_utxos = api.addresses_utxos(address).await;
    let addresses_transactions = api.addresses_transactions(address).await;
    let assets = api.assets().await;
    let assets_by_id = api.assets_by_id(address).await;
    let assets_history = api.assets_history(address).await;
    let assets_transactions = api.assets_transactions(address).await;
    let assets_addresses = api.assets_addresses(address).await;
    let assets_policy_by_id = api.assets_policy_by_id(address).await;
    let pools = api.pools().await;
    let pools_retired = api.pools_retired().await;
    let pools_retiring = api.pools_retiring().await;
    let pools_by_id = api.pools_by_id(pool).await;
    let pools_history = api.pools_history(pool).await;
    let pools_metadata = api.pools_metadata(pool).await;
    let pools_relays = api.pools_relays(pool).await;
    let pools_delegators = api.pools_delegators(pool).await;
    let pools_blocks = api.pools_blocks(pool).await;
    let pools_updates = api.pools_updates(pool).await;

    println!("{:#?}", root);
    println!("{:#?}", health);
    println!("{:#?}", health_clock);
    println!("{:#?}", metrics);
    println!("{:#?}", metrics_endpoints);
    println!("{:#?}", accounts);
    println!("{:#?}", accounts_rewards);
    println!("{:#?}", accounts_history);
    println!("{:#?}", accounts_delegations);
    println!("{:#?}", accounts_registrations);
    println!("{:#?}", accounts_withdrawals);
    println!("{:#?}", accounts_mirs);
    println!("{:#?}", accounts_addresses);
    println!("{:#?}", accounts_addresses_assets);
    println!("{:#?}", addresses);
    println!("{:#?}", addresses_total);
    println!("{:#?}", addresses_utxos);
    println!("{:#?}", addresses_transactions);
    println!("{:#?}", assets);
    println!("{:#?}", assets_by_id);
    println!("{:#?}", assets_history);
    println!("{:#?}", assets_transactions);
    println!("{:#?}", assets_addresses);
    println!("{:#?}", assets_policy_by_id);
    println!("{:#?}", pools);
    println!("{:#?}", pools_retired);
    println!("{:#?}", pools_retiring);
    println!("{:#?}", pools_by_id);
    println!("{:#?}", pools_history);
    println!("{:#?}", pools_metadata);
    println!("{:#?}", pools_relays);
    println!("{:#?}", pools_delegators);
    println!("{:#?}", pools_blocks);
    println!("{:#?}", pools_updates);

    Ok(())
}
