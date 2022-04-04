//! Contains structs and enums relative to the x/relationships module.

use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Struct that represents a block.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UserBlock {
    /// User that had blocked the `blocked`.
    pub blocker: Addr,
    /// User blocked from the `blocker`.
    pub blocked: Addr,
    /// Reason of the block.
    pub reason: String,
    /// Subspace in which the block has been created.
    pub subspace_id: Uint64,
}

/// Struct that represents a relationship, like a follow on Twitter.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Relationship {
    /// Address of who has created the relationship.
    pub creator: Addr,
    /// Address of the counterparty.
    pub counterparty: Addr,
    /// Subspace in which the relationship has been created.
    pub subspace_id: Uint64,
}
