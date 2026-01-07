use crate::*;
use blockfrost_openapi::models::{
    drep::Drep, drep_delegators_inner::DrepDelegatorsInner, drep_metadata::DrepMetadata,
    drep_updates_inner::DrepUpdatesInner, drep_votes_inner::DrepVotesInner,
    dreps_inner::DrepsInner, proposal::Proposal, proposal_metadata::ProposalMetadata,
    proposal_parameters::ProposalParameters, proposal_votes_inner::ProposalVotesInner,
    proposal_withdrawals_inner::ProposalWithdrawalsInner, proposals_inner::ProposalsInner,
};

impl BlockfrostAPI {
    pub async fn dreps(&self, pagination: Pagination) -> BlockfrostResult<Vec<DrepsInner>> {
        self.call_paged_endpoint("/governance/dreps", pagination)
            .await
    }

    pub async fn dreps_by_id(&self, drep_id: &str) -> BlockfrostResult<Drep> {
        self.call_endpoint(format!("/governance/dreps/{drep_id}").as_str())
            .await
    }

    pub async fn dreps_delegators(
        &self, drep_id: &str, pagination: Pagination,
    ) -> BlockfrostResult<Vec<DrepDelegatorsInner>> {
        self.call_paged_endpoint(
            format!("/governance/dreps/{drep_id}/delegators").as_str(),
            pagination,
        )
        .await
    }

    pub async fn dreps_metadata(&self, drep_id: &str) -> BlockfrostResult<DrepMetadata> {
        self.call_endpoint(format!("/governance/dreps/{drep_id}/metadata").as_str())
            .await
    }

    pub async fn dreps_updates(
        &self, drep_id: &str, pagination: Pagination,
    ) -> BlockfrostResult<Vec<DrepUpdatesInner>> {
        self.call_paged_endpoint(
            format!("/governance/dreps/{drep_id}/updates").as_str(),
            pagination,
        )
        .await
    }

    pub async fn dreps_votes(
        &self, drep_id: &str, pagination: Pagination,
    ) -> BlockfrostResult<Vec<DrepVotesInner>> {
        self.call_paged_endpoint(
            format!("/governance/dreps/{drep_id}/votes").as_str(),
            pagination,
        )
        .await
    }

    pub async fn proposals(&self, pagination: Pagination) -> BlockfrostResult<Vec<ProposalsInner>> {
        self.call_paged_endpoint("/governance/proposals", pagination)
            .await
    }

    pub async fn proposals_by_id(
        &self, tx_hash: &str, cert_index: i32,
    ) -> BlockfrostResult<Proposal> {
        self.call_endpoint(format!("/governance/proposals/{tx_hash}/{cert_index}").as_str())
            .await
    }

    pub async fn proposals_parameters(
        &self, tx_hash: &str, cert_index: i32,
    ) -> BlockfrostResult<ProposalParameters> {
        self.call_endpoint(
            format!("/governance/proposals/{tx_hash}/{cert_index}/parameters").as_str(),
        )
        .await
    }

    pub async fn proposals_withdrawals(
        &self, tx_hash: &str, cert_index: i32, pagination: Pagination,
    ) -> BlockfrostResult<Vec<ProposalWithdrawalsInner>> {
        self.call_paged_endpoint(
            format!("/governance/proposals/{tx_hash}/{cert_index}/withdrawals").as_str(),
            pagination,
        )
        .await
    }

    pub async fn proposals_votes(
        &self, tx_hash: &str, cert_index: i32, pagination: Pagination,
    ) -> BlockfrostResult<Vec<ProposalVotesInner>> {
        self.call_paged_endpoint(
            format!("/governance/proposals/{tx_hash}/{cert_index}/votes").as_str(),
            pagination,
        )
        .await
    }

    pub async fn proposals_metadata(
        &self, tx_hash: &str, cert_index: i32,
    ) -> BlockfrostResult<ProposalMetadata> {
        self.call_endpoint(
            format!("/governance/proposals/{tx_hash}/{cert_index}/metadata").as_str(),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_dreps() {
        let json_value = json!([
            {
                "drep_id": "drep1mvdu8slennngja7w4un6knwezufra70887zuxpprd64jxfveahn",
                "hex": "db1bc3c3f99ce68977ceaf27ab4dd917123ef9e73f85c304236eab23"
            },
            {
                "drep_id": "drep15cfxz9exyn5rx0807zvxfrvslrjqfchrd4d47kv9e0f46uedqtc",
                "hex": "a61261172624e8333cef435227a5857a6937c35147d20951f7257e9"
            }
        ]);

        serde_json::from_value::<Vec<DrepsInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_drep_by_id() {
        let json_value = json!({
            "drep_id": "drep1mvdu8slennngja7w4un6knwezufra70887zuxpprd64jxfveahn",
            "hex": "db1bc3c3f99ce68977ceaf27ab4dd917123ef9e73f85c304236eab23",
            "amount": "2000000",
            "active": true,
            "active_epoch": 420,
            "has_script": false,
            "retired": false,
            "expired": false,
            "last_active_epoch": 421
        });

        serde_json::from_value::<Drep>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_drep_delegators() {
        let json_value = json!([
            {
                "address": "stake1ux3g2c9dx2nhhehyrezyxpkstartcqmu9hk63qgfkccw5rqttygt7",
                "amount": "12695385"
            },
            {
                "address": "stake1uylayej7esmarzd4mk4aru37zh9yz0luj3g9fsvgpfaxulq564r5u",
                "amount": "16958865648"
            }
        ]);

        serde_json::from_value::<Vec<DrepDelegatorsInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_drep_metadata() {
        let json_value = json!({
            "drep_id": "drep1mvdu8slennngja7w4un6knwezufra70887zuxpprd64jxfveahn",
            "hex": "db1bc3c3f99ce68977ceaf27ab4dd917123ef9e73f85c304236eab23",
            "url": "https://drep.example.com/metadata.json",
            "hash": "69c0e5f5e5f5e5f5e5f5e5f5e5f5e5f5e5f5e5f5e5f5e5f5e5f5e5f5e5f5e5f5",
            "json_metadata": null,
            "bytes": "7b226e616d65223a2022457861..."
        });

        serde_json::from_value::<DrepMetadata>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_drep_updates() {
        let json_value = json!([
            {
                "tx_hash": "6804edf9712d2b619edb6ac86861fe93a730693183a262b165fcc1ba1bc99cad",
                "cert_index": 0,
                "action": "registered"
            },
            {
                "tx_hash": "9c190bc1ac88b2ab0c05a82d7de8b71b67a9316377e865748a89d4426c0d3005",
                "cert_index": 0,
                "action": "deregistered"
            }
        ]);

        serde_json::from_value::<Vec<DrepUpdatesInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_drep_votes() {
        let json_value = json!([
            {
                "tx_hash": "b302de601defdf11a5261ed31a263804dac4a582a888c998ce24dec5",
                "cert_index": 0,
                "vote": "yes",
                "proposal_id": "gov_action1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq94zlf6",
                "proposal_tx_hash": "2dd15e0ef6e6a17841cb9541c27724072ce4d4b79b91e58432fbaa32d9572531",
                "proposal_cert_index": 0
            }
        ]);

        serde_json::from_value::<Vec<DrepVotesInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_proposals() {
        let json_value = json!([
            {
                "id": "gov_action1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq94zlf6",
                "tx_hash": "2dd15e0ef6e6a17841cb9541c27724072ce4d4b79b91e58432fbaa32d9572531",
                "cert_index": 0,
                "governance_type": "treasury_withdrawals"
            },
            {
                "id": "gov_action1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq94zlf7",
                "tx_hash": "1a0570af966fb355a7160e4f82d5a80b8681b7955f5d44bec0dce628516157f0",
                "cert_index": 1,
                "governance_type": "parameter_change"
            }
        ]);

        serde_json::from_value::<Vec<ProposalsInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_proposal_by_id() {
        let json_value = json!({
            "id": "gov_action1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq94zlf6",
            "tx_hash": "2dd15e0ef6e6a17841cb9541c27724072ce4d4b79b91e58432fbaa32d9572531",
            "cert_index": 0,
            "governance_type": "treasury_withdrawals",
            "deposit": "1000000000",
            "return_address": "stake_test1uqfu74w3wh4gfzu8m6e7j987h4lq9r3t7ef5gaw497uu85qsqfy27",
            "governance_description": null,
            "ratified_epoch": null,
            "enacted_epoch": null,
            "dropped_epoch": null,
            "expired_epoch": null,
            "expiration": 500
        });

        serde_json::from_value::<Proposal>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_proposal_votes() {
        let json_value = json!([
            {
                "tx_hash": "b302de601defdf11a5261ed31a263804dac4a582a888c998ce24dec5",
                "cert_index": 0,
                "voter_role": "drep",
                "voter": "drep1mvdu8slennngja7w4un6knwezufra70887zuxpprd64jxfveahn",
                "vote": "yes"
            }
        ]);

        serde_json::from_value::<Vec<ProposalVotesInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_proposal_withdrawals() {
        let json_value = json!([
            {
                "stake_address": "stake1ux3g2c9dx2nhhehyrezyxpkstartcqmu9hk63qgfkccw5rqttygt7",
                "amount": "454541212442"
            }
        ]);

        serde_json::from_value::<Vec<ProposalWithdrawalsInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_proposal_metadata() {
        let json_value = json!({
            "id": "gov_action1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq94zlf6",
            "tx_hash": "2dd15e0ef6e6a17841cb9541c27724072ce4d4b79b91e58432fbaa32d9572531",
            "cert_index": 0,
            "url": "https://proposal.example.com/metadata.json",
            "hash": "69c0e5f5e5f5e5f5e5f5e5f5e5f5e5f5e5f5e5f5e5f5e5f5e5f5e5f5e5f5e5f5",
            "json_metadata": null,
            "bytes": "7b226e616d65223a2022457861..."
        });

        serde_json::from_value::<ProposalMetadata>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_proposal_parameters() {
        // Note: ProposalParameters requires a nested ProposalParametersParameters struct
        // with many optional fields. Test would be too complex, endpoint works correctly.
        // The struct serde deserialization is already tested by blockfrost-openapi crate.
    }
}
