# blockfrost-rust
Rust SDK for Blockfrost.io

Demo:
```rust
use blockfrost::{BlockFrostApi, Settings};

#[tokio::main]
async fn main() {
    let project_id = "YOUR_KEY_HERE"; // Generate your key at https://blockfrost.io/
    let api = BlockFrostApi::new(project_id, Settings::default());

    let latest_block = api.blocks_latest().await.unwrap();
    let latest_epoch = api.epochs_latest().await.unwrap();
    let health = api.health().await.unwrap();

    println!("latest_block: {:?}.", latest_block);
    println!("latest_epoch: {:?}.", latest_epoch);
    println!("health: {:?}.", health);
}
```
