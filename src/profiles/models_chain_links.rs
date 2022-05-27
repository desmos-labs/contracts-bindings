//! Contains structs and enums related to the chain links.

use crate::types::PubKey;
use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Contains the data of the external chain address to be connected with the Desmos profile.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Address {
    /// The address proto type.
    #[serde(rename = "@type")]
    pub proto_type: String,
    /// The encoded address.
    pub value: String,
    /// Optional address prefix when `prototype` is Bech32 or Hex.
    pub prefix: Option<String>,
}

/// Contains the data representing either an inter- or cross- chain link.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ChainLink {
    /// Defines the destination profile address to link.
    pub user: Addr,
    /// Contains the data of the external chain address to be connected
    /// with the Desmos profile.
    pub address: Address,
    /// Contains the ownership proof of the external chain address.
    pub proof: Proof,
    /// Contains the configuration of the external chain.
    pub chain_config: ChainConfig,
    /// Represents the time in which the link has been created.
    pub creation_time: String,
}

/// Contains all the data used to verify a signature when linking an account to a profile.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Proof {
    /// Represents the public key associated with the address for which to prove the ownership.
    pub pub_key: PubKey,
    /// Represents the hex-encoded signature of the `plain_text` value.
    pub signature: Signature,
    /// represents the hex-encoded value signed in order to produce the `signature`.
    pub plain_text: String,
}

/// Represents a signature of a payload.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Signature {
    /// Signature type.
    #[serde(rename = "@type")]
    pub proto_type: String,
    /// Sign mode.
    pub mode: String,
    /// Signature data.
    pub signature: String,
}

/// Contains the data of the linked chain.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ChainConfig {
    /// Name of the chain.
    pub name: String,
}
