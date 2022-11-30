//! Contains the types definitions of all the responses to the x/reactions query messages.

use crate::legacy::{
    reactions::models::{Reaction, RegisteredReaction, SubspaceReactionsParams},
    types::PageResponse,
};
use cosmwasm_schema::cw_serde;

/// Response to [`ReactionsQuery::Reactions`](crate::legacy::reactions::query::ReactionsQuery::Reactions).
#[cw_serde]
pub struct QueryReactionsResponse {
    /// Queried reactions.
    pub reactions: Vec<Reaction>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`ReactionsQuery::Reaction`](crate::legacy::reactions::query::ReactionsQuery::Reaction).
#[cw_serde]
pub struct QueryReactionResponse {
    /// Queried reaction.
    pub reaction: Reaction,
}

/// Response to [`ReactionsQuery::RegisteredReactions`](crate::legacy::reactions::query::ReactionsQuery::RegisteredReactions).
#[cw_serde]
pub struct QueryRegisteredReactionsResponse {
    /// Queried registred reactions.
    pub registered_reactions: Vec<RegisteredReaction>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`ReactionsQuery::RegisteredReaction`](crate::legacy::reactions::query::ReactionsQuery::RegisteredReaction).
#[cw_serde]
pub struct QueryRegisteredReactionResponse {
    /// Queried registered reaction.
    pub registered_reaction: RegisteredReaction,
}

/// Response to [`ReactionsQuery::ReactionsParams`](crate::legacy::reactions::query::ReactionsQuery::ReactionsParams).
#[cw_serde]
pub struct QueryReactionsParamsResponse {
    /// Queried reactions parameters.
    pub params: SubspaceReactionsParams,
}
