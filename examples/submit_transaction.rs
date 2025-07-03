use blockfrost::{BlockFrostSettings, BlockfrostAPI, BlockfrostResult};

fn build_api() -> BlockfrostResult<BlockfrostAPI> {
    let api = BlockfrostAPI::new(
        "mainnetxvMK4xOpp5mHJgihi055KDLU64JJv2be",
        BlockFrostSettings::new(),
    );
    Ok(api)
}

#[tokio::main]
async fn main() -> BlockfrostResult<()> {
    let api = build_api()?;

    // Should contain the correct cbor contents
    let transaction_data = vec![0; 1024]; // Just an example (will fail)
    let transaction_hash = api.transactions_submit(transaction_data).await?;

    // At this point, you should probably save the transaction_hash
    println!("{transaction_hash}");

    Ok(())
}
