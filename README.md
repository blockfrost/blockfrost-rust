<p align="center">
  <a href="https://crates.io/crates/blockfrost">
    <img src="https://img.shields.io/crates/v/blockfrost?color=0A60DD" alt="Crates.io link">
  </a>
  <a href="https://docs.rs/blockfrost">
    <img src="https://img.shields.io/docsrs/blockfrost?color=%230A60DD" alt="Docs.rs link">
  </a>
  <a href="https://github.com/blockfrost/blockfrost-rust/blob/master/LICENSE">
    <img src="https://img.shields.io/crates/l/blockfrost?color=%230A60DD" alt="License">
  </a>
</p>


<img src="https://blockfrost.io/images/logo.svg" width="250" align="right" height="90">

# blockfrost-rust

<p align="center">A Rust SDK for <a href="https://blockfrost.io">Blockfrost.io</a> API.</p>
<p align="center">
  <a href="#getting-started">Getting started</a> •
  <a href="#installation">Installation</a> •
  <a href="#usage">Usage</a>
</p>

## Getting started

To use this SDK, you first need login into to [blockfrost.io](https://blockfrost.io) create your project to retrieve
your API key.

<img src="https://i.imgur.com/smY12ro.png">

## Installation

Add to your project's `Cargo.toml`:

```toml
blockfrost = "0.1.1"
```

## Usage

There are multiple other examples in the [`examples/`](./examples) folder.

### Simple example

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
