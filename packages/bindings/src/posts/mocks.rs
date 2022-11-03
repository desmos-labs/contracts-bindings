use crate::posts::proto::{
    Attachment, Media, Post, QueryPollAnswersResponse, QueryPostAttachmentsResponse,
    QueryPostResponse, QuerySectionPostsResponse, QuerySubspacePostsResponse, ReplySetting,
    UserAnswer,
};
use crate::posts::types::AttachmentContent;
use chrono::DateTime;
use cosmwasm_std::Addr;
use desmos_std::shim::Timestamp;

pub const AUTHOR: &str = "author";
pub const ANSWERER: &str = "answerer";

/// Struct that contains some utility methods to mock data of the Desmos
/// x/posts module.
pub struct MockPostsQueries {}
impl MockPostsQueries {
    /// Function that mock a post.
    pub fn get_mocked_post(subspace_id: u64, section_id: u32, post_id: u64) -> Post {
        Post {
            subspace_id,
            section_id,
            id: post_id,
            external_id: "".into(),
            text: "test".into(),
            entities: None,
            tags: vec!["hello".into(), "world".into()],
            author: "author".into(),
            conversation_id: 0,
            referenced_posts: vec![],
            reply_settings: ReplySetting::Everyone.into(),
            creation_date: Some(Timestamp::from(DateTime::from(
                DateTime::parse_from_rfc3339("2011-11-11T11:11:11.111Z").unwrap(),
            ))),
            last_edited_date: None,
        }
    }
    /// Function that mocks a attachment.
    pub fn get_mocked_attachment(subspace_id: u64, post_id: u64, attachment_id: u32) -> Attachment {
        Attachment {
            subspace_id,
            post_id,
            id: attachment_id,
            content: Some(
                AttachmentContent::Media(Media {
                    uri: "ftp://domain.io/image.png".into(),
                    mime_type: "image/png".into(),
                })
                .into(),
            ),
        }
    }
    /// Function that mocks a poll answers.
    pub fn get_mocked_user_answer(
        subspace_id: u64,
        post_id: u64,
        poll_id: u32,
        user: Addr,
    ) -> UserAnswer {
        UserAnswer {
            subspace_id,
            post_id,
            poll_id,
            answers_indexes: vec![0],
            user: user.into(),
        }
    }
    /// Function that mocks the posts inside a subspace.
    pub fn get_mocked_subspace_posts(subspace_id: u64) -> Vec<Post> {
        vec![
            Self::get_mocked_post(subspace_id, 0, 1),
            Self::get_mocked_post(subspace_id, 0, 2),
        ]
    }
    /// Function that mocks a subspace posts response.
    pub fn get_mocked_subspace_posts_response() -> QuerySubspacePostsResponse {
        QuerySubspacePostsResponse {
            posts: Self::get_mocked_subspace_posts(1),
            pagination: None,
        }
    }
    /// Functions that mocks the posts inside a section.
    pub fn get_mocked_section_posts(subspace_id: u64, section_id: u32) -> Vec<Post> {
        vec![
            Self::get_mocked_post(subspace_id, section_id, 1),
            Self::get_mocked_post(subspace_id, section_id, 2),
        ]
    }
    /// Function that mocks a section posts response.
    pub fn get_mocked_section_posts_response() -> QuerySectionPostsResponse {
        QuerySectionPostsResponse {
            posts: Self::get_mocked_section_posts(1, 1),
            pagination: None,
        }
    }
    /// Function that mocks a post response.
    pub fn get_mocked_post_response() -> QueryPostResponse {
        QueryPostResponse {
            post: Some(Self::get_mocked_post(1, 0, 1)),
        }
    }
    /// Function that mocks post attachments.
    pub fn get_mocked_post_attachments(subspace_id: u64, post_id: u64) -> Vec<Attachment> {
        vec![
            Self::get_mocked_attachment(subspace_id, post_id, 1),
            Self::get_mocked_attachment(subspace_id, post_id, 2),
        ]
    }
    /// Function that mocks a post attachments response
    pub fn get_mocked_post_attachments_response() -> QueryPostAttachmentsResponse {
        QueryPostAttachmentsResponse {
            attachments: Self::get_mocked_post_attachments(1, 1),
            pagination: None,
        }
    }
    pub fn get_mocked_poll_answers(
        subspace_id: u64,
        post_id: u64,
        poll_id: u32,
    ) -> Vec<UserAnswer> {
        vec![Self::get_mocked_user_answer(
            subspace_id,
            post_id,
            poll_id,
            Addr::unchecked(ANSWERER),
        )]
    }
    pub fn get_mocked_poll_answers_response() -> QueryPollAnswersResponse {
        QueryPollAnswersResponse {
            answers: Self::get_mocked_poll_answers(1, 1, 1),
            pagination: None,
        }
    }
}
