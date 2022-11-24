<p align="center">
  <a href="https://crates.io/crates/blockfrost">
    <img src="https://img.shields.io/crates/v/blockfrost?color=2E83FA" alt="Crates.io link">
  </a>
  <a href="https://docs.rs/blockfrost">
    <img src="https://img.shields.io/docsrs/blockfrost?color=2E83FA" alt="Docs.rs link">
  </a>
  <a href="https://github.com/blockfrost/blockfrost-rust/blob/master/LICENSE">
    <img src="https://img.shields.io/crates/l/blockfrost?color=2E83FA" alt="License">
  </a>
</p>

<img src="https://blockfrost.io/images/logo.svg" width="250" align="right" height="90" style="margin-bottom: -50px">

# blockfrost-rust

<br>
<p align="center">A Rust SDK for <a href="https://blockfrost.io">Blockfrost.io</a> API.</p>
<p align="center">
  <a href="#getting-started">Getting started</a> •
  <a href="#installation">Installation</a> •
  <a href="#examples">Examples</a>
</p>

## Getting started

To use this SDK you need to login at [blockfrost.io](https://blockfrost.io)
and create a new project to receive an API key.

<img src="https://i.imgur.com/smY12ro.png">

## Installation

Add to your project's `Cargo.toml`:

```toml
blockfrost = "0.2.1"
```

## Examples

All the examples are located at the [`examples/`] folder.

You might want to check [`all_requests.rs`] and [`ipfs.rs`].

Here is [`simple_request.rs`] with the basic setup necessary and no settings
customization:

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
    let genesis = api.genesis().await?;

    println!("{:#?}", genesis);
    Ok(())
}
```

[`examples/`]: https://github.com/blockfrost/blockfrost-rust/tree/master/examples
[`all_requests.rs`]: https://github.com/blockfrost/blockfrost-rust/blob/master/examples/all_requests.rs
[`ipfs.rs`]: https://github.com/blockfrost/blockfrost-rust/blob/master/examples/ipfs.rs
[`simple_request.rs`]: https://github.com/blockfrost/blockfrost-rust/blob/master/examples/simple_request.rs
