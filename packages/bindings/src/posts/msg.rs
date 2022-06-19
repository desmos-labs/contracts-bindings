//! Contains the messages that can be sent to the chain to interact with the x/posts module.

use crate::posts::models::{Entities, PostReference, ReplySetting};
use crate::types::Any;
use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Represents the messages to interact with the posts module.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PostsMsg {
    /// Represents the message to be used to create a post.
    CreatePost {
        /// Id of the subspace inside which the post must be created.
        subspace_id: Uint64,
        /// Id of the section inside which the post must be created.
        section_id: u32,
        /// External id for this post.
        external_id: Option<String>,
        /// Text of the post.
        text: Option<String>,
        /// Entities connected to this post.
        entities: Option<Entities>,
        /// Attachments of the post.
        attachments: Option<Vec<Any>>,
        /// Author of the post.
        author: Addr,
        /// Id of the original post of the conversation.
        conversation_id: Option<Uint64>,
        /// Reply settings of this post.
        reply_settings: ReplySetting,
        /// A list this posts references (either as a reply, repost or quote).
        referenced_posts: Vec<PostReference>,
    },
    /// Represents the message to be used to edit a post.
    EditPost {
        /// Id of the subspace inside which the post is.
        subspace_id: Uint64,
        /// Id of the post to edit.
        post_id: Uint64,
        /// New text of the post. If set to `[do-not-modify]` it will not change the current
        /// post's text.
        text: String,
        /// New entities connected to this post. These will always replace the current
        /// post's entities.
        entities: Option<Entities>,
        /// Editor of the post.
        editor: Addr,
    },
    /// Represents the message used when deleting a post.
    DeletePost {
        /// Id of the subspace containing the post.
        subspace_id: Uint64,
        /// Id of the post to be deleted.
        post_id: Uint64,
        /// User that is deleting the post.
        signer: Addr,
    },
    /// Represents the message that should be used when adding an attachment to post.
    AddPostAttachment {
        /// Id of the subspace containing the post.
        subspace_id: Uint64,
        /// Id of the post to which to add the attachment.
        post_id: Uint64,
        /// Content of the attachment.
        content: Any,
        /// Editor of the post.
        editor: Addr,
    },
    /// Represents the message to be used when removing an attachment from a post.
    RemovePostAttachment {
        /// Id of the subspace containing the post.
        subspace_id: Uint64,
        /// Id of the post from which to remove the attachment.
        post_id: Uint64,
        /// Id of the attachment to be removed.
        attachment_id: u32,
        /// User that is removing the attachment.
        editor: Addr,
    },
    /// Represents the message used to answer a poll.
    AnswerPoll {
        /// Id of the subspace containing the post.
        subspace_id: Uint64,
        /// Id of the post that contains the poll to be answered.
        post_id: Uint64,
        /// Id of the poll to be answered.
        poll_id: u32,
        /// Indexes of the answer inside the ProvidedAnswers array.
        answers_indexes: Vec<u32>,
        /// Address of the user answering the poll.
        signer: Addr,
    },
}
