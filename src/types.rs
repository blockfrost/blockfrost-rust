//! Common types that are reused in multiple models/schemas.

use serde::{Deserialize, Serialize};

pub type Integer = i64;
pub type Float = f64;

/// Represents an amount of some unit.
///
/// This struct is a member in the structs:
/// - [`Address`].
/// - [`AddressTotal`].
/// - [`AddressUtxo`].
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

#[cfg(test)]
mod tests {
    use super::*;

    test_schema! { test_transaction_amount, Amount, r#"
    {
      "unit": "lovelace",
      "quantity": "42000000"
    }
    "# }
}
