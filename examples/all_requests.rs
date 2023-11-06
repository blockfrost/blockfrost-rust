use blockfrost::{load, BlockFrostApi, BlockFrostSettings, Order, Pagination};

fn build_api() -> blockfrost::Result<BlockFrostApi> {
    let configurations = load::configurations_from_env()?;
    let project_id = configurations["project_id"].as_str().unwrap();
    let mut settings = BlockFrostSettings::new();
    // Limit quantity of items per page listed
    settings.query_parameters.set_count(2);
    let api = BlockFrostApi::new(project_id, settings);
    Ok(api)
}

#[tokio::main]
async fn main() -> blockfrost::Result<()> {
    let api = build_api()?;
    let pagination = Some(Pagination {
        count: 1,
        page: 1,
        order: Order::Asc,
        fetch_all: false,
    });

    // Health
    let root = api.root().await;
    let health = api.health().await;
    let health_clock = api.health_clock().await;

    // // Metrics
    let metrics = api.metrics(None).await;
    let metrics_endpoints = api.metrics_endpoints(None).await;

    // Accounts
    let address = "stake1u9ylzsgxaa6xctf4juup682ar3juj85n8tx3hthnljg47zctvm3rc";
    let accounts = api.accounts(address).await;
    let accounts_rewards = api.accounts_rewards(address, pagination).await;
    let accounts_history = api.accounts_history(address, pagination).await;
    let accounts_delegations = api.accounts_delegations(address, pagination).await;
    let accounts_registrations = api.accounts_registrations(address, pagination).await;
    let accounts_withdrawals = api.accounts_withdrawals(address, pagination).await;
    let accounts_mirs = api.accounts_mirs(address, pagination).await;
    let accounts_addresses = api.accounts_addresses(address, pagination).await;
    let accounts_addresses_assets = api.accounts_addresses_assets(address, pagination).await;

    // Addresses
    let address =
        "addr1qxqs59lphg8g6qndelq8xwqn60ag3aeyfcp33c2kdp46a09re5df3pzwwmyq946axfcejy5n4x0y99wqpgtp2gd0k09qsgy6pz";
    let addresses = api.addresses(address).await;
    let addresses_total = api.addresses_total(address).await;
    let addresses_utxos = api.addresses_utxos(address, pagination).await;
    let addresses_transactions = api.addresses_transactions(address, pagination).await;

    // Assets
    let asset_name = "6e7574636f696e";
    let policy_id = "00000002df633853f6a47465c9496721d2d5b1291b8398016c0e87ae";
    let asset = asset_name.to_string() + policy_id;
    let assets = api.assets(pagination).await;
    let assets_by_id = api.assets_by_id(&asset).await;
    let assets_history = api.assets_history(&asset, pagination).await;
    let assets_transactions = api.assets_transactions(&asset, pagination).await;
    let assets_addresses = api.assets_addresses(&asset, pagination).await;
    let assets_policy_by_id = api.assets_policy_by_id(policy_id, pagination).await;

    // Epochs
    let epoch = 225;
    let pool_id = "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy";
    let epochs_latest = api.epochs_latest().await;
    let epochs_latest_parameters = api.epochs_latest_parameters().await;
    let epochs_by_number = api.epochs_by_number(epoch).await;
    let epochs_next = api.epochs_next(epoch, pagination).await;
    let epochs_previous = api.epochs_previous(epoch, pagination).await;
    let epochs_stakes = api.epochs_stakes(epoch, pagination).await;
    let epochs_stakes_by_pool = api.epochs_stakes_by_pool(epoch, pool_id, pagination).await;
    let epochs_blocks = api.epochs_blocks(epoch).await;
    let epochs_blocks_by_pool = api.epochs_blocks_by_pool(epoch, pool_id).await;
    let epochs_parameters = api.epochs_parameters(epoch).await;

    // Pools
    let pools = api.pools(pagination).await;
    let pools_retired = api.pools_retired(pagination).await;
    let pools_retiring = api.pools_retiring(pagination).await;
    let pools_by_id = api.pools_by_id(pool_id).await;
    let pools_history = api.pools_history(pool_id, pagination).await;
    let pools_metadata = api.pools_metadata(pool_id).await;
    let pools_relays = api.pools_relays(pool_id).await;
    let pools_delegators = api.pools_delegators(pool_id, pagination).await;
    let pools_blocks = api.pools_blocks(pool_id, pagination).await;
    let pools_updates = api.pools_updates(pool_id, pagination).await;

    // Blocks
    let block = "4874756";
    let slot = 12268658;
    let blocks_latest = api.blocks_latest().await;
    let blocks_latest_txs = api.blocks_latest_txs(pagination).await;
    let blocks_by_id = api.blocks_by_id(block).await;
    let blocks_slot = api.blocks_slot(slot).await;
    let blocks_by_epoch_and_slot = api.blocks_by_epoch_and_slot(epoch, slot).await; // Not found
    let blocks_next = api.blocks_next(block, pagination).await;
    let blocks_previous = api.blocks_previous(block, pagination).await;
    let blocks_txs = api.blocks_txs(block, pagination).await;
    let blocks_affected_addresses = api.blocks_affected_addresses("3746845", pagination).await;

    // Ledger
    let genesis = api.genesis().await;

    // Metadata
    let label = "1990";
    let metadata_txs_labels = api.metadata_txs_labels(pagination).await;
    let metadata_txs_by_label = api.metadata_txs_by_label(label, pagination).await;
    let metadata_txs_by_label_cbor = api.metadata_txs_by_label_cbor(label, pagination).await;

    // Network
    let network = api.network().await;

    // Scripts
    let script_hash = "e1457a0c47dfb7a2f6b8fbb059bdceab163c05d34f195b87b9f2b30e";
    let scripts = api.scripts(pagination).await;
    let scripts_by_id = api.scripts_by_id(script_hash).await;
    let scripts_redeemers = api.scripts_redeemers(script_hash, pagination).await;

    // Transactions
    let transaction_hash = "cb0e7b0cd0f0edbe5c6e260c369f5f0f0069cd41f501243ab67f1052040de28f";
    let transaction_by_hash = api.transaction_by_hash(transaction_hash).await;
    let transactions_utxos = api.transactions_utxos(transaction_hash).await;
    let transactions_stakes = api.transactions_stakes(transaction_hash).await;
    let transactions_delegations = api.transactions_delegations(transaction_hash).await;
    let transactions_withdrawals = api.transactions_withdrawals(transaction_hash).await;
    let transactions_mirs = api.transactions_mirs(transaction_hash).await;
    let transactions_pool_updates = api.transactions_pool_updates(transaction_hash).await;
    let transactions_pool_retires = api.transactions_pool_retires(transaction_hash).await;
    let transactions_metadata = api.transactions_metadata(transaction_hash).await;
    let transactions_metadata_cbor = api.transactions_metadata_cbor(transaction_hash).await;
    let transactions_redeemers = api.transactions_redeemers(transaction_hash).await;

    // Nutlink
    let address =
        "addr1qxqs59lphg8g6qndelq8xwqn60ag3aeyfcp33c2kdp46a09re5df3pzwwmyq946axfcejy5n4x0y99wqpgtp2gd0k09qsgy6pz";
    let ticker = "ADAUSD";
    let nutlink_address = api.nutlink_address(address).await;
    let nutlink_address_tickers = api.nutlink_address_tickers(address, pagination).await;
    let nutlink_address_ticker_by_id = api
        .nutlink_address_ticker_by_id(address, ticker, pagination)
        .await;
    let nutlink_ticker_by_id = api.nutlink_ticker_by_id(ticker, pagination).await;

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
    println!(
        "accounts_addresses_assets: {:#?}",
        accounts_addresses_assets
    );
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
    println!("blocks_latest: {:#?}", blocks_latest);
    println!("blocks_latest_txs: {:#?}", blocks_latest_txs);
    println!("blocks_by_id: {:#?}", blocks_by_id);
    println!("blocks_slot: {:#?}", blocks_slot);
    println!("blocks_by_epoch_and_slot: {:#?}", blocks_by_epoch_and_slot);
    println!("blocks_next: {:#?}", blocks_next);
    println!("blocks_previous: {:#?}", blocks_previous);
    println!("blocks_txs: {:#?}", blocks_txs);
    println!(
        "blocks_affected_addresses: {:#?}",
        blocks_affected_addresses
    );
    println!("genesis: {:#?}", genesis);
    println!("metadata_txs_labels: {:#?}", metadata_txs_labels);
    println!("metadata_txs_by_label: {:#?}", metadata_txs_by_label);
    println!(
        "metadata_txs_by_label_cbor: {:#?}",
        metadata_txs_by_label_cbor
    );
    println!("network: {:#?}", network);
    println!("scripts: {:#?}", scripts);
    println!("scripts_by_id: {:#?}", scripts_by_id);
    println!("scripts_redeemers: {:#?}", scripts_redeemers);
    println!("transaction_by_hash: {:#?}", transaction_by_hash);
    println!("transactions_utxos: {:#?}", transactions_utxos);
    println!("transactions_stakes: {:#?}", transactions_stakes);
    println!("transactions_delegations: {:#?}", transactions_delegations);
    println!("transactions_withdrawals: {:#?}", transactions_withdrawals);
    println!("transactions_mirs: {:#?}", transactions_mirs);
    println!(
        "transactions_pool_updates: {:#?}",
        transactions_pool_updates
    );
    println!(
        "transactions_pool_retires: {:#?}",
        transactions_pool_retires
    );
    println!("transactions_metadata: {:#?}", transactions_metadata);
    println!(
        "transactions_metadata_cbor: {:#?}",
        transactions_metadata_cbor
    );
    println!("transactions_redeemers: {:#?}", transactions_redeemers);
    println!("nutlink_address: {:#?}", nutlink_address);
    println!("nutlink_address_tickers: {:#?}", nutlink_address_tickers);
    println!(
        "nutlink_address_ticker_by_id: {:#?}",
        nutlink_address_ticker_by_id
    );
    println!("nutlink_ticker_by_id: {:#?}", nutlink_ticker_by_id);

    Ok(())
}
