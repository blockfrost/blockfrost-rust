//! Definitions for common types returned in requests.

use serde::{Deserialize, Serialize};

// Use this module as an interface to export all types declared inside of endpoints/
//
// These are not used in here, just exporting
pub use crate::{
    api::endpoints::*,
    ipfs::{IpfsAdd, IpfsPinList, IpfsPinState, IpfsPinUpdate},
};

/// Inner member of [`Address`], [`AddressTotal`], [`AddressUtxo`] and [`Transaction`].
///
/// # Format:
///
/// The `unit` String can be "lovelace" or other, in the latter case, the String will be made of
/// a concatenation of the asset `policy_id` and hex-encoded `asset_name`.
///
/// # Example:
///
/// ```
/// # use blockfrost::Amount;
/// let unit = "lovelace".to_string();
/// let quantity = "700".to_string();
///
/// // Amount: 700 lovelaces
/// let amount = Amount { unit, quantity };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Amount {
    /// The unit of the value.
    ///
    /// Format: "Lovelace" or concatenation of asset `policy_id` and hex-encoded `asset_name`.
    pub unit: String,
    /// The quantity of the unit.
    pub quantity: String,
}

/// Enum for any possible JSON value.
///
/// Declared as the following:
///
/// ```no_run
/// # use f64 as Number;
/// # use std::collections::HashMap as Map;
/// pub enum Value {
///     Null,
///     Bool(bool),
///     Number(Number),
///     String(String),
///     Array(Vec<Value>),
///     Object(Map<String, Value>),
/// }
/// ```
pub type JsonValue = serde_json::Value;

/// Integer used in other types.
///
/// A signed 128-bit integer can store up to 2^127, or 340282366920938463463374607431768211456.
///
/// This integer can store a number with 38 digits, the max supply of ADA is 45 billion (11 digits).
pub type Integer = i128;

/// Float used in other types.
pub type Float = f64;

/// JSON Map (or JSON object) made of key-value pairs.
///
/// Used in types:
/// [`EpochParameters`]
/// [`AssetDetails`]
pub type JsonMap = serde_json::Map<String, JsonValue>;

/// Inner enum for [`PoolUpdate`] and [`AccountRegistration`].
///
/// Action in the certificate.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    Registered,
    Deregistered,
}
