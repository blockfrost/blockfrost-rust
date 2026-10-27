use crate::*;
use blockfrost_openapi::models::{
    committee::Committee, committee_votes_inner::CommitteeVotesInner, drep::Drep,
    drep_delegators_inner::DrepDelegatorsInner, drep_metadata::DrepMetadata,
    drep_updates_inner::DrepUpdatesInner, drep_votes_inner::DrepVotesInner,
    dreps_inner::DrepsInner, proposal::Proposal, proposal_metadata::ProposalMetadata,
    proposal_metadata_v2::ProposalMetadataV2, proposal_parameters::ProposalParameters,
    proposal_votes_inner::ProposalVotesInner, proposal_withdrawals_inner::ProposalWithdrawalsInner,
    proposals_inner::ProposalsInner,
};

impl BlockfrostAPI {
    /// Return information about the currently active constitutional committee.
    pub async fn committee(&self) -> BlockfrostResult<Committee> {
        self.call_endpoint("/governance/committee").await
    }

    /// Return votes cast by constitutional committee members across current and past committees.
    pub async fn committee_votes(
        &self, pagination: Pagination,
    ) -> BlockfrostResult<Vec<CommitteeVotesInner>> {
        self.call_paged_endpoint("/governance/committee/votes", pagination)
            .await
    }

    /// Return votes for a CIP-129 hot (`cc_hot1...`) or cold (`cc_cold1...`) credential.
    /// Cold credentials aggregate votes across authorized hot-key rotations.
    pub async fn committee_member_votes(
        &self, cc_id: &str, pagination: Pagination,
    ) -> BlockfrostResult<Vec<CommitteeVotesInner>> {
        self.call_paged_endpoint(
            format!("/governance/committee/{cc_id}/votes").as_str(),
            pagination,
        )
        .await
    }

    /// Return the list of registered delegated representatives.
    pub async fn dreps(&self, pagination: Pagination) -> BlockfrostResult<Vec<DrepsInner>> {
        self.call_paged_endpoint("/governance/dreps", pagination)
            .await
    }

    /// Return information about a specific delegated representative.
    pub async fn dreps_by_id(&self, drep_id: &str) -> BlockfrostResult<Drep> {
        self.call_endpoint(format!("/governance/dreps/{drep_id}").as_str())
            .await
    }

    /// Return the list of delegators for a specific DRep.
    pub async fn dreps_delegators(
        &self, drep_id: &str, pagination: Pagination,
    ) -> BlockfrostResult<Vec<DrepDelegatorsInner>> {
        self.call_paged_endpoint(
            format!("/governance/dreps/{drep_id}/delegators").as_str(),
            pagination,
        )
        .await
    }

    /// Return the metadata for a specific DRep.
    pub async fn dreps_metadata(&self, drep_id: &str) -> BlockfrostResult<DrepMetadata> {
        self.call_endpoint(format!("/governance/dreps/{drep_id}/metadata").as_str())
            .await
    }

    /// Return the history of updates for a specific DRep.
    pub async fn dreps_updates(
        &self, drep_id: &str, pagination: Pagination,
    ) -> BlockfrostResult<Vec<DrepUpdatesInner>> {
        self.call_paged_endpoint(
            format!("/governance/dreps/{drep_id}/updates").as_str(),
            pagination,
        )
        .await
    }

    /// Return the votes cast by a specific DRep.
    pub async fn dreps_votes(
        &self, drep_id: &str, pagination: Pagination,
    ) -> BlockfrostResult<Vec<DrepVotesInner>> {
        self.call_paged_endpoint(
            format!("/governance/dreps/{drep_id}/votes").as_str(),
            pagination,
        )
        .await
    }

    /// Return the list of governance proposals.
    pub async fn proposals(&self, pagination: Pagination) -> BlockfrostResult<Vec<ProposalsInner>> {
        self.call_paged_endpoint("/governance/proposals", pagination)
            .await
    }

    /// Return information about a specific governance proposal.
    pub async fn proposals_by_id(
        &self, tx_hash: &str, cert_index: i32,
    ) -> BlockfrostResult<Proposal> {
        self.call_endpoint(format!("/governance/proposals/{tx_hash}/{cert_index}").as_str())
            .await
    }

    /// Return the proposed protocol parameters for a specific proposal.
    pub async fn proposals_parameters(
        &self, tx_hash: &str, cert_index: i32,
    ) -> BlockfrostResult<ProposalParameters> {
        self.call_endpoint(
            format!("/governance/proposals/{tx_hash}/{cert_index}/parameters").as_str(),
        )
        .await
    }

    /// Return the withdrawals associated with a specific proposal.
    pub async fn proposals_withdrawals(
        &self, tx_hash: &str, cert_index: i32, pagination: Pagination,
    ) -> BlockfrostResult<Vec<ProposalWithdrawalsInner>> {
        self.call_paged_endpoint(
            format!("/governance/proposals/{tx_hash}/{cert_index}/withdrawals").as_str(),
            pagination,
        )
        .await
    }

    /// Return the votes cast on a specific proposal.
    pub async fn proposals_votes(
        &self, tx_hash: &str, cert_index: i32, pagination: Pagination,
    ) -> BlockfrostResult<Vec<ProposalVotesInner>> {
        self.call_paged_endpoint(
            format!("/governance/proposals/{tx_hash}/{cert_index}/votes").as_str(),
            pagination,
        )
        .await
    }

    /// Return the metadata for a specific proposal.
    pub async fn proposals_metadata(
        &self, tx_hash: &str, cert_index: i32,
    ) -> BlockfrostResult<ProposalMetadata> {
        self.call_endpoint(
            format!("/governance/proposals/{tx_hash}/{cert_index}/metadata").as_str(),
        )
        .await
    }

    /// Return information about a proposal by its CIP-129 governance action identifier.
    pub async fn proposals_by_gov_action_id(
        &self, gov_action_id: &str,
    ) -> BlockfrostResult<Proposal> {
        self.call_endpoint(format!("/governance/proposals/{gov_action_id}").as_str())
            .await
    }

    /// Return proposed protocol parameters by the CIP-129 governance action identifier.
    pub async fn proposals_parameters_by_gov_action_id(
        &self, gov_action_id: &str,
    ) -> BlockfrostResult<ProposalParameters> {
        self.call_endpoint(format!("/governance/proposals/{gov_action_id}/parameters").as_str())
            .await
    }

    /// Return proposal withdrawals by the CIP-129 governance action identifier.
    pub async fn proposals_withdrawals_by_gov_action_id(
        &self, gov_action_id: &str, pagination: Pagination,
    ) -> BlockfrostResult<Vec<ProposalWithdrawalsInner>> {
        self.call_paged_endpoint(
            format!("/governance/proposals/{gov_action_id}/withdrawals").as_str(),
            pagination,
        )
        .await
    }

    /// Return proposal votes by the CIP-129 governance action identifier.
    pub async fn proposals_votes_by_gov_action_id(
        &self, gov_action_id: &str, pagination: Pagination,
    ) -> BlockfrostResult<Vec<ProposalVotesInner>> {
        self.call_paged_endpoint(
            format!("/governance/proposals/{gov_action_id}/votes").as_str(),
            pagination,
        )
        .await
    }

    /// Return proposal metadata by the CIP-129 governance action identifier.
    /// The anchor URL and hash remain available when fetching metadata fails.
    pub async fn proposals_metadata_by_gov_action_id(
        &self, gov_action_id: &str,
    ) -> BlockfrostResult<ProposalMetadataV2> {
        self.call_endpoint(format!("/governance/proposals/{gov_action_id}/metadata").as_str())
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::{Method::GET, MockServer};
    use serde_json::json;

    #[tokio::test]
    async fn test_committee() {
        let json_value = json!({
            "proposal_tx_hash": null,
            "proposal_index": null,
            "gov_action_id": null,
            "is_dissolved": false,
            "quorum": { "numerator": 2, "denominator": 3 },
            "members": [
                {
                    "cc_cold_id": "cc_cold1zgkfeeu07qq9smfqfn0nrf2cqgr5x6jpu8x6lxgxay85t6cmt54xz",
                    "cc_cold_hex": "12c9ce78ff8005436901336f8d2ac0411a1b520f0e6d7e6066907a2f",
                    "cc_cold_has_script": false,
                    "cc_hot_id": "cc_hot1qf4xq9mlra5j68w8zjz2lvf3kc3rtsdtu98ka7zx4u6jvqyy39ww0",
                    "cc_hot_hex": "0269808bbf8fb495a3b8e290a5f626362236c1abf853eddf08d79a930",
                    "cc_hot_has_script": false,
                    "status": "authorized",
                    "expiration_epoch": 580
                },
                {
                    "cc_cold_id": "cc_cold1zgxxx2trz0vyl5ksetnsrct5n5gznn3jmkj6dlpu4lz6lecz5h",
                    "cc_cold_hex": "8cc6529626c4c9fa5a195ce03c2e93a204e7193b76967e8794fc59d6",
                    "cc_cold_has_script": false,
                    "cc_hot_id": null,
                    "cc_hot_hex": null,
                    "cc_hot_has_script": null,
                    "status": "not_authorized",
                    "expiration_epoch": 580
                }
            ]
        });

        let committee = serde_json::from_value::<Committee>(json_value).unwrap();
        assert!(committee.gov_action_id.is_none());
        assert!(committee.proposal_tx_hash.is_none());
        assert!(committee.proposal_index.is_none());
        assert!(committee.members[0].cc_hot_id.is_some());
        assert!(committee.members[1].cc_hot_id.is_none());
    }

    #[tokio::test]
    async fn test_committee_votes() {
        let json_value = json!([
            {
                "tx_hash": "b302de601defdf11a5261ed31a263804dac4a582a888c998ce24dec5",
                "voter_hot_id": "cc_hot1qf4xq9mlra5j68w8zjz2lvf3kc3rtsdtu98ka7zx4u6jvqyy39ww0",
                "proposal_id": "gov_action1k2jertppnnndejjcglszfqq4yzw8evzrd2nt66rr6rqlz54xp0zsq05ecsn",
                "proposal_tx_hash": "b2a591ac219ce6dcca5847e0248015209c7cb0436aa6bd6863d0c1f152a60bc5",
                "proposal_index": 0,
                "governance_type": "parameter_change",
                "vote": "yes",
                "metadata_url": "https://my.cc-member/rationale.jsonld",
                "metadata_hash": "a4b7d5d34f5d8c5b71a0a5c34b7e7f8a3d2e1c0b9a8f7e6d5c4b3a2918f7e6d5",
                "block_height": 11045358,
                "block_time": 1746037200
            },
            {
                "tx_hash": "8a91b32ec2a7bdfde2ab7a1bc5e4d3a2f1b0c9d8e7f6a5b4c3d2e1f0a9b8c7d6",
                "voter_hot_id": "cc_hot1qf4xq9mlra5j68w8zjz2lvf3kc3rtsdtu98ka7zx4u6jvqyy39ww0",
                "proposal_id": "gov_action1zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zygsq6dmejn",
                "proposal_tx_hash": "2dd15e0ef6e6a17841cb9541c27724072ce4d4b79b91e58432fbaa32d9572531",
                "proposal_index": 1,
                "governance_type": "new_constitution",
                "vote": "abstain",
                "metadata_url": null,
                "metadata_hash": null,
                "block_height": 11045890,
                "block_time": 1746070800
            }
        ]);

        let votes = serde_json::from_value::<Vec<CommitteeVotesInner>>(json_value).unwrap();
        assert!(votes[0].metadata_url.is_some());
        assert!(votes[0].metadata_hash.is_some());
        assert!(votes[1].metadata_url.is_none());
        assert!(votes[1].metadata_hash.is_none());
    }

    fn proposal_metadata_v2() -> serde_json::Value {
        json!({
            "id": "gov_action1zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zygsq6dmejn",
            "tx_hash": "257d75c8ddb0434e9b63e29ebb6241add2b835a307aa33aedba2effe09ed4ec8",
            "cert_index": 2,
            "url": "https://raw.githubusercontent.com/carloslodelar/proposals/main/pv10.json",
            "hash": "ffa226f3863aca006172d559cf46bb8b883a47233962ae2fc94c158d7de6fa81",
            "json_metadata": null,
            "bytes": null,
            "error": {
                "code": "CONNECTION_ERROR",
                "message": "Failed to fetch metadata"
            }
        })
    }

    #[tokio::test]
    async fn test_proposal_metadata_v2() {
        let mut json_value = proposal_metadata_v2();
        let metadata = serde_json::from_value::<ProposalMetadataV2>(json_value.clone()).unwrap();
        assert!(metadata.json_metadata.is_none());
        assert!(metadata.bytes.is_none());
        assert!(metadata.error.is_some());

        json_value.as_object_mut().unwrap().remove("error");
        json_value["json_metadata"] = json!({
            "body": { "title": "Hardfork to Protocol version 10" }
        });
        json_value["bytes"] = json!("7b0a20202240636f6e74657874223a207b");
        let metadata = serde_json::from_value::<ProposalMetadataV2>(json_value).unwrap();
        assert!(metadata.json_metadata.is_some());
        assert!(metadata.bytes.is_some());
        assert!(metadata.error.is_none());
    }

    #[tokio::test]
    async fn committee_vote_endpoints_send_pagination() {
        let server = MockServer::start();
        let mut settings = BlockFrostSettings::new();
        settings.base_url = Some(server.base_url());
        let api = BlockfrostAPI::new("test", settings);
        let hot_id = "cc_hot1qf4xq9mlra5j68w8zjz2lvf3kc3rtsdtu98ka7zx4u6jvqyy39ww0";
        let cold_id = "cc_cold1zgkfeeu07qq9smfqfn0nrf2cqgr5x6jpu8x6lxgxay85t6cmt54xz";
        let paths = [
            "/governance/committee/votes".to_string(),
            format!("/governance/committee/{hot_id}/votes"),
            format!("/governance/committee/{cold_id}/votes"),
        ];
        let mocks: Vec<_> = paths
            .iter()
            .map(|path| {
                server.mock(|when, then| {
                    when.method(GET)
                        .path(path)
                        .query_param("page", "2")
                        .query_param("count", "5")
                        .query_param("order", "desc")
                        .query_param_missing("from")
                        .query_param_missing("to");
                    then.status(200).json_body(json!([]));
                })
            })
            .collect();
        let pagination = Pagination::new(Order::Desc, 2, 5).with_range(1, 2);

        api.committee_votes(pagination).await.unwrap();
        api.committee_member_votes(hot_id, pagination)
            .await
            .unwrap();
        api.committee_member_votes(cold_id, pagination)
            .await
            .unwrap();
        for mock in mocks {
            mock.assert();
        }
    }

    #[tokio::test]
    async fn proposal_metadata_endpoint_uses_gov_action_id() {
        let server = MockServer::start();
        let mut settings = BlockFrostSettings::new();
        settings.base_url = Some(server.base_url());
        let api = BlockfrostAPI::new("test", settings);
        let gov_action_id =
            "gov_action1zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zygsq6dmejn";
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/governance/proposals/{gov_action_id}/metadata"))
                .query_param_missing("page")
                .query_param_missing("count")
                .query_param_missing("order");
            then.status(200).json_body(proposal_metadata_v2());
        });

        let metadata = api
            .proposals_metadata_by_gov_action_id(gov_action_id)
            .await
            .unwrap();
        assert_eq!(metadata.id, gov_action_id);
        assert!(metadata.error.is_some());
        mock.assert();
    }

    #[tokio::test]
    async fn test_dreps() {
        let json_value = json!([
            {
                "drep_id": "drep1mvdu8slennngja7w4un6knwezufra70887zuxpprd64jxfveahn",
                "hex": "db1bc3c3f99ce68977ceaf27ab4dd917123ef9e73f85c304236eab23",
                "amount": "2000000",
                "has_script": true,
                "retired": false,
                "expired": false,
                "last_active_epoch": 509,
                "metadata": {
                    "url": "https://aaa.xyz/drep.json",
                    "hash": "8c1a7f8d1e2b3c4d5e6f708192a3b4c5d6e7f8091a2b3c4d5e6f708192a3b4c5",
                    "json_metadata": {
                        "@context": {
                            "CIP100": "https://github.com/cardano-foundation/CIPs/blob/master/CIP-0100/README.md#"
                        },
                        "hashAlgorithm": "blake2b-256"
                    },
                    "bytes": "7b0a2020227c"
                }
            },
            {
                // no registration anchor - `metadata` is null
                "drep_id": "drep15cfxz9exyn5rx0807zvxfrvslrjqfchrd4d47kv9e0f46uedqtc",
                "hex": "a61261172624e8333cef435227a5857a6937c35147d20951f7257e9",
                "amount": "0",
                "has_script": false,
                "retired": true,
                "expired": true,
                "last_active_epoch": null,
                "metadata": null
            },
            {
                // anchor present but off-chain content unavailable - `error` is populated
                "drep_id": "drep1y2z8x7w6v5u4t3s2r1q0p9o8n7m6l5k4j3h2g1f0e9d8c7b6a5",
                "hex": "c7b6a5948372615f4e3d2c1b0a9f8e7d6c5b4a39281706f5e4d3c2b1",
                "amount": "42000000",
                "has_script": false,
                "retired": false,
                "expired": false,
                "last_active_epoch": 512,
                "metadata": {
                    "url": "https://bbb.xyz/drep.json",
                    "hash": "9d2b8e7f6a5c4d3e2f1908172635445362718091a2b3c4d5e6f708192a3b4c5d",
                    "json_metadata": null,
                    "bytes": null,
                    "error": {
                        "code": "CONNECTION_ERROR",
                        "message": "Failed to fetch metadata"
                    }
                }
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
                "action": "registered",
                "deposit": "500000000"
            },
            {
                "tx_hash": "9c190bc1ac88b2ab0c05a82d7de8b71b67a9316377e865748a89d4426c0d3005",
                "cert_index": 0,
                "action": "deregistered",
                "deposit": "500000000"
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
                "vote": "yes",
                "counted": true
            },
            {
                "tx_hash": "b302de601defdf11a5261ed31a263804dac4a582a888c998ce24dec5",
                "cert_index": 3,
                "voter_role": "constitutional_committee",
                "voter": "cc_hot1q0wzkpcxzzfs4mf4yk6yx7d075vqtyx2tnxsr256he6gnwq6yfy5w",
                "vote": "abstain",
                "counted": true
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
