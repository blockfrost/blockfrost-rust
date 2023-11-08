use blockfrost::BlockFrostApi;

fn build_api() -> blockfrost::BlockfrostResult<BlockFrostApi> {
    let api = BlockFrostApi::new(
        "mainnetxvMK4xOpp5mHJgihi055KDLU64JJv2be",
        Default::default(),
    );

    Ok(api)
}

#[tokio::main]
async fn main() -> blockfrost::BlockfrostResult<()> {
    let api = build_api()?;
    let genesis = api.genesis().await?;

    println!("{:#?}", genesis);
    Ok(())
}
