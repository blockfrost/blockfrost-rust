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
  <a href="#examples">Examples</a> •
  <a href="./ENDPOINTS.md">Endpoints</a>
</p>

## Getting started

To use this SDK you need to login at [blockfrost.io](https://blockfrost.io)
and create a new project to receive an API key.

<img src="https://i.imgur.com/smY12ro.png">

## Installation

Add to your project's `Cargo.toml`:

```toml
blockfrost = "1.3.0"
```

## Examples

All the examples are located at the [`examples/`] folder.

You might want to check [`all_requests.rs`] and [`ipfs.rs`].

Here is [`simple_request.rs`] with the basic setup necessary and no settings
customization:

```rust
use blockfrost::{BlockfrostAPI, BlockfrostResult};

fn build_api() -> BlockfrostResult<BlockfrostAPI> {
    let api = BlockfrostAPI::new("mainnetxvMK4xOpp5mHJgihi055KDLU64JJv2be", Default::default());
    Ok(api)
}

#[tokio::main]
async fn main() -> blockfrost::BlockfrostResult<()> {
    let api = build_api()?;
    let genesis = api.genesis().await?;

    println!("{:#?}", genesis);
    Ok(())
}
```

## Pagination

Every paginated endpoint takes a `Pagination`, which controls `page`, `count` and `order`:

```rust
use blockfrost::{Order, Pagination};

let pagination = Pagination::new(Order::Desc, 1, 100);
let everything = Pagination::all(); // fetch every page
```

### Block range cursors

These endpoints additionally accept inclusive `from`/`to` block cursors:

- `accounts_transactions`
- `addresses_transactions`
- `assets_transactions`
- `assets_utxos` (block heights only, without a transaction index)

```rust
use blockfrost::{BlockCursor, Pagination};

// a plain block height, or a block height with a transaction index
let pagination = Pagination::default().with_range(8929261, BlockCursor::tx(9999269, 10));
```

Cursors also accept `BlockCursor::block(..)`, `BlockCursor::tx(..)` and `"8929261:10".parse()`.
They are preserved across pages when `fetch_all` is set.

> **Note:** every other paginated endpoint ignores `from`/`to`. Passing a `Pagination` that carries
> cursors to, say, `accounts_utxos` returns the full unfiltered list rather than an error, so keep
> cursors on a `Pagination` you only hand to the endpoints above.

[`examples/`]: https://github.com/blockfrost/blockfrost-rust/tree/master/examples
[`all_requests.rs`]: https://github.com/blockfrost/blockfrost-rust/blob/master/examples/all_requests.rs
[`ipfs.rs`]: https://github.com/blockfrost/blockfrost-rust/blob/master/examples/ipfs.rs
[`simple_request.rs`]: https://github.com/blockfrost/blockfrost-rust/blob/master/examples/simple_request.rs
