# blockfrost-rust
Rust SDK for Blockfrost.io

You can find multiple usage examples in the [`examples folder`](./examples)

```rust
use blockfrost::{env, BlockFrostApi, BlockFrostSettings};

fn build_api() -> blockfrost::Result<BlockFrostApi> {
    let project_id = env::load_project_id()?.expect("BLOCKFROST_PROJECT_ID not found.");
    let settings = BlockFrostSettings::new();
    let api = BlockFrostApi::new(project_id, settings);

    Ok(api)
}

#[tokio::main]
async fn main() -> blockfrost::Result<()> {
    let api = build_api()?;

    // A request example
    let health = api.health().await?;
    println!("{:#?}", health);

    Ok(())
}
```
