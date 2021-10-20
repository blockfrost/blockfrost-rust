use std::fs;

use blockfrost::{load, IpfsApi, IpfsSettings};

fn build_ipfs() -> blockfrost::Result<IpfsApi> {
    let configurations = load::configurations_from_env()?;
    let project_id = configurations["project_id"].as_str().unwrap();
    let api = IpfsApi::new(project_id, IpfsSettings::new());
    Ok(api)
}

#[tokio::main]
async fn main() -> blockfrost::Result<()> {
    let ipfs = build_ipfs()?;
    let file = fs::read_to_string("/etc/fstab")?.into_bytes();

    let added_file = ipfs.add(file).await?;
    println!("{:#?}", added_file);

    Ok(())
}
