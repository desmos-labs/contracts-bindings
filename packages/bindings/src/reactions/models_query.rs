//! Contains the types definitions of all the responses to the x/reactions query messages.

use crate::{
    reactions::models::{Reaction, RegisteredReaction, SubspaceReactionsParams},
    types::PageResponse,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Response to [`ReactionsQuery::Reactions`](crate::reactions::query::ReactionsQuery::Reactions).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryReactionsResponse{
    /// Queried reactions.
    pub reactions: Vec<Reaction>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`ReactionsQuery::RegisteredReactions`](crate::reactions::query::ReactionsQuery::RegisteredReactions).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryRegisteredReactionsResponse{
    /// Queried registred reactions.
    pub registered_reaction: Vec<RegisteredReaction>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`ReactionsQuery::ReactionsParams`](crate::reactions::query::ReactionsQuery::ReactionsParams).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryReactionsParamsResponse{
    /// Queried reactions parameters.
    pub params: SubspaceReactionsParams,
}