//! Contains structs and enums related to the x/relationships module.

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint64};

/// Struct that represents a block.
#[cw_serde]
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
#[cw_serde]
pub struct Relationship {
    /// Address of who has created the relationship.
    pub creator: Addr,
    /// Address of the counterparty.
    pub counterparty: Addr,
    /// Subspace in which the relationship has been created.
    pub subspace_id: Uint64,
}
