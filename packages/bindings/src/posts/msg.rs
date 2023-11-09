//! Contains the messages that can be sent to the chain to interact with the x/posts module.

use crate::posts::types::AttachmentContent;
use crate::posts::types::*;
use cosmwasm_std::Addr;

/// Represents the messages to interact with the posts module.
pub struct PostsMsg {}

impl PostsMsg {
    /// Creates an instance of [`MsgCreatePost`].
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
        text: &str,
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
            text: text.into(),
            entities,
            attachments: attachments.into_iter().map(Into::into).collect(),
            tags: tags.into_iter().map(Into::into).collect(),
            author: author.into(),
            conversation_id: conversation_id.unwrap_or_default(),
            reply_settings: reply_settings.into(),
            referenced_posts,
        }
    }

    /// Creates an instance of [`MsgEditPost`].
    ///
    /// * `subspace_id` - Id of the subspace inside which the post is.
    /// * `post_id` - Id of the post to edit.
    /// * `text` - New text of the post. It will not change the current post's text if set this field to `None`.
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
            text: text.unwrap_or("[do-not-modify]").into(),
            entities,
            tags: tags.into_iter().map(Into::into).collect(),
            editor: editor.into(),
        }
    }

    /// Creates an instance of [`MsgDeletePost`].
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

    /// Creates an instance of [`MsgAddPostAttachment`].
    ///
    /// * `subspace_id` - Id of the subspace containing the post.
    /// * `post_id` - Id of the post from which to remove the attachment.
    /// * `attachment_id` - Id of the attachment to be removed.
    /// * `editor` - User that is removing the attachment.
    pub fn add_post_attachment(
        subspace_id: u64,
        post_id: u64,
        content: AttachmentContent,
        editor: Addr,
    ) -> MsgAddPostAttachment {
        MsgAddPostAttachment {
            subspace_id,
            post_id,
            content: Some(content.into()),
            editor: editor.into(),
        }
    }

    /// Creates an instance of [`MsgRemovePostAttachment`].
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

    /// Creates an instance of [`MsgAnswerPoll`].
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

    /// Creates an instance of [`MsgMovePost`].
    /// * `subspace_id` - Id of the subspace where the post is currently located.
    /// * `post_id` - Id of the post to be moved.
    /// * `target_subspace_id` - Id of the target subspace to which the post will be moved.
    /// * `target_section_id` - Id of the target section to which the post will be moved.
    /// * `owner` - Address of the post owner.
    pub fn move_post(
        subspace_id: u64,
        post_id: u64,
        target_subspace_id: u64,
        target_section_id: u32,
        owner: Addr,
    ) -> MsgMovePost {
        MsgMovePost {
            subspace_id,
            post_id,
            target_subspace_id,
            target_section_id,
            owner: owner.into(),
        }
    }

    /// Creates an instance of [`MsgRequestPostOwnerTransfer`].
    /// * `subspace_id` - Id of the subspace that holds the post which ownership should be transferred.
    /// * `post_id` - Id of the post which will be transferred.
    /// * `receiver` - Address of the post ownership receiver.
    /// * `sender` - Address of the sender who is creating a transfer request.
    pub fn request_post_owner_transfer(
        subspace_id: u64,
        post_id: u64,
        receiver: Addr,
        sender: Addr,
    ) -> MsgRequestPostOwnerTransfer {
        MsgRequestPostOwnerTransfer {
            subspace_id,
            post_id,
            receiver: receiver.into(),
            sender: sender.into(),
        }
    }

    /// Creates an instance of [`MsgCancelPostOwnerTransferRequest`].
    /// * `subspace_id` - Id of the subspace that holds the post for which the request should be canceled.
    /// * `post_id` - Id of the post for which the request will be cancelled.
    /// * `sender` - Address of the transfer request sender.
    pub fn cancel_post_owner_transfer_request(
        subspace_id: u64,
        post_id: u64,
        sender: Addr,
    ) -> MsgCancelPostOwnerTransferRequest {
        MsgCancelPostOwnerTransferRequest {
            subspace_id,
            post_id,
            sender: sender.into(),
        }
    }

    /// Creates an instance of [`MsgAcceptPostOwnerTransferRequest`].
    /// * `subspace_id` - Id of the subspace holding the post for which the request will be accepted.
    /// * `post_id` - Id of the post for which the request will be accepted.
    /// * `receiver` - Address of the request receiver.
    pub fn accept_post_owner_transfer_request(
        subspace_id: u64,
        post_id: u64,
        receiver: Addr,
    ) -> MsgAcceptPostOwnerTransferRequest {
        MsgAcceptPostOwnerTransferRequest {
            subspace_id,
            post_id,
            receiver: receiver.into(),
        }
    }

    /// Creates an instance of [`MsgRefusePostOwnerTransferRequest`].
    /// * `subspace_id` - Id of the subspace holding the post for which the request will be refused.
    /// * `post_id` - Id of the post for which the request will be refused.
    /// * `receiver` - Address of the request receiver.
    pub fn refuse_post_owner_transfer_request(
        subspace_id: u64,
        post_id: u64,
        receiver: Addr,
    ) -> MsgRefusePostOwnerTransferRequest {
        MsgRefusePostOwnerTransferRequest {
            subspace_id,
            post_id,
            receiver: receiver.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cosmos_types::Timestamp;
    use chrono::DateTime;

    #[test]
    fn test_create_post() {
        let msg = PostsMsg::create_post(
            1,
            1,
            Some("1"),
            "test",
            None,
            vec![],
            vec![
                AttachmentContent::Media(Media {
                    uri: "ftp://domain.io/image.png".into(),
                    mime_type: "image/png".into(),
                }),
                AttachmentContent::Poll(Poll {
                    question: "questions?".into(),
                    provided_answers: vec![poll::ProvidedAnswer {
                        text: "Answer 1".into(),
                        attachments: vec![],
                    }],
                    end_date: Some(Timestamp::from(DateTime::from(
                        DateTime::parse_from_rfc3339("2140-01-01T10:00:20.021Z").unwrap(),
                    ))),
                    allows_multiple_answers: false,
                    allows_answer_edits: false,
                    final_tally_results: None,
                }),
            ],
            Addr::unchecked("user"),
            Some(1),
            ReplySetting::Everyone,
            vec![],
        );

        let expected = MsgCreatePost {
            subspace_id: 1,
            section_id: 1,
            external_id: "1".into(),
            text: "test".into(),
            entities: None,
            tags: vec![],
            attachments: vec![
                AttachmentContent::Media(Media {
                    uri: "ftp://domain.io/image.png".into(),
                    mime_type: "image/png".into(),
                })
                .into(),
                AttachmentContent::Poll(Poll {
                    question: "questions?".into(),
                    provided_answers: vec![poll::ProvidedAnswer {
                        text: "Answer 1".into(),
                        attachments: vec![],
                    }],
                    end_date: Some(Timestamp::from(DateTime::from(
                        DateTime::parse_from_rfc3339("2140-01-01T10:00:20.021Z").unwrap(),
                    ))),
                    allows_multiple_answers: false,
                    allows_answer_edits: false,
                    final_tally_results: None,
                })
                .into(),
            ],
            author: "user".into(),
            conversation_id: 1,
            reply_settings: ReplySetting::Everyone.into(),
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
            vec![],
            Addr::unchecked("user"),
        );

        let expected = MsgEditPost {
            subspace_id: 1,
            post_id: 1,
            text: "new text".into(),
            entities: None,
            tags: vec![],
            editor: "user".into(),
        };

        assert_eq!(expected, msg)
    }

    #[test]
    fn test_edit_post_without_new_text() {
        let msg = PostsMsg::edit_post(1, 1, None, None, vec![], Addr::unchecked("user"));

        let expected = MsgEditPost {
            subspace_id: 1,
            post_id: 1,
            text: "[do-not-modify]".into(),
            entities: None,
            tags: vec![],
            editor: "user".into(),
        };

        assert_eq!(expected, msg)
    }

    #[test]
    fn test_delete_post() {
        let msg = PostsMsg::delete_post(1, 1, Addr::unchecked("user"));

        let expected = MsgDeletePost {
            subspace_id: 1,
            post_id: 1,
            signer: "user".into(),
        };

        assert_eq!(expected, msg)
    }

    #[test]
    fn test_add_post_attachment() {
        let msg = PostsMsg::add_post_attachment(
            1,
            1,
            AttachmentContent::Media(Media {
                uri: "ftp://domain.io/image.png".into(),
                mime_type: "image/png".into(),
            }),
            Addr::unchecked("user"),
        );

        let expected = MsgAddPostAttachment {
            subspace_id: 1,
            post_id: 1,
            content: Some(
                AttachmentContent::Media(Media {
                    uri: "ftp://domain.io/image.png".into(),
                    mime_type: "image/png".into(),
                })
                .into(),
            ),
            editor: "user".into(),
        };

        assert_eq!(expected, msg)
    }

    #[test]
    fn test_remove_post_attachment() {
        let msg = PostsMsg::remove_post_attachment(1, 1, 1, Addr::unchecked("user"));

        let expected = MsgRemovePostAttachment {
            subspace_id: 1,
            post_id: 1,
            attachment_id: 1,
            editor: "user".into(),
        };

        assert_eq!(expected, msg)
    }

    #[test]
    fn test_answer_poll() {
        let msg = PostsMsg::answer_poll(1, 1, 1, vec![1], Addr::unchecked("user"));

        let expected = MsgAnswerPoll {
            subspace_id: 1,
            post_id: 1,
            poll_id: 1,
            answers_indexes: vec![1],
            signer: "user".into(),
        };

        assert_eq!(expected, msg)
    }

    #[test]
    fn test_move_post() {
        let msg = PostsMsg::move_post(1, 1, 2, 0, Addr::unchecked("user"));

        let expected = MsgMovePost {
            subspace_id: 1,
            post_id: 1,
            target_subspace_id: 2,
            target_section_id: 0,
            owner: "user".into(),
        };

        assert_eq!(expected, msg)
    }

    #[test]
    fn test_request_post_owner_transfer() {
        let msg = PostsMsg::request_post_owner_transfer(
            1,
            1,
            Addr::unchecked("receiver"),
            Addr::unchecked("sender"),
        );

        let expected = MsgRequestPostOwnerTransfer {
            subspace_id: 1,
            post_id: 1,
            receiver: "receiver".into(),
            sender: "sender".into(),
        };

        assert_eq!(expected, msg)
    }

    #[test]
    fn test_cancel_post_owner_transfer_request() {
        let msg = PostsMsg::cancel_post_owner_transfer_request(1, 1, Addr::unchecked("sender"));

        let expected = MsgCancelPostOwnerTransferRequest {
            subspace_id: 1,
            post_id: 1,
            sender: "sender".into(),
        };

        assert_eq!(expected, msg)
    }

    #[test]
    fn test_accept_post_owner_transfer_request() {
        let msg = PostsMsg::accept_post_owner_transfer_request(1, 1, Addr::unchecked("receiver"));

        let expected = MsgAcceptPostOwnerTransferRequest {
            subspace_id: 1,
            post_id: 1,
            receiver: "receiver".into(),
        };

        assert_eq!(expected, msg)
    }

    #[test]
    fn test_refuse_post_owner_transfer_request() {
        let msg = PostsMsg::refuse_post_owner_transfer_request(1, 1, Addr::unchecked("receiver"));

        let expected = MsgRefusePostOwnerTransferRequest {
            subspace_id: 1,
            post_id: 1,
            receiver: "receiver".into(),
        };

        assert_eq!(expected, msg)
    }
}
