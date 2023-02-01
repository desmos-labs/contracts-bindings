//! Contains the types definitions of x/reactions.

pub use desmos_std::proto::desmos::reactions::v1::*;

use crate::stargate::types::Any;

/// Represents a generic reaction value.
pub enum ReactionValue {
    /// Represents the value of free text reaction.
    FreeTextValue(FreeTextValue),

    /// Represents the value of registered reaction.
    RegisteredReactionValue(RegisteredReactionValue),
}

impl Into<Any> for ReactionValue {
    fn into(self) -> Any {
        match self {
            ReactionValue::FreeTextValue(address) => address.into(),
            ReactionValue::RegisteredReactionValue(address) => address.into(),
        }
    }
}
