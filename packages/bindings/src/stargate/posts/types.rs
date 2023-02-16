//! Contains the types definitions of x/posts.

pub use desmos_std::proto::desmos::posts::v3::*;

use crate::stargate::types::Any;

/// Represents a generic attachment content.
pub enum AttachmentContent {
    /// Represents the poll content.
    Poll(Poll),

    /// Represents the media content.
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
