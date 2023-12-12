use crate::*;
use blockfrost_openapi::models::{
    mempool_content_inner::MempoolContentInner, mempool_tx_content::MempoolTxContent,
};

impl BlockfrostAPI {
    pub async fn mempool(
        &self,
        pagination: Pagination,
    ) -> BlockfrostResult<Vec<MempoolContentInner>> {
        self.call_paged_endpoint("/mempool", pagination).await
    }

    pub async fn mempool_hash(&self, hash: &str) -> BlockfrostResult<MempoolTxContent> {
        self.call_endpoint(format!("/mempool/{}", hash).as_str())
            .await
    }
    pub async fn mempool_addresses_address(
        &self,
        address: &str,
        pagination: Pagination,
    ) -> BlockfrostResult<Vec<MempoolContentInner>> {
        self.call_paged_endpoint(
            format!("/mempool/addresses/{}", address).as_str(),
            pagination,
        )
        .await
    }
}
