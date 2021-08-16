# blockfrost-rust
Rust SDK for Blockfrost.io

Demo:
```rust
use blockfrost::BlockFrostApi;

#[tokio::main]
async fn main() -> blockfrost::Result<()> {
    // Gather your key at https://blockfrost.io/
    let api = BlockFrostApi::new("YOUR_KEY_HERE");

    let latest_block = api.blocks_latest().await?;
    let latest_epoch = api.epochs_latest().await?;
    let health = api.health().await?;

    println!("latest_block: {:?}.", latest_block);
    println!("latest_epoch: {:?}.", latest_epoch);
    println!("health: {:?}.", health);

    Ok(())
}
```
