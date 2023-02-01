pub use desmos_std::proto::desmos::posts::v2::*;

use crate::stargate::types::Any;

pub enum AttachmentContent {
    Poll(Poll),
    Media(Media),
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
