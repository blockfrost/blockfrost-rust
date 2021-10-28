<!-- [![Crates.io](https://img.shields.io/crates/v/file_type_enum.svg)](https://crates.io/crates/file_type_enum) -->
<!-- [![Docs.rs](https://docs.rs/file_type_enum/badge.svg)](https://docs.rs/file_type_enum) -->
[![License](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](https://github.com/blockfrost/blockfrost-rust/blob/master/LICENSE)

<img src="https://blockfrost.io/images/logo.svg" width="250" align="right" height="90">

# blockfrost-rust

<br/>

<p align="center">A Rust SDK for Blockfrost.io API.</p>
<p align="center">
  <a href="#getting-started">Getting started</a> •
  <a href="#installation">Installation</a> •
  <a href="#usage">Usage</a>
</p>
<br>

## Getting started

To use this SDK, you first need login into to [blockfrost.io](https://blockfrost.io) create your project to retrieve
your API key.

<img src="https://i.imgur.com/smY12ro.png">

<br/>

## Installation

Add to your project's `Cargo.toml`:

```toml
blockfrost = "0.1.0"
```

<br/>

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
