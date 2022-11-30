//! Contains the query messages that can be sent to the chain in order to query data related
//! to the x/reactions module.

use crate::reactions::models_query::*;
use crate::types::PageRequest;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint64};

/// Query messages that can be sent to the x/reactions module.
#[cw_serde]
#[derive(QueryResponses)]
pub enum ReactionsQuery {
    /// Message to query the reactions associated to the post.
    #[returns(QueryReactionsResponse)]
    Reactions {
        /// Id of the subspace that contains the post to query the reactions for.
        subspace_id: Uint64,
        /// Id of the post to query the reactions for.
        post_id: Uint64,
        /// Address of the user to query the reactions for.
        user: Option<Addr>,
        /// Optional pagination configs.
        pagination: Option<PageRequest>,
    },
    /// Message to query the reaction with the given id.
    #[returns(QueryReactionResponse)]
    Reaction {
        /// Id of the subspace that contains the post to query the reactions for.
        subspace_id: Uint64,
        /// Post id to query the reactions for.
        post_id: Uint64,
        /// Id of the reaction to query.
        reaction_id: u32,
    },
    /// Message to query the regiestered reactions inside the subspace.
    #[returns(QueryRegisteredReactionsResponse)]
    RegisteredReactions {
        /// Id of the subspace to query the registered reactions for.
        subspace_id: Uint64,
        /// Optional pagination configs.
        pagination: Option<PageRequest>,
    },
    /// Message to query the registered reactions.
    #[returns(QueryRegisteredReactionResponse)]
    RegisteredReaction {
        /// Id of the subspace to query the registered reactions for.
        subspace_id: Uint64,
        /// Id of the registered reaction to query for.
        reaction_id: u32,
    },
    /// Message to query the reactions parameters inside the subspace.
    #[returns(QueryReactionsParamsResponse)]
    ReactionsParams {
        /// Id of the subspace for which to query the params.
        subspace_id: Uint64,
    },
}
