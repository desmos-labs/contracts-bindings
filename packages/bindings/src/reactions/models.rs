//! Contains structs and enums related to the x/reactions module.

use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Contains the data of a single post reaction.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Reaction {
    /// Id of the subspace inside which the reaction has been put.
    pub subspace_id: Uint64,
    /// Id of the post to which the reaction is associated.
    pub post_id: Uint64,
    /// Id of the reaction within the post.
    pub id: u32,
    /// Value of the reaction.
    pub value: ReactionValue,
    /// Author of the reaction.
    pub author: Addr,
}

/// Struct representing a generic reaction value that can be serialized and sent to the chain.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(tag = "@tag")]
pub enum ReactionValue {
    /// Represents the registered reaction.
    #[serde(rename = "/desmos.reactions.v1.RegisteredReactionValue")]
    Registered {
        /// Id of the registered reaction.
        registered_reaction_id: u32,
    },
    /// Represents the free text reaction.
    #[serde(rename = "/desmos.reactions.v1.FreeTextValue")]
    FreeText {
        /// Text of the reaction value.
        text: String,
    }
}

/// Contains the details of a registered reaction within a subspace.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RegisteredReaction{
    /// Id of the subspace for which this reaction has been registered.
    pub subspace_id: Uint64,
    /// Id of the registered reaction.
    pub id: u32,
    /// Unique shorthand code associated to this reaction.
    pub shorthand_code: String,
    /// Value that should be displayed when using this reaction.
    pub display_value: String,
}

/// Contains the parameters related to a single subspace reactions.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SubspaceReactionsParams {
    /// Id of the subspace for which these params are valid.
    pub subspace_id: Uint64,
    /// Params related to [`RegisteredReactionValue`] reactions.
    pub registered_reaction: RegisteredReactionValueParams,
    /// Params related to [`FreeTextValue`] reactions.
    pub free_text: FreeTextValueParams,
}

/// Contains the params for [`FreeTextValue`] based reactions.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FreeTextValueParams {
    /// Whether [`FreeTextValue`] reactions should be enabled.
    pub enabled: bool,
    /// The max length that [`FreeTextValue`] reactions should have.
    pub max_length: u32,
    /// RegEx that each [`FreeTextValue`] should respect.
    /// This is useful to limit what characters can be used as a reaction.
    pub reg_ex: String,
}

/// Contains the params for [`RegisteredReactionValue`] based reactions.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RegisteredReactionValueParams {
    /// Whether [`RegisteredReactionValue`] reactions should be enabled.
    pub enabled: bool,
}

