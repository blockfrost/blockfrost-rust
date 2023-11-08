use crate::*;
use blockfrost_openapi::models::{
    tx_metadata_label_cbor_inner::TxMetadataLabelCborInner,
    tx_metadata_label_json_inner::TxMetadataLabelJsonInner,
    tx_metadata_labels_inner::TxMetadataLabelsInner,
};

impl BlockfrostApi {
    /// List of all used transaction metadata labels.
    pub async fn metadata_txs_labels(
        &self,
        pagination: Pagination,
    ) -> BlockfrostResult<Vec<TxMetadataLabelsInner>> {
        self.call_paged_endpoint("/metadata/txs/labels", pagination)
            .await
    }

    /// Transaction metadata per label (json).
    pub async fn metadata_txs_by_label(
        &self,
        label: &str,
        pagination: Pagination,
    ) -> BlockfrostResult<Vec<TxMetadataLabelJsonInner>> {
        self.call_paged_endpoint(
            format!("/metadata/txs/labels/{}", label).as_str(),
            pagination,
        )
        .await
    }

    /// Transaction metadata per label (cbor).
    pub async fn metadata_txs_by_label_cbor(
        &self,
        label: &str,
        pagination: Pagination,
    ) -> BlockfrostResult<Vec<TxMetadataLabelCborInner>> {
        self.call_paged_endpoint(
            format!("/metadata/txs/labels/{}/cbor", label).as_str(),
            pagination,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use blockfrost_openapi::models::{
        tx_metadata_label_cbor_inner::TxMetadataLabelCborInner,
        tx_metadata_label_json_inner::TxMetadataLabelJsonInner,
        tx_metadata_labels_inner::TxMetadataLabelsInner,
    };
    use serde_json::json;

    #[test]
    fn test_metadata_txs_labels() {
        let json_value = json!([
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
        ]);

        serde_json::from_value::<Vec<TxMetadataLabelsInner>>(json_value).unwrap();
    }

    #[test]
    fn test_metadata_txs_by_label() {
        let json_value = json!([
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
            "tx_hash": "19e20c46fd73dc483fd45aee7e8df0a88917ac404143b63787bd39f535bdbb96",
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
          }
        ]);

        serde_json::from_value::<Vec<TxMetadataLabelJsonInner>>(json_value).unwrap();
    }

    #[test]
    fn test_test_metadata_txs_labels() {
        let json_value = json!([
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
        ]);

        serde_json::from_value::<Vec<TxMetadataLabelsInner>>(json_value).unwrap();
    }

    #[test]
    fn test_metadata_txs_by_label_cbor() {
        let json_value = json!([
          {
            "tx_hash": "257d75c8ddb0434e9b63e29ebb6241add2b835a307aa33aedba2effe09ed4ec8",
            "cbor_metadata": null,
            "metadata": null
          },
          {
            "tx_hash": "e865f2cc01ca7381cf98dcdc4de07a5e8674b8ea16e6a18e3ed60c186fde2b9c",
            "cbor_metadata": null,
            "metadata": null
          },
          {
            "tx_hash": "4237501da3cfdd53ade91e8911e764bd0699d88fd43b12f44a1f459b89bc91be",
            "cbor_metadata": "\\xa100a16b436f6d62696e6174696f6e8601010101010c",
            "metadata": null
          }
        ]);

        serde_json::from_value::<Vec<TxMetadataLabelCborInner>>(json_value).unwrap();
    }
}
