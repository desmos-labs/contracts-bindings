use crate::stargate::reactions::proto::{FreeTextValue, RegisteredReactionValue};
use crate::stargate::types::Any;
use cosmwasm_std::StdError;
use prost::Message;

pub enum ReactionValue {
    FreeTextValue(FreeTextValue),
    RegisteredReactionValue(RegisteredReactionValue),
}

impl TryFrom<Any> for ReactionValue {
    type Error = StdError;
    fn try_from(value: Any) -> Result<Self, Self::Error> {
        if let Ok(address) = FreeTextValue::decode(value.value.as_slice()) {
            return Ok(ReactionValue::FreeTextValue(address));
        }
        if let Ok(address) = RegisteredReactionValue::decode(value.value.as_slice()) {
            return Ok(ReactionValue::RegisteredReactionValue(address));
        }
        Err(StdError::ParseErr {
            target_type: "ReactionValue".to_string(),
            msg: "Unmatched type: must be either `FreeTextValue` or `RegisteredReactionValue`."
                .to_string(),
        })
    }
}

impl Into<Any> for ReactionValue {
    fn into(self) -> Any {
        match self {
            ReactionValue::FreeTextValue(address) => address.into(),
            ReactionValue::RegisteredReactionValue(address) => address.into(),
        }
    }
}
