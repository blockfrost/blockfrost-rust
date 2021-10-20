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

    // Add file
    let added_file = ipfs.add(file).await?;
    println!("{:#?}", added_file);
    let hash = &added_file.ipfs_hash;

    // Pin it
    let pinned_file = ipfs.pin_add(hash).await?;
    println!("{:#?}", pinned_file);
    let hash = &pinned_file.ipfs_hash;

    // List pins
    let pin_list = ipfs.pin_list().await?;
    println!("{:#?}", pin_list);

    // List pin by ipfs_hash (id)
    let pin_list_by_id = ipfs.pin_list_by_id(hash).await?;
    println!("{:#?}", pin_list_by_id);

    // Query contents
    let gateway = ipfs.gateway(hash).await?;
    let string = String::from_utf8(gateway).unwrap();
    println!("content: '{:#?}'", string);

    // Remove pin
    let pin_removed = ipfs.pin_remove(hash).await?;
    println!("{:#?}", pin_removed);

    Ok(())
}
