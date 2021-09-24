use blockfrost::{env, BlockFrostApi, Settings};

fn build_api() -> blockfrost::Result<BlockFrostApi> {
    let project_id = env::load_project_id()?.expect("BLOCKFROST_PROJECT_ID not found.");
    let settings = Settings::new(project_id).configure(|query| {
        query.set_count(3);
    });
    let api = BlockFrostApi::new(settings);

    Ok(api)
}

#[tokio::main]
async fn main() -> blockfrost::Result<()> {
    let api = build_api()?;

    // Health
    let root = api.root().await;
    let health = api.health().await;
    let health_clock = api.health_clock().await;

    // Metrics
    let metrics = api.metrics().await;
    let metrics_endpoints = api.metrics_endpoints().await;

    // Accounts
    let address = "stake1u9ylzsgxaa6xctf4juup682ar3juj85n8tx3hthnljg47zctvm3rc";
    let accounts = api.accounts(address).await;
    let accounts_rewards = api.accounts_rewards(address).await;
    let accounts_history = api.accounts_history(address).await;
    let accounts_delegations = api.accounts_delegations(address).await;
    let accounts_registrations = api.accounts_registrations(address).await;
    let accounts_withdrawals = api.accounts_withdrawals(address).await;
    let accounts_mirs = api.accounts_mirs(address).await;
    let accounts_addresses = api.accounts_addresses(address).await;
    let accounts_addresses_assets = api.accounts_addresses_assets(address).await;

    // Addresses
    let address =
        "addr1qxqs59lphg8g6qndelq8xwqn60ag3aeyfcp33c2kdp46a09re5df3pzwwmyq946axfcejy5n4x0y99wqpgtp2gd0k09qsgy6pz";
    let addresses = api.addresses(address).await;
    let addresses_total = api.addresses_total(address).await;
    let addresses_utxos = api.addresses_utxos(address).await;
    let addresses_transactions = api.addresses_transactions(address).await;

    // Assets
    let address = "b0d07d45fe9514f80213f4020e5a61241458be626841cde717cb38a76e7574636f696e";
    let assets = api.assets().await;
    let assets_by_id = api.assets_by_id(address).await;
    let assets_history = api.assets_history(address).await;
    let assets_transactions = api.assets_transactions(address).await;
    let assets_addresses = api.assets_addresses(address).await;
    let assets_policy_by_id = api.assets_policy_by_id(address).await;

    // Epochs
    let epoch = 225;
    let pool_id = "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy";
    let epochs_latest = api.epochs_latest().await;
    let epochs_latest_parameters = api.epochs_latest_parameters().await;
    let epochs_by_number = api.epochs_by_number(epoch).await;
    let epochs_next = api.epochs_next(epoch).await;
    let epochs_previous = api.epochs_previous(epoch).await;
    let epochs_stakes = api.epochs_stakes(epoch).await;
    let epochs_stakes_by_pool = api.epochs_stakes_by_pool(epoch, pool_id).await;
    let epochs_blocks = api.epochs_blocks(epoch).await;
    let epochs_blocks_by_pool = api.epochs_blocks_by_pool(epoch, pool_id).await;
    let epochs_parameters = api.epochs_parameters(epoch).await;

    // Pools
    let pools = api.pools().await;
    let pools_retired = api.pools_retired().await;
    let pools_retiring = api.pools_retiring().await;
    let pools_by_id = api.pools_by_id(pool_id).await;
    let pools_history = api.pools_history(pool_id).await;
    let pools_metadata = api.pools_metadata(pool_id).await;
    let pools_relays = api.pools_relays(pool_id).await;
    let pools_delegators = api.pools_delegators(pool_id).await;
    let pools_blocks = api.pools_blocks(pool_id).await;
    let pools_updates = api.pools_updates(pool_id).await;

    println!("root: {:#?}", root);
    println!("health: {:#?}", health);
    println!("health_clock: {:#?}", health_clock);
    println!("metrics: {:#?}", metrics);
    println!("metrics_endpoints: {:#?}", metrics_endpoints);
    println!("accounts: {:#?}", accounts);
    println!("accounts_rewards: {:#?}", accounts_rewards);
    println!("accounts_history: {:#?}", accounts_history);
    println!("accounts_delegations: {:#?}", accounts_delegations);
    println!("accounts_registrations: {:#?}", accounts_registrations);
    println!("accounts_withdrawals: {:#?}", accounts_withdrawals);
    println!("accounts_mirs: {:#?}", accounts_mirs);
    println!("accounts_addresses: {:#?}", accounts_addresses);
    println!("accounts_addresses_assets: {:#?}", accounts_addresses_assets);
    println!("addresses: {:#?}", addresses);
    println!("addresses_total: {:#?}", addresses_total);
    println!("addresses_utxos: {:#?}", addresses_utxos);
    println!("addresses_transactions: {:#?}", addresses_transactions);
    println!("assets: {:#?}", assets);
    println!("assets_by_id: {:#?}", assets_by_id);
    println!("assets_history: {:#?}", assets_history);
    println!("assets_transactions: {:#?}", assets_transactions);
    println!("assets_addresses: {:#?}", assets_addresses);
    println!("assets_policy_by_id: {:#?}", assets_policy_by_id);
    println!("epochs_latest: {:#?}", epochs_latest);
    println!("epochs_latest_parameters: {:#?}", epochs_latest_parameters);
    println!("epochs_by_number: {:#?}", epochs_by_number);
    println!("epochs_next: {:#?}", epochs_next);
    println!("epochs_previous: {:#?}", epochs_previous);
    println!("epochs_stakes: {:#?}", epochs_stakes);
    println!("epochs_stakes_by_pool: {:#?}", epochs_stakes_by_pool);
    println!("epochs_blocks: {:#?}", epochs_blocks);
    println!("epochs_blocks_by_pool: {:#?}", epochs_blocks_by_pool);
    println!("epochs_parameters: {:#?}", epochs_parameters);
    println!("pools: {:#?}", pools);
    println!("pools_retired: {:#?}", pools_retired);
    println!("pools_retiring: {:#?}", pools_retiring);
    println!("pools_by_id: {:#?}", pools_by_id);
    println!("pools_history: {:#?}", pools_history);
    println!("pools_metadata: {:#?}", pools_metadata);
    println!("pools_relays: {:#?}", pools_relays);
    println!("pools_delegators: {:#?}", pools_delegators);
    println!("pools_blocks: {:#?}", pools_blocks);
    println!("pools_updates: {:#?}", pools_updates);

    Ok(())
}
