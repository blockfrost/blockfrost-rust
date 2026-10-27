# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- Constitutional committee endpoints: `committee` (`GET /governance/committee`), `committee_votes`
  (`GET /governance/committee/votes`) and `committee_member_votes`
  (`GET /governance/committee/:cc_id/votes`, accepting a `cc_hot1...` or `cc_cold1...` credential)
- Proposal endpoints addressed by CIP-129 governance action id: `proposals_by_gov_action_id`,
  `proposals_parameters_by_gov_action_id`, `proposals_withdrawals_by_gov_action_id`,
  `proposals_votes_by_gov_action_id` and `proposals_metadata_by_gov_action_id` (the last one
  returns `ProposalMetadataV2`)
- `assets_utxos` for `GET /assets/:asset/utxos`, listing the unspent UTXOs that hold an asset. It
  accepts `from`/`to` block cursors (block heights only)

### Changed

- Bumped `blockfrost-openapi` to `0.1.93`. The generated models it re-exports changed:
  - `ProposalVotesInner` (returned by `proposals_votes`) gained the required `counted` field, and
    its `new` constructor takes it as an extra argument
  - Constitutional committee voters in proposal votes are CIP-129 `cc_hot1...` ids instead of
    raw hex
  - Redeemer `purpose` enums gained the Conway `vote` and `propose` purposes

## 1.3.0 - 2026-07-29

### Added

- `Pagination::with_from`, `Pagination::with_to` and `Pagination::with_range` for inclusive `from`/`to`
  block cursors, plus the `BlockCursor` type they take
- Cursors are only sent by the three endpoints that accept them (`accounts_transactions`,
  `addresses_transactions`, `assets_transactions`). Every other paginated endpoint silently drops
  them and returns an unfiltered result rather than an error, so keep cursors on a `Pagination`
  handed only to those three
- Cursors are preserved across pages when `fetch_all` is set
- `scripts_utxos` for `GET /scripts/:script_hash/utxos`, listing the UTXOs that hold a reference
  script

### Changed

- `Pagination` gained the public `from` and `to` fields. Endpoint signatures are unchanged and
  `Pagination` stays `Copy`, so only code constructing it via a struct literal needs updating
  (add `..Default::default()`); `Pagination::default()`, `::new()` and `::all()` are unaffected
- Bumped `blockfrost-openapi` to `0.1.91`. The generated models it re-exports changed:
  - `DrepsInner` (returned by `governance_dreps`) gained `amount`, `has_script`, `retired`,
    `expired`, `last_active_epoch` and `metadata`
  - `DrepMetadataError` was renamed to `DrepsInnerMetadataError`, affecting the `error` field of
    `DrepMetadata`, `PoolMetadata`, `ProposalMetadataV2`, `PoolListExtendedInnerMetadata` and
    `PoolsPoolIdMetadataGet200Response`

## 1.2.4 - 2026-05-28

### Added

- Re-export of `blockfrost-openapi` crate, so all generated types are accessible via `blockfrost::blockfrost_openapi::models::…` without a second dependency

### Changed

- Updated blockfrost-openapi to 0.1.88

## 1.2.3 - 2026-04-01

### Changed

- Updated blockfrost-openapi to 0.1.87
- Updated reqwest to 0.13.2

## 1.2.2 - 2026-03-09

### Changed

- Updated blockfrost-openapi to 0.1.86

## 1.2.1 - 2026-01-08

### Added

- Documentation comments for all public API functions

## 1.2.0 - 2026-01-07

### Updated

- dependencies

### Added

- `dreps`
- `dreps_by_id`
- `dreps_delegators`
- `dreps_metadata`
- `dreps_updates`
- `dreps_votes`
- `proposals`
- `proposals_by_id`
- `proposals_parameters`
- `proposals_withdrawals`
- `proposals_votes`
- `proposals_metadata`
- `accounts_utxos`
- `accounts_utxos_asset`
- `accounts_transactions`
- `blocks_latest_txs_cbor`
- `blocks_txs_cbor`
- `pools_votes`
- `transactions_required_signers`
- `utils_tx_evaluate_utxos`

### Fixed

- `transactions_pool_updates` now returns correct type `TxContentPoolCertsInner`
- `scripts_datum_hash_cbor` now uses correct url path `/scripts/datum/{hash}/cbor`
- `scripts_hash_json` now returns `ScriptJson`
- `scripts_hash_cbor` now returns `ScriptCbor`

### Changed

- `epochs_blocks` and `epochs_blocks_by_pool` now support pagination

## 1.1.0 - 2025-08-05

## Changed

- Updated pagination with tests
- Updated dependencies
- Updated openapi

## 1.0.6 - 2025-07-03

## Changed

- blockfrost-openapi 0.1.75

## Added

- Posibility to change the base url

## 1.0.5 - 2025-11-20

## Fixed

- Readme version

## 1.0.4 - 2023-11-20

## Fixed

- Off-by-one when using Pagination::all(). Thanks [SupernaviX](https://github.com/SupernaviX) 💖

## Added

- Transactions cbor by @SupernaviX in https://github.com/blockfrost/blockfrost-rust/pull/51

## Changed

- Rate limit handling by @SupernaviX in https://github.com/blockfrost/blockfrost-rust/pull/52

## 1.0.3 - 2023-10-05

### Changed

- Updated dependencies to latest versions.

## 1.0.2 - 2023-05-31

### Added

- Added `derive_address` and `utils_tx_evaluate` endpoints.

## 1.0.1 - 2023-01-01

### Fixed

- Return types for asset and tx endpoints

## 1.0.0 - 2023-12-18

### Changed

- Implemented pagination for each endpoint.
- Integrated GitHub Continuous Integration for automated testing and deployment.
- Applied Cargo Clippy and rustfmt for code linting and formatting.
- Removed macros.
- Expanded test coverage with additional tests.
- Enhanced data fetching methods.
- Conducted code cleanup and refactoring.
- Refactored settings for improved clarity and efficiency.
- Updated data structures and schemas to the latest standards.
- Provided more comprehensive examples for better user understanding.
- Enhanced unit tests for more robust code testing.
- Implemented additional linting measures for code quality assurance.
- ENDPOINTS.md now contains a table of implemented endpoints.
- Added missing endpoints:
  - `accounts_addresses_total`
  - `addresses_extended`
  - `addresses_utxos_asset`
  - `network_eras`
  - `mempool`
  - `mempool_hash`
  - `mempool_addresses_address`
  - `scripts_hash_json`
  - `scripts_hash_cbor`
  - `scripts_datum_hash`
  - `scripts_datum_hash_cbor`
  - `utils_tx_evaluate`
  - `utils_tx_evaluate_utxos`

## 0.2.1 - 2023-05-02

### Changed

`use_previewnet` -> `use_preview`
`use_preprodnet` -> `use_preprod`

### Fixed

redemeers -> redeemers
