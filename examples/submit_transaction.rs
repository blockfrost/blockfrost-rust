use blockfrost::{env, BlockFrostApi, BlockFrostSettings};

fn build_api() -> blockfrost::Result<BlockFrostApi> {
    let project_id = env::load_project_id()?.expect("BLOCKFROST_PROJECT_ID not found.");
    let api = BlockFrostApi::new(project_id, BlockFrostSettings::new());
    Ok(api)
}

#[tokio::main]
async fn main() -> blockfrost::Result<()> {
    let api = build_api()?;

    // Should contain the correct cbor contents
    let transaction_data = vec![0; 1024]; // Just an example (will fail)
    let transaction_hash = api.transactions_submit(transaction_data).await?;

    // At this point, you should probably save the transaction_hash
    println!("{}", transaction_hash);

    Ok(())
}
