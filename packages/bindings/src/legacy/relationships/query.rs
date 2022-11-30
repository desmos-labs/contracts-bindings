//! Contains the query messages that can be sent to the chain in order to query data related
//! to the x/relationships module.

use crate::legacy::relationships::models_query::*;
use crate::legacy::types::PageRequest;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint64};

/// Query messages that can be sent to the x/relationships module.
#[cw_serde]
#[derive(QueryResponses)]
pub enum RelationshipsQuery {
    /// Message to query the relationships inside a subspaces.
    #[returns(QueryRelationshipsResponse)]
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
    #[returns(QueryBlocksResponse)]
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
