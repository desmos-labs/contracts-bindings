pub use desmos_std::proto::desmos::reactions::v1::*;

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
