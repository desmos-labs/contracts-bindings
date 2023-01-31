use crate::stargate::reactions::proto::{FreeTextValue, RegisteredReactionValue};
use crate::stargate::types::Any;

pub enum ReactionValue {
    FreeTextValue(FreeTextValue),
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
