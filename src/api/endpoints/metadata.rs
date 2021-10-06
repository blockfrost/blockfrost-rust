use serde::{Deserialize, Serialize};

use crate::*;

impl BlockFrostApi {
    paged_endpoints! {
        /// List of all used transaction metadata labels.
        metadata_txs_labels() -> Vec<MetadataTxsLabel> => "/metadata/txs/labels";
            ("https://docs.blockfrost.io/#tag/Cardano-Metadata/paths/~1metadata~1txs~1labels/get"),

        /// Transaction metadata per label.
        metadata_txs_by_label(label: &str) -> Vec<MetadataTxsLabelJson> => "/metadata/txs/labels/{label}";
            ("https://docs.blockfrost.io/#tag/Cardano-Metadata/paths/~1metadata~1txs~1labels~1{label}/get"),

        /// Transaction metadata per label.
        metadata_txs_by_label_cbor(label: &str) -> Vec<MetadataTxsLabelCbor> => "/metadata/txs/labels/{label}/cbor";
            ("https://docs.blockfrost.io/#tag/Cardano-Metadata/paths/~1metadata~1txs~1labels~1{label}~1cbor/get"),
    }
}

/// Created by [`metadata_txs_labels`](BlockFrostApi::metadata_txs_labels) method.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MetadataTxsLabel {
    /// Metadata label.
    pub label: String,
    /// CIP10 defined description.
    pub cip10: Option<String>,
    /// The count of metadata entries with a specific label.
    pub count: String,
}

/// Created by [`metadata_txs_by_label`](BlockFrostApi::metadata_txs_by_label) method.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MetadataTxsLabelJson {
    /// Transaction hash that contains the specific metadata.
    pub tx_hash: String,
    /// Content of the JSON metadata.
    pub json_metadata: Option<JsonValue>,
}

/// Created by [`metadata_txs_by_label_cbor`](BlockFrostApi::metadata_txs_by_label_cbor) method.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MetadataTxsLabelCbor {
    /// Transaction hash that contains the specific metadata.
    pub tx_hash: String,
    /// Content of the CBOR metadata.
    pub cbor_metadata: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    test_example! { test_metadata_txs_labels, Vec<MetadataTxsLabel>, r#"
    [
      {
        "label": "1990",
        "cip10": null,
        "count": "1"
      },
      {
        "label": "1967",
        "cip10": "nut.link metadata oracles registry",
        "count": "3"
      },
      {
        "label": "1968",
        "cip10": "nut.link metadata oracles data points",
        "count": "16321"
      }
    ]
    "# }

    test_example! { test_metadata_txs_by_label, Vec<MetadataTxsLabelJson>, r#"
    [
      {
        "tx_hash": "257d75c8ddb0434e9b63e29ebb6241add2b835a307aa33aedba2effe09ed4ec8",
        "json_metadata": {
          "ADAUSD": [
            {
              "value": "0.10409800535729975",
              "source": "ergoOracles"
            }
          ]
        }
      },
      {
        "tx_hash": "e865f2cc01ca7381cf98dcdc4de07a5e8674b8ea16e6a18e3ed60c186fde2b9c",
        "json_metadata": {
          "ADAUSD": [
            {
              "value": "0.15409850555139935",
              "source": "ergoOracles"
            }
          ]
        }
      },
      {
        "tx_hash": "4237501da3cfdd53ade91e8911e764bd0699d88fd43b12f44a1f459b89bc91be",
        "json_metadata": null
      },
      {
        "json_metadata": [
          {
            "eng": {
              "content": [
                "For more information, visit our website www.stakenuts.com."
              ],
              "title": "Welcome to NUTS!"
            }
          }
        ],
        "tx_hash": "19e20c46fd73dc483fd45aee7e8df0a88917ac404143b63787bd39f535bdbb96"
      }
    ]
    "# }

    test_example! { test_metadata_txs_by_label_cbor, Vec<MetadataTxsLabelCbor>, r#"
    [
      {
        "tx_hash": "257d75c8ddb0434e9b63e29ebb6241add2b835a307aa33aedba2effe09ed4ec8",
        "cbor_metadata": null
      },
      {
        "tx_hash": "e865f2cc01ca7381cf98dcdc4de07a5e8674b8ea16e6a18e3ed60c186fde2b9c",
        "cbor_metadata": null
      },
      {
        "tx_hash": "4237501da3cfdd53ade91e8911e764bd0699d88fd43b12f44a1f459b89bc91be",
        "cbor_metadata": "\\xa100a16b436f6d62696e6174696f6e8601010101010c"
      }
    ]
    "# }
}
