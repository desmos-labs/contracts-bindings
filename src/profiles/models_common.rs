use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Represents a generic public key.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PubKey {
    /// Public key type.
    #[serde(rename = "@type")]
    pub proto_type: String,
    /// Hex encoded public key.
    pub key: String,
}
