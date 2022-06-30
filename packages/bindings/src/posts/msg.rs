//! Contains the messages that can be sent to the chain to interact with the x/posts module.

use crate::posts::models::{Entities, PostReference, PostAttachment, RawPostAttachment, ReplySetting};
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
         attachments: Option<Vec<PostAttachment>>,
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
            attachments: attachments.map(|attachments| attachments.into_iter().map(|content| content.into()).collect()),
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
            signer: signer,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::posts::models::ProvidedAnswer;

    #[test]
    fn test_create_post() {
        let msg = PostsMsg::create_post(
            1,
            1,
            Some("1"),
            Some("test"),
            None,
            Some(vec![
                PostAttachment::Media {
                    uri: "ftp://domain.io/image.png".to_string(),
                    mime_type: "image/png".to_string(),
                },
                PostAttachment::Poll{
                    question: "questions?".to_string(),
                    provided_answers: vec![ProvidedAnswer {
                            text: Some("Answer 1".to_string()),
                            attachments: vec![],
                    }],
                    end_date: "2140-01-01T10:00:20.021Z".to_string(),
                    allows_multiple_answers: false,
                    allows_answer_edits: false,
                    final_tally_results: None,
                },
            ]),
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            Some(1),
            ReplySetting::Everyone,
            vec![],
        );
        let expected = PostsMsg::CreatePost {
            subspace_id: Uint64::new(1),
            section_id: 1,
            external_id: Some("1".to_string()),
            text: Some("test".to_string()),
            entities: None,
            attachments: Some(vec![
                PostAttachment::Media {
                    uri: "ftp://domain.io/image.png".to_string(),
                    mime_type: "image/png".to_string(),
                }.into(),
                PostAttachment::Poll{
                    question: "questions?".to_string(),
                    provided_answers: vec![ProvidedAnswer {
                            text: Some("Answer 1".to_string()),
                            attachments: vec![],
                    }],
                    end_date: "2140-01-01T10:00:20.021Z".to_string(),
                    allows_multiple_answers: false,
                    allows_answer_edits: false,
                    final_tally_results: None,
                }.into(),
            ]),
            author: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            conversation_id: Some(Uint64::new(1)),
            reply_settings: ReplySetting::Everyone,
            referenced_posts: vec![],
        };
        assert_eq!(expected, msg)
    }
    
    #[test]
    fn test_edit_post_with_new_text() {
        let msg = PostsMsg::edit_post(
            1,
            1,
            Some("new text"),
            None,
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        );
        let expected = PostsMsg::EditPost {
            subspace_id: Uint64::new(1),
            post_id: Uint64::new(1),
            text: "new text".to_string(),
            entities: None,
            editor: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_edit_post_without_new_text() {
        let msg = PostsMsg::edit_post(
            1,
            1,
            None,
            None,
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        );
        let expected = PostsMsg::EditPost {
            subspace_id: Uint64::new(1),
            post_id: Uint64::new(1),
            text: "[do-not-modify]".to_string(),
            entities: None,
            editor: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_delete_post() {
        let msg = PostsMsg::delete_post(
            1,
            1,
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        );
        let expected = PostsMsg::DeletePost {
            subspace_id: Uint64::new(1),
            post_id: Uint64::new(1),
            signer: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        };
        assert_eq!(expected, msg);
    }

    #[test]
    fn test_add_post_attachment() {
        let msg = PostsMsg::add_post_attachment(
            1,
            1,
            PostAttachment::Media {
                uri: "ftp://domain.io/image.png".to_string(),
                mime_type: "image/png".to_string(),
            },
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        );
        let expected = PostsMsg::AddPostAttachment {
            subspace_id: Uint64::new(1),
            post_id: Uint64::new(1),
            content: PostAttachment::Media {
                uri: "ftp://domain.io/image.png".to_string(),
                mime_type: "image/png".to_string(),
            }.into(),
            editor: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        };
        assert_eq!(expected, msg);
    }

    
    #[test]
    fn test_remove_post_attachment() {
        let msg = PostsMsg::remove_post_attachment(
            1,
            1,
            1,
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        );
        let expected = PostsMsg::RemovePostAttachment {
            subspace_id: Uint64::new(1),
            post_id: Uint64::new(1),
            attachment_id: 1,
            editor: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        };
        assert_eq!(expected, msg);
    }

    #[test]
    fn test_answer_poll() {
        let msg = PostsMsg::answer_poll(
            1,
            1,
            1,
            vec![1],
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        );
        let expected = PostsMsg::AnswerPoll {
            subspace_id: Uint64::new(1),
            post_id: Uint64::new(1),
            poll_id: 1,
            answers_indexes: vec![1],
            signer: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        };
        assert_eq!(expected, msg);
    }
}