//! Contains structs and enums related to the x/reactions module.

use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use thiserror::Error;

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
    pub value: ReactionValueJSON,
    /// Author of the reaction.
    pub author: Addr,
}

/// Struct representing a generic reaction value that can be serialized and sent to the chain.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(untagged)]
pub enum ReactionValueJSON {
    /// Represents the registered reaction.
    Registered {
        /// ReactionValue uri type, it must be:
        /// * `/desmos.reactions.v1.RegisteredReactionValue`
        #[serde(rename = "@type")]
        type_uri: String,
        /// Id of the registered reaction.
        registered_reaction_id: u32,
    },
    /// Represents the free text reaction.
    FreeText {
        /// ReactionValue uri type, it must be:
        /// * `/desmos.reactions.v1.FreeTextValue`
        #[serde(rename = "@type")]
        type_uri: String,
        /// Text of the reaction value.
        text: String,
    }
}

/// Contains the details of a reaction value that
/// references a reaction registered within the subspace.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RegisteredReactionValue {
    /// Id of the registered reaction.
    pub registered_reaction_id: u32,
}

impl TryFrom<ReactionValueJSON> for RegisteredReactionValue {
    type Error = UnwrapReactionValueJSONError;
    fn try_from(value: ReactionValueJSON) ->  Result<Self, Self::Error> {
        match value {
            ReactionValueJSON::Registered { type_uri, registered_reaction_id } => { Ok(Self { registered_reaction_id: registered_reaction_id }) }
            _ => { Err(UnwrapReactionValueJSONError::InvalidReactionValue) }
        }
    }
}

impl Into<ReactionValueJSON> for RegisteredReactionValue {
    fn into(self) -> ReactionValueJSON {
        ReactionValueJSON::Registered {
            type_uri: "/desmos.reactions.v1.RegisteredReactionValue".to_string(),
            registered_reaction_id: self.registered_reaction_id,
        }
    }
}

/// Represents the errors that can occur when converting a [`RegisteredReaction`] or [`FreeTextValue`] into a [`ReactionValueJSON`].
#[derive(Error, Debug, Clone)]
pub enum UnwrapReactionValueJSONError {
    /// Error that occur if [`ReactionValueJSON`] is an invalid reaction value.
    #[error("invalid reaction value")]
    InvalidReactionValue,
}

/// Contains the details of a reaction value that is made of free text.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FreeTextValue {
    /// Text of the reaction.
    pub text: String,
}

impl TryFrom<ReactionValueJSON> for FreeTextValue {
    type Error = UnwrapReactionValueJSONError;
    fn try_from(value: ReactionValueJSON) ->  Result<Self, Self::Error> {
        match value {
            ReactionValueJSON::FreeText { type_uri, text } => { Ok(Self { text: text }) }
            _ => { Err(UnwrapReactionValueJSONError::InvalidReactionValue) }
        }
    }
}

impl Into<ReactionValueJSON> for FreeTextValue{
    fn into(self) -> ReactionValueJSON {
        ReactionValueJSON::FreeText{
            type_uri: "/desmos.reactions.v1.FreeTextValue".to_string(),
            text: self.text,
        }
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

