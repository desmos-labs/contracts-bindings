//! Contains the types definitions of x/reactions.

pub use desmos_std::proto::desmos::reactions::v1::*;

use crate::cosmos_types::Any;

/// Represents a generic reaction value.
pub enum ReactionValue {
    /// Represents the value of free text reaction.
    FreeText(FreeTextValue),

    /// Represents the value of registered reaction.
    Registered(RegisteredReactionValue),
}

impl Into<Any> for ReactionValue {
    fn into(self) -> Any {
        match self {
            ReactionValue::FreeText(value) => value.into(),
            ReactionValue::Registered(value) => value.into(),
        }
    }
}
