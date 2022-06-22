//! Contains structs and enums related to the x/reactions module.

use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Contains the data of a single post reaction.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Reaction {
    /// Id of the subspace inside which the reaction has been put
    pub subspace_id: Uint64,
    /// Id of the post to which the reaction is associated
    pub post_id: Uint64,
    /// Id of the reaction within the post
    pub id: u32,
    /// Value of the reaction.
    pub value: ReactionValueJSON,
    /// Author of the reaction
    pub author: Addr,
}

/// Struct representing a generic reaction value that can be serialized and sent to the chain.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(untagged)]
pub enum ReactionValueJSON {
    /// Represents the registered reaction
    Registered {
        /// ReactionValue uri type, it must be:
        /// * `/desmos.reactions.v1.RegisteredReactionValue`
        #[serde(rename = "@type")]
        type_uri: String,
        /// Id of the registered reaction
        registered_reaction_id: u32,
    },
    /// Represents the free text reaction
    FreeText {
        /// ReactionValue uri type, it must be:
        /// * `/desmos.reactions.v1.FreeTextValue`
        #[serde(rename = "@type")]
        type_uri: String,
        /// Text of the reaction value
        text: String,
    }
}

/// Contains the details of a reaction value that
/// references a reaction registered within the subspace.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RegisteredReactionValue {
    /// Id of the registered reaction
    pub registered_reaction_id: u32,
}

/// Contains the details of a reaction value that is made of free text.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FreeTextValue {
    /// Text of the reaction 
    pub text: String,
}

/// Contains the details of a registered reaction within a subspace.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RegisteredReaction{
    /// Id of the subspace for which this reaction has been registered
    pub subspace_id: Uint64,
    /// Id of the registered reaction
    pub id: u32,
    /// Unique shorthand code associated to this reaction
    pub shorthand_code: String,
    /// Value that should be displayed when using this reaction
    pub display_value: String,
}

/// Contains the params related to a single subspace
/// reactions
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SubspaceReactionsParams {
    /// Id of the subspace for which these params are valid
    pub subspace_id: Uint64,
    /// Params related to [`RegisteredReactionValue`](crate::reactions::models::RegisteredReactionValue) reactions
    pub registered_reaction: RegisteredReactionValueParams,
    /// Params related to [`FreeTextValue`](crate::reactions::models::FreeTextValue) reactions
    pub free_text: FreeTextValueParams,
}

/// Contains the params for [`FreeTextValue`](crate::reactions::models::FreeTextValue) based reactions
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FreeTextValueParams {
    /// Whether [`FreeTextValue`](crate::reactions::models::FreeTextValue) reactions should be enabled
    pub enabled: bool,
    /// The max length that [`FreeTextValue`](crate::reactions::models::FreeTextValue) reactions should have
    pub max_length: u32,
    /// RegEx that each [`FreeTextValue`](crate::reactions::models::FreeTextValue) should respect.
    /// This is useful to limit what characters can be used as a reaction.
    pub reg_ex: String,
}

/// Contains the params for [`RegisteredReactionValue`](crate::reactions::models::RegisteredReactionValue) based reactions.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RegisteredReactionValueParams {
    /// Whether [`RegisteredReactionValue`](crate::reactions::models::RegisteredReactionValue) reactions should be enabled
    pub enabled: bool,
}

