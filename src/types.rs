use serde::{Deserialize, Serialize};

// Use this module as an interface to export all types declared inside of endpoints/
//
// These are not used in here, just exporting
pub use crate::api::endpoints::*;

/// Enum for any possible JSON value.
///
/// Declared as the following:
///
/// ```ignore
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
