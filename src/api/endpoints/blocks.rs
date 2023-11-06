use crate::*;
use blockfrost_openapi::models::{
    block_content::BlockContent, block_content_addresses_inner::BlockContentAddressesInner,
};

impl BlockFrostApi {
    pub async fn blocks_latest(&self) -> Result<BlockContent> {
        self.call_endpoint("/blocks/latest").await
    }

    pub async fn blocks_by_id(&self, hash_or_number: &str) -> Result<BlockContent> {
        self.call_endpoint(format!("/blocks/{}", hash_or_number).as_str())
            .await
    }

    pub async fn blocks_slot(&self, slot_number: i64) -> Result<BlockContent> {
        self.call_endpoint(format!("/blocks/slot/{}", slot_number).as_str())
            .await
    }

    pub async fn blocks_by_epoch_and_slot(
        &self,
        epoch_number: i64,
        slot_number: i64,
    ) -> Result<BlockContent> {
        self.call_endpoint(format!("/blocks/epoch/{}/slot/{}", epoch_number, slot_number).as_str())
            .await
    }

    pub async fn blocks_latest_txs(&self, pagination: Option<Pagination>) -> Result<Vec<String>> {
        self.call_paged_endpoint("/blocks/latest/txs", pagination)
            .await
    }

    pub async fn blocks_next(
        &self,
        hash_or_number: &str,
        pagination: Option<Pagination>,
    ) -> Result<Vec<BlockContent>> {
        self.call_paged_endpoint(
            format!("/blocks/{}/next", hash_or_number).as_str(),
            pagination,
        )
        .await
    }

    pub async fn blocks_previous(
        &self,
        hash_or_number: &str,
        pagination: Option<Pagination>,
    ) -> Result<Vec<BlockContent>> {
        self.call_paged_endpoint(
            format!("/blocks/{}/previous", hash_or_number).as_str(),
            pagination,
        )
        .await
    }

    pub async fn blocks_txs(
        &self,
        hash_or_number: &str,
        pagination: Option<Pagination>,
    ) -> Result<Vec<String>> {
        self.call_paged_endpoint(
            format!("/blocks/{}/txs", hash_or_number).as_str(),
            pagination,
        )
        .await
    }

    pub async fn blocks_affected_addresses(
        &self,
        hash_or_number: &str,
        pagination: Option<Pagination>,
    ) -> Result<Vec<BlockContentAddressesInner>> {
        self.call_paged_endpoint(
            format!("/blocks/{}/addresses", hash_or_number).as_str(),
            pagination,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use blockfrost_openapi::models::block_content::BlockContent;
    use serde_json::json;

    #[tokio::test]
    async fn test_blocks_latest() {
        let json_value = json!({
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
        });

        serde_json::from_value::<BlockContent>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_blocks_latest_txs() {
        let json_value = json!([
            "8788591983aa73981fc92d6cddbbe643959f5a784e84b8bee0db15823f575a5b",
            "4eef6bb7755d8afbeac526b799f3e32a624691d166657e9d862aaeb66682c036",
            "52e748c4dec58b687b90b0b40d383b9fe1f24c1a833b7395cdf07dd67859f46f",
            "e8073fd5318ff43eca18a852527166aa8008bee9ee9e891f585612b7e4ba700b"
        ]);

        serde_json::from_value::<Vec<String>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_blocks_next() {
        let json_value = json!([{
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
        }]);

        serde_json::from_value::<BlockContent>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_blocks_txs() {
        let json_value = json!([
            "8788591983aa73981fc92d6cddbbe643959f5a784e84b8bee0db15823f575a5b",
            "4eef6bb7755d8afbeac526b799f3e32a624691d166657e9d862aaeb66682c036",
            "52e748c4dec58b687b90b0b40d383b9fe1f24c1a833b7395cdf07dd67859f46f",
            "e8073fd5318ff43eca18a852527166aa8008bee9ee9e891f585612b7e4ba700b"
        ]);

        serde_json::from_value::<Vec<String>>(json_value).unwrap();
    }
}
