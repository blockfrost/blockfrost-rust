use blockfrost::{BlockFrostSettings, BlockfrostAPI, BlockfrostResult, Pagination};

fn build_api() -> BlockfrostResult<BlockfrostAPI> {
    let settings = BlockFrostSettings::new();
    let api = BlockfrostAPI::new("mainnetxvMK4xOpp5mHJgihi055KDLU64JJv2be", settings);

    Ok(api)
}

#[tokio::main]
async fn main() -> BlockfrostResult<()> {
    let api = build_api()?;
    let pagination_all = Pagination::all();

    println!("Fetching ...");

    // Accounts
    let address = "stake1u9ylzsgxaa6xctf4juup682ar3juj85n8tx3hthnljg47zctvm3rc";
    let accounts_rewards = api.accounts_rewards(address, pagination_all).await;

    Ok(())
}
