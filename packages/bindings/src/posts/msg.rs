//! Contains the messages that can be sent to the chain to interact with the x/posts module.

use crate::posts::proto::*;
use crate::posts::types::AttachmentContent;
use cosmwasm_std::Addr;

/// Represents the messages to interact with the posts module.
pub struct PostsMsgBuilder {}

impl PostsMsgBuilder {
    /// Creates an instance of [`PostsMsg::CreatePost`].
    ///
    /// * `subspace_id` - Id of the subspace inside which the post must be created.
    /// * `section_id` - Id of the section inside which the post must be created.
    /// * `external_id` - External id for this post.
    /// * `text` - Text of the post.
    /// * `entities` - Entities connected to this post.
    /// * `tags` - Tags related to this post.
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
        tags: Vec<&str>,
        attachments: Vec<AttachmentContent>,
        author: Addr,
        conversation_id: Option<u64>,
        reply_settings: ReplySetting,
        referenced_posts: Vec<PostReference>,
    ) -> MsgCreatePost {
        MsgCreatePost {
            subspace_id,
            section_id,
            external_id: external_id.unwrap_or_default().into(),
            text: text.unwrap_or_default().into(),
            entities,
            attachments: attachments
                .into_iter()
                .map(Into::into)
                .collect(),
            tags: tags.into_iter().map(Into::into).collect(),
            author: author.into(),
            conversation_id: conversation_id.unwrap_or_default(),
            reply_settings: reply_settings.into(),
            referenced_posts,
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
        tags: Vec<&str>,
        editor: Addr,
    ) -> MsgEditPost {
        MsgEditPost {
            subspace_id,
            post_id,
            text: text.unwrap_or("[do-not-modify]").to_string(),
            entities,
            tags: tags.into_iter().map(Into::into).collect(),
            editor: editor.into(),
        }
    }

    /// Creates an instance of [`PostsMsg::DeletePost`].
    ///
    /// * `subspace_id` - Id of the subspace containing the post.
    /// * `post_id` - Id of the post to be deleted.
    /// * `signer` - User that is deleting the post.
    pub fn delete_post(subspace_id: u64, post_id: u64, signer: Addr) -> MsgDeletePost {
        MsgDeletePost {
            subspace_id,
            post_id,
            signer: signer.into(),
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
        content: Option<AttachmentContent>,
        editor: Addr,
    ) -> MsgAddPostAttachment {
        MsgAddPostAttachment {
            subspace_id,
            post_id,
            content: content.map(Into::into),
            editor: editor.into(),
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
    ) -> MsgRemovePostAttachment {
        MsgRemovePostAttachment {
            subspace_id,
            post_id,
            attachment_id,
            editor: editor.into(),
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
    ) -> MsgAnswerPoll {
        MsgAnswerPoll {
            subspace_id,
            post_id,
            poll_id,
            answers_indexes,
            signer: signer.into(),
        }
    }
}

#[cfg(test)]
mod tests {}
