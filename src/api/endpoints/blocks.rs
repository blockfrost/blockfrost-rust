use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use serde::{Deserialize, Serialize};

use crate::{
    stream::{FuturesOrdered, Stream},
    *,
};

type ListerFuture<'api> = dyn Future<Output = crate::Result<Vec<Block>>> + Send + 'api;
type PinnedListerFuture<'api> = Pin<Box<ListerFuture<'api>>>;

pub struct BlockLister<'api> {
    inner: FuturesOrdered<PinnedListerFuture<'api>>,
    api: &'api BlockFrostApi,
    url_suffix: String,
}

impl<'api> Stream for BlockLister<'api> {
    type Item = crate::Result<Vec<Block>>;

    fn poll_next(mut self: Pin<&mut Self>, context: &mut Context) -> Poll<Option<Self::Item>> {
        let item = Pin::new(&mut self.inner).poll_next(context);

        if let Poll::Ready(Some(_)) = &item {
            let future = self.api.blocks_next("foobar");
            self.inner.push(Box::pin(future));
        }

        item
    }
}

impl BlockFrostApi {
    pub fn blocks_previous_all<'api>(&'api self, hash_or_number: &str) -> BlockLister<'api> {
        let mut inner: FuturesOrdered<PinnedListerFuture> = FuturesOrdered::new();
        let url_suffix = format!("/blocks/{hash_or_number}/next", hash_or_number = hash_or_number);
        let future = self.blocks_previous(hash_or_number);
        inner.push(Box::pin(future));
        BlockLister { inner, api: self, url_suffix }
    }
}

impl BlockFrostApi {
    endpoints! {
        /// Return the latest block available to the backends, also known as the tip of the blockchain.
        blocks_latest() -> Block => "/blocks/latest";
            ("https://docs.blockfrost.io/#tag/Cardano-Blocks/paths/~1blocks~1latest/get"),

        /// Return the transactions within the latest block.
        blocks_latest_txs() -> Vec<String> => "/blocks/latest/txs";
            ("https://docs.blockfrost.io/#tag/Cardano-Blocks/paths/~1blocks~1latest~1txs/get"),

        /// Return the content of a requested block.
        blocks_by_id(hash_or_number: &str) -> Block => "/blocks/{hash_or_number}";
            ("https://docs.blockfrost.io/#tag/Cardano-Blocks/paths/~1blocks~1{hash_or_number}/get"),

        /// Return the content of a requested block for a specific slot.
        blocks_slot(slot_number: Integer) -> Block => "/blocks/slot/{slot_number}";
            ("https://docs.blockfrost.io/#tag/Cardano-Blocks/paths/~1blocks~1slot~1{slot_number}/get"),

        /// Return the content of a requested block for a specific slot in an epoch.
        blocks_by_epoch_and_slot(epoch_number: Integer, slot_number: Integer) -> Block => "/blocks/epoch/{epoch_number}/slot/{slot_number}";
            ("https://docs.blockfrost.io/#tag/Cardano-Blocks/paths/~1blocks~1epoch~1{epoch_number}~1slot~1{slot_number}/get"),

        /// Return the list of blocks following a specific block.
        blocks_next(hash_or_number: &str) -> Vec<Block> => "/blocks/{hash_or_number}/next";
            ("https://docs.blockfrost.io/#tag/Cardano-Blocks/paths/~1blocks~1{hash_or_number}~1next/get"),

        /// Return the list of blocks preceding a specific block.
        blocks_previous(hash_or_number: &str) -> Vec<Block> => "/blocks/{hash_or_number}/previous";
            ("https://docs.blockfrost.io/#tag/Cardano-Blocks/paths/~1blocks~1{hash_or_number}~1previous/get"),

        /// Return the transactions within the block.
        blocks_txs(hash_or_number: &str) -> Vec<String> => "/blocks/{hash_or_number}/txs";
            ("https://docs.blockfrost.io/#tag/Cardano-Blocks/paths/~1blocks~1{hash_or_number}~1txs/get"),
    }
}

/// Created by [`blocks_latest`](BlockFrostApi::blocks_latest) and other 5 methods.
///
/// Methods that can retrieve this type:
/// [`BlockFrostApi::blocks_latest`].
/// [`BlockFrostApi::blocks_by_id`].
/// [`BlockFrostApi::blocks_slot`].
/// [`BlockFrostApi::blocks_by_epoch_and_slot`].
/// [`BlockFrostApi::blocks_next`].
/// [`BlockFrostApi::blocks_previous`].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    /// Block creation time in UNIX time.
    pub time: Integer,
    /// Block number.
    pub height: Option<Integer>,
    /// Hash of the block.
    pub hash: String,
    /// Slot number.
    pub slot: Option<Integer>,
    /// Epoch number.
    pub epoch: Option<Integer>,
    /// Slot within the epoch.
    pub epoch_slot: Option<Integer>,
    /// Bech32 ID of the slot leader or specific block description in case there is no slot leader.
    pub slot_leader: String,
    /// Block size in Bytes.
    pub size: Integer,
    /// Number of transactions in the block.
    pub tx_count: Integer,
    /// Total output within the block in Lovelaces.
    pub output: Option<String>,
    /// Total fees within the block in Lovelaces.
    pub fees: Option<String>,
    /// VRF key of the block (exactly 65 characters).
    pub block_vrf: Option<String>,
    /// Hash of the previous block.
    pub previous_block: Option<String>,
    /// Hash of the next block.
    pub next_block: Option<String>,
    /// Number of block confirmations.
    pub confirmations: Integer,
}

#[cfg(test)]
mod tests {
    use super::*;

    test_example! { test_blocks_latest, Block, r#"
    {
      "time": 1641338934,
      "height": 15243593,
      "hash": "4ea1ba291e8eef538635a53e59fddba7810d1679631cc3aed7c8e6c4091a516a",
      "slot": 412162133,
      "epoch": 425,
      "epoch_slot": 12,
      "slot_leader": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2qnikdy",
      "size": 3,
      "tx_count": 1,
      "output": "128314491794",
      "fees": "592661",
      "block_vrf": "vrf_vk1wf2k6lhujezqcfe00l6zetxpnmh9n6mwhpmhm0dvfh3fxgmdnrfqkms8ty",
      "previous_block": "43ebccb3ac72c7cebd0d9b755a4b08412c9f5dcb81b8a0ad1e3c197d29d47b05",
      "next_block": "8367f026cf4b03e116ff8ee5daf149b55ba5a6ec6dec04803b8dc317721d15fa",
      "confirmations": 4698
    }
    "# }

    test_example! { test_blocks_latest_txs, Vec<String>, r#"
    [
      "8788591983aa73981fc92d6cddbbe643959f5a784e84b8bee0db15823f575a5b",
      "4eef6bb7755d8afbeac526b799f3e32a624691d166657e9d862aaeb66682c036",
      "52e748c4dec58b687b90b0b40d383b9fe1f24c1a833b7395cdf07dd67859f46f",
      "e8073fd5318ff43eca18a852527166aa8008bee9ee9e891f585612b7e4ba700b"
    ]
    "# }

    test_example! { test_blocks_next, Vec<Block>, r#"
    [
      {
        "time": 1641338934,
        "height": 15243593,
        "hash": "4ea1ba291e8eef538635a53e59fddba7810d1679631cc3aed7c8e6c4091a516a",
        "slot": 412162133,
        "epoch": 425,
        "epoch_slot": 12,
        "slot_leader": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2qnikdy",
        "size": 3,
        "tx_count": 1,
        "output": "128314491794",
        "fees": "592661",
        "block_vrf": "vrf_vk1wf2k6lhujezqcfe00l6zetxpnmh9n6mwhpmhm0dvfh3fxgmdnrfqkms8ty",
        "previous_block": "43ebccb3ac72c7cebd0d9b755a4b08412c9f5dcb81b8a0ad1e3c197d29d47b05",
        "next_block": "8367f026cf4b03e116ff8ee5daf149b55ba5a6ec6dec04803b8dc317721d15fa",
        "confirmations": 4698
      }
    ]
    "# }

    test_example! { test_blocks_txs, Vec<String>, r#"
    [
      "8788591983aa73981fc92d6cddbbe643959f5a784e84b8bee0db15823f575a5b",
      "4eef6bb7755d8afbeac526b799f3e32a624691d166657e9d862aaeb66682c036",
      "52e748c4dec58b687b90b0b40d383b9fe1f24c1a833b7395cdf07dd67859f46f",
      "e8073fd5318ff43eca18a852527166aa8008bee9ee9e891f585612b7e4ba700b"
    ]
    "# }
}
