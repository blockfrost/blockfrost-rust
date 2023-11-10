use crate::*;
use blockfrost_openapi::models::{
    pool::Pool, pool_delegators_inner::PoolDelegatorsInner, pool_history_inner::PoolHistoryInner,
    pool_list_retire_inner::PoolListRetireInner, pool_metadata::PoolMetadata,
    pool_updates_inner::PoolUpdatesInner,
    tx_content_pool_certs_inner_relays_inner::TxContentPoolCertsInnerRelaysInner,
};

impl BlockfrostAPI {
    pub async fn derive_address(
        &self,
        xpub: &str,
        role: &str,
        index: &str,
    ) -> BlockfrostResult<Derive> {
        self.call_endpoint(format!("/utils/addresses/xpub/{}/{}/{}", xpub, role, index).as_str())
            .await
    }
}
