use blockfrost::{BlockfrostAPI, BlockfrostResult};

fn build_api() -> BlockfrostResult<BlockfrostAPI> {
    let api = BlockfrostAPI::new(
        "mainnetxvMK4xOpp5mHJgihi055KDLU64JJv2be",
        Default::default(),
    );

    Ok(api)
}

#[tokio::main]
async fn main() -> BlockfrostResult<()> {
    let api = build_api()?;
    let genesis = api.genesis().await?;

    println!("{:#?}", genesis);
    Ok(())
}
