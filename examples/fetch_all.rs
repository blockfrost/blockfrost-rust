use blockfrost::{BlockFrostSettings, BlockfrostAPI, BlockfrostResult, Pagination};

fn build_api_preprod() -> BlockfrostResult<BlockfrostAPI> {
    let settings = BlockFrostSettings::new();
    let api = BlockfrostAPI::new("preprododflHjPhpRp4NzRFL1m9zzd6ZJb1RjYi", settings);

    Ok(api)
}

fn build_api_preview() -> BlockfrostResult<BlockfrostAPI> {
    let settings = BlockFrostSettings::new();
    let api = BlockfrostAPI::new("previewy2pbyga8FifUwJSverBCwhESegV6I7gT", settings);

    Ok(api)
}

#[tokio::main]
async fn main() -> BlockfrostResult<()> {
    let pagination_all = Pagination::all();

    println!("Fetching ...");

    let all_utxos_for_asset = build_api_preprod()?
        .addresses_utxos_asset(
            "addr_test1wz2mzj532enpgu5vgwxuh249fpknx5ft9wxse2876z0mp2q89ye7k",
            "c6e65ba7878b2f8ea0ad39287d3e2fd256dc5c4160fc19bdf4c4d87e7447454e53",
            pagination_all,
        )
        .await;

    println!(
        "count of all all_utxos_for_asset: {:#?}",
        all_utxos_for_asset?.len()
    );

    let all_utxos = build_api_preview()?
        .addresses_utxos(
            "addr_test1qqcxj57pwhesgkty9uzyyzq6hnl983nnzfug7f99dsntxayxvw0zufye5rc6nz7hcetdsx0ex03wjdrg8xrw9jen2lyslqawdk",
            pagination_all,
        )
        .await;

    println!("count of all utxos: {:#?}", all_utxos?.len());

    Ok(())
}
