use serde::{Deserialize, Serialize};

use crate::Integer;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockContent {
    /// Block creation time in UNIX time
    pub time: Integer,
    /// Block number
    pub height: Option<Integer>,
    /// Hash of the block
    pub hash: String,
    /// Slot number
    pub slot: Option<Integer>,
    /// Epoch number
    pub epoch: Option<Integer>,
    /// Slot within the epoch
    pub epoch_slot: Option<Integer>,
    /// Bech32 ID of the slot leader or specific block description in case there is no slot leader
    pub slot_leader: String,
    /// Block size in Bytes
    pub size: Integer,
    /// Number of transactions in the block
    pub tx_count: Integer,
    /// Total output within the block in Lovelaces
    pub output: Option<String>,
    /// Total fees within the block in Lovelaces
    pub fees: Option<String>,
    /// VRF key of the block
    pub block_vrf: Option<String>,
    /// Hash of the previous block
    pub previous_block: Option<String>,
    /// Hash of the next block
    pub next_block: Option<String>,
    /// Number of block confirmations
    pub confirmations: Integer,
}

test_schema! { test_block_content, BlockContent, r#"
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
