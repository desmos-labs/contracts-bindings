//! Contains the messages that can be sent to the chain to interact with the x/posts module.

use crate::posts::models::{Entities, PostReference, RawPostAttachment, ReplySetting};
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
        attachments: Option<Vec<RawPostAttachment>>,
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
        content: RawPostAttachment,
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

impl PostsMsg {
    /// Creates an instance of [`PostsMsg::CreatePost`].
    /// 
    /// * `subspace_id` - Id of the subspace inside which the post must be created.
    /// * `section_id` - Id of the section inside which the post must be created.
    /// * `external_id` - External id for this post.
    /// * `text` - Text of the post.
    /// * `entities` - Entities connected to this post.
    /// * `attachments` - Attachments of the post.
    /// * `author` - Author of the post.
    /// * `conversation_id` - Id of the original post of the conversation.
    /// * `reply_settings` - Reply settings of this post.
    /// * `reference_posts` - A list this posts references (either as a reply, repost or quote).
    pub fn create_post(
         subspace_id: u64,
         section_id: u32,
         external_id: Option<&str>,
         text: Option<&str>,
         entities: Option<Entities>,
         attachments: Option<Vec<RawPostAttachment>>,
         author: Addr,
         conversation_id: Option<u64>,
         reply_settings: ReplySetting,
         referenced_posts: Vec<PostReference>,
    ) -> Self {
        Self::CreatePost{
            subspace_id: subspace_id.into(),
            section_id: section_id,
            external_id: external_id.map(str::to_string),
            text: text.map(str::to_string),
            entities: entities,
            attachments: attachments,
            author: author,
            conversation_id: conversation_id.map(Uint64::from),
            reply_settings: reply_settings,
            referenced_posts: referenced_posts
        }
    }

    /// Creates an instance of [`PostsMsg::EditPost`].
    /// 
    /// * `subspace_id` - Id of the subspace inside which the post is.
    /// * `post_id` - Id of the post to edit.
    /// * `text` - New text of the post. If set to `[do-not-modify]` it will not change the current post's text.
    /// * `entities` - New entities connected to this post. These will always replace the current post's entities.
    /// * `editor` - Editor of the post.
    pub fn edit_post(
        subspace_id: u64,
        post_id: u64,
        text: Option<&str>,
        entities: Option<Entities>,
        editor: Addr,
    ) -> Self {
        Self::EditPost {
            subspace_id: subspace_id.into(),
            post_id: post_id.into(),
            text: text.unwrap_or("[do-not-modify]").to_string(),
            entities: entities,
            editor: editor,
        }
    }

    /// Creates an instance of [`PostsMsg::DeletePost`].
    /// 
    /// * `subspace_id` - Id of the subspace containing the post.
    /// * `post_id` - Id of the post to be deleted.
    /// * `signer` - User that is deleting the post.
    pub fn delete_post(
        subspace_id: u64,
        post_id: u64,
        signer: Addr
    ) -> Self {
        Self::DeletePost {
            subspace_id: subspace_id.into(),
            post_id: post_id.into(),
            signer: signer,
        }
    }

    /// Creates an instance of [`PostsMsg::AddPostAttachment`].
    /// 
    /// * `subspace_id` - Id of the subspace containing the post.
    /// * `post_id` - Id of the post from which to remove the attachment.
    /// * `attachment_id` - Id of the attachment to be removed.
    /// * `editor` - User that is removing the attachment.
    pub fn add_post_attachment(
        subspace_id: u64,
        post_id: u64,
        content: PostAttachment,
        editor: Addr,
    ) -> Self {
        Self::AddPostAttachment {
            subspace_id: subspace_id.into(),
            post_id: post_id.into(),
            content: content.into(),
            editor: editor,
        }
    }

    /// Creates an instance of [`PostsMsg::RemovePostAttachment`].
    /// 
    /// * `subspace_id` - Id of the subspace containing the post.
    /// * `post_id` - Id of the post from which to remove the attachment.
    /// * `attachment_id` - Id of the attachment to be removed.
    /// * `editor` - User that is removing the attachment.
    pub fn remove_post_attachment(
        subspace_id: u64,
        post_id: u64,
        attachment_id: u32,
        editor: Addr,
    ) -> Self {
        Self::RemovePostAttachment {
            subspace_id: subspace_id.into(),
            post_id: post_id.into(),
            attachment_id: attachment_id,
            editor: editor,
        }
    }

    /// Creates an instance of [`PostsMsg::AnswerPoll`].
    /// * `subspace_id` - Id of the subspace containing the post.
    /// * `post_id` - Id of the post that contains the poll to be answered.
    /// * `poll_id` - Id of the poll to be answered.
    /// * `answers_indexes` - Indexes of the answer inside the ProvidedAnswers array.
    /// * `signer` - Address of the user answering the poll.
    pub fn answer_poll(
        subspace_id: u64,
        post_id: u64,
        poll_id: u32,
        answers_indexes: Vec<u32>,
        signer: Addr,
    ) -> Self {
        Self::AnswerPoll {
            subspace_id: subspace_id.into(),
            post_id: post_id.into(),
            poll_id: poll_id,
            answers_indexes: answers_indexes,
            editor: editor,
        }
    }
}