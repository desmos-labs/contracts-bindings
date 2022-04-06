//! Contains the query messages that can be sent to the chain in order to query data related
//! to the x/relationships module.

use crate::types::PageRequest;
use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Query messages that can be sent to the x/relationships module.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RelationshipsQuery {
    /// Message to query the relationships inside a subspaces.
    Relationships {
        /// Subspace to query the relationships for.
        subspace_id: Uint64,
        /// Optional address of the user for which to query the relationships.
        user: Option<Addr>,
        /// Optional address of the counterparty of the relationships (used only if the
        /// `user` is provided).
        counterparty: Option<Addr>,
        /// Pagination configs.
        pagination: Option<PageRequest>,
    },
    /// Message to query the blocks created inside a subspace.
    Blocks {
        /// Subspace to query the blocks for.
        subspace_id: Uint64,
        /// Optional address of the blocker to query the blocks for.
        blocker: Option<Addr>,
        /// Optional address of the blocked user to query the block for (used only if
        /// the `blocker` is provided).
        blocked: Option<Addr>,
        /// Pagination configs.
        pagination: Option<PageRequest>,
    },
}
