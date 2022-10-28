use crate::posts::proto::{Poll, Media};
use desmos_std::shim::Any;
use cosmwasm_std::StdError;
use prost::Message;

pub enum AttachmentContent {
    Poll(Poll),
    Media(Media),
}

impl TryFrom<Any> for AttachmentContent {
    type Error = StdError;
    fn try_from(value: Any) -> Result<Self, Self::Error> {
        if let Ok(content) = Poll::decode(value.value.as_slice()) {
            return Ok(AttachmentContent::Poll(content));
        }
        if let Ok(content) = Media::decode(value.value.as_slice()) {
            return Ok(AttachmentContent::Media(content));
        }
        Err(StdError::ParseErr {
            target_type: "AttachmentContent".to_string(),
            msg: "Unmatched type: must be either `Poll`, `Media` or `Base58Address`.".to_string(),
        })
    }
}

impl Into<Any> for AttachmentContent {
    fn into(self) -> Any {
        match self {
            AttachmentContent::Poll(content) => content.into(),
            AttachmentContent::Media(content) => content.into(),
        }
    }
}