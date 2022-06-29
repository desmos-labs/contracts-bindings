//! Contains the messages that can be sent to the chain to interact with the x/reactions module.

use crate::reactions::models::{
    FreeTextValueParams, RawReactionValue, RegisteredReactionValueParams,
};

use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Represents the messages to interact with the reactions module.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReactionsMsg {
    /// Add a reaction to a post.
    AddReaction {
        /// Id of the subspace inside which the post to react to is
        subspace_id: Uint64,
        /// Id of the post to react to
        post_id: Uint64,
        /// Value of the reaction
        value: RawReactionValue,
        /// User reacting to the post
        user: Addr,
    },
    /// Remove a reaction from a post.
    RemoveReaction {
        /// Id of the subspace inside which the reaction to remove is
        subspace_id: Uint64,
        /// Id of the post from which to remove the reaction
        post_id: Uint64,
        /// Id of the reaction to be removed
        reaction_id: u32,
        /// User removing the reaction
        user: Addr,
    },
    /// Register a new supported reaction to the subspace.
    AddRegisteredReaction {
        /// Id of the subspace inside which this reaction should be registered
        subspace_id: Uint64,
        /// Shorthand code of the reaction
        shorthand_code: String,
        /// Display value of the reaction
        display_value: String,
        /// User adding the supported reaction
        user: Addr,
    },
    /// Edit a registered reaction inside the subspace.
    EditRegisteredReaction {
        /// Id of the subspace inside which the reaction to edit is
        subspace_id: Uint64,
        /// Id of the registered reaction to edit
        registered_reaction_id: u32,
        /// New shorthand code to be set
        shorthand_code: String,
        /// Display value to be set
        display_value: String,
        /// User editing the registered reaction
        user: Addr,
    },
    /// Remove a registered reaction from the subspace.
    RemoveRegisteredReaction {
        /// Id of the subspace from which to remove the registered reaction
        subspace_id: Uint64,
        /// Id of the registered reaction to be removed
        registered_reaction_id: u32,
        /// User removing the registered reaction
        user: Addr,
    },
    /// Set reactions parameters for the subspace.
    SetReactionsParams {
        /// Id of the subspace for which to set the params
        subspace_id: Uint64,
        /// Params related to [`RegisteredReactionValue`](crate::reactions::models::RegisteredReactionValue) reactions
        registered_reaction: RegisteredReactionValueParams,
        /// Params related to [`FreeTextValue`](crate::reactions::models::FreeTextValue) reactions
        free_text: FreeTextValueParams,
        /// User setting the params
        user: Addr,
    },
}
