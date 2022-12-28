use crate::stargate::posts::proto::{Media, Poll};
use crate::stargate::types::Any;
use cosmwasm_std::StdError;
use prost::Message;

pub enum AttachmentContent {
    Poll(Poll),
    Media(Media),
}

impl TryFrom<Any> for AttachmentContent {
    type Error = StdError;
    fn try_from(any: Any) -> Result<Self, Self::Error> {
        match any.type_url {
            Poll::TYPE_URL => Poll::from(any),
            Media::TYPE_URL => Media::from(any),
            _ =>  Err(StdError::ParseErr {
                target_type: "AttachmentContent".to_string(),
                msg: "Unmatched type: must be either `Poll`, `Media` or `Base58Address`.".to_string(),
            })
        }
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

pub enum PublicKey {
    Ed25519PublicKey(),
}
