use serde_json::Value as JsonValue;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// Use this module as an interface to export all types declared inside of endpoints/
//
// These are not used in here, just exporting
pub use crate::api::endpoints::*;

/// Integer used in other types.
///
/// A signed 128-bit integer can store up to 2^127, or 340282366920938463463374607431768211456.
///
/// This number contains 39 digits, the max supply of ADA is 45 billion (11 digits).
pub type Integer = i128;

/// Float used in other types.
pub type Float = f64;

/// Arbitrary JSON, for example protocol entropy, metadata, etc.
pub type ArbitraryJson = HashMap<String, JsonValue>;

/// Inner enum for [`PoolUpdate`] and [`AccountRegistration`].
///
/// Action in the certificate.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    Registered,
    Deregistered,
}
