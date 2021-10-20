# blockfrost-rust
Rust SDK for Blockfrost.io

# Basic example

This and more examples can be found in the [`examples/`](./examples) folder.

```rust
use blockfrost::{load, BlockFrostApi};

fn build_api() -> blockfrost::Result<BlockFrostApi> {
    let configurations = load::configurations_from_env()?;
    let project_id = configurations["project_id"].as_str().unwrap();
    let api = BlockFrostApi::new(project_id, Default::default());
    Ok(api)
}

#[tokio::main]
async fn main() -> blockfrost::Result<()> {
    let api = build_api()?;
    let health = api.health().await?;

    println!("{:#?}", health);
    Ok(())
}
```
