//! Contains structs and enums related to the chain links.

use crate::types::PubKey;
use cosmwasm_std::{Addr, Binary};
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
    /// Optional address prefix when `proto_type` is Bech32 or Hex.
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
    pub value_type: SignatureValueType,
    /// Signature data.
    pub signature: Binary,
}

/// Represents all the possible signature types.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum SignatureValueType {
    /// Specifies an unknown signing mode and will be rejected.
    #[serde(rename = "SIGNATURE_VALUE_TYPE_UNSPECIFIED")]
    Unspecified,
    /// Used when the value has been signed as a raw byte array
    #[serde(rename = "SIGNATURE_VALUE_TYPE_RAW")]
    Raw,
    /// Used when the signed value has been encoded as a Protobuf transaction containing the owner
    /// address inside its memo field.
    #[serde(rename = "SIGNATURE_VALUE_TYPE_COSMOS_DIRECT")]
    CosmosDirect,
    /// Used when the value has been encoded as an Amino transaction containing the owner address inside
    /// its memo field.
    #[serde(rename = "SIGNATURE_VALUE_TYPE_COSMOS_AMINO")]
    CosmosAnimo,
    /// Used when the value has been encoded following the EVM personal_sign specification.
    #[serde(rename = "SIGNATURE_VALUE_TYPE_EVM_PERSONAL_SIGN")]
    EVMPersonalSign,
}

/// Contains the data of the linked chain.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ChainConfig {
    /// Name of the chain.
    pub name: String,
}

/// Contains the details of a single chain link owner.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ChainLinkOwnerDetails {
    /// Address of the link owner.
    pub user: Addr,
    /// Name of the chain.
    pub chain_name: String,
    /// Address of the link target
    pub target: String,
}
