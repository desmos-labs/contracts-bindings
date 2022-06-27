//! Contains some useful mocks of the Desmos x/posts module's types made to be used in any test.

use crate::posts::models::{Attachment, Post, PostAttachment, ReplySetting, UserAnswer};
use crate::posts::models_query::{
    QueryPollAnswersResponse, QueryPostAttachmentsResponse, QueryPostResponse,
    QuerySectionPostsResponse, QuerySubspacePostsResponse,
};
use crate::posts::query::PostsQuery;
use cosmwasm_std::{to_binary, Addr, Binary, ContractResult, Uint64};

/// Functions that mocks the posts inside a subspace.
pub fn get_mocked_subspace_posts(subspace_id: &Uint64) -> Vec<Post> {
    vec![
        Post {
            subspace_id: *subspace_id,
            section_id: 0,
            eternal_id: None,
            text: None,
            entities: None,
            author: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
            id: Uint64::new(0),
            referenced_posts: vec![],
            reply_settings: ReplySetting::Everyone,
            creation_date: "".to_string(),
            conversation_id: None,
            last_edit_date: None,
        },
        Post {
            subspace_id: *subspace_id,
            section_id: 0,
            eternal_id: None,
            text: None,
            entities: None,
            author: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
            id: Uint64::new(1),
            referenced_posts: vec![],
            reply_settings: ReplySetting::Everyone,
            creation_date: "".to_string(),
            conversation_id: None,
            last_edit_date: None,
        },
    ]
}

/// Functions that mocks the posts inside a section.
pub fn get_mocked_section_posts(subspace_id: &Uint64, section_id: &u32) -> Vec<Post> {
    vec![
        Post {
            subspace_id: *subspace_id,
            section_id: *section_id,
            eternal_id: None,
            text: None,
            entities: None,
            author: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
            id: Uint64::new(0),
            referenced_posts: vec![],
            reply_settings: ReplySetting::Everyone,
            creation_date: "".to_string(),
            conversation_id: None,
            last_edit_date: None,
        },
        Post {
            subspace_id: *subspace_id,
            section_id: *section_id,
            eternal_id: None,
            text: None,
            entities: None,
            author: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
            id: Uint64::new(1),
            referenced_posts: vec![],
            reply_settings: ReplySetting::Everyone,
            creation_date: "".to_string(),
            conversation_id: None,
            last_edit_date: None,
        },
    ]
}

/// Functions that mocks the post returned from the query post request.
pub fn get_mocked_post(post_id: Uint64, subspace_id: Uint64) -> Post {
    Post {
        id: post_id,
        subspace_id,
        section_id: 0,
        eternal_id: None,
        text: None,
        entities: None,
        author: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
        conversation_id: None,
        referenced_posts: vec![],
        reply_settings: ReplySetting::Unspecified,
        creation_date: "".to_string(),
        last_edit_date: None,
    }
}

/// Functions that mocks the attachments of a post.
pub fn get_mocked_post_attachments(subspace_id: &Uint64, post_id: &Uint64) -> Vec<Attachment> {
    vec![
        Attachment {
            subspace_id: *subspace_id,
            section_id: 0,
            post_id: *post_id,
            id: 0,
            content: PostAttachment::Media {
                uri: "ftp://domain.io/image.png".to_string(),
                mime_type: "image/png".to_string(),
            }
            .into(),
        },
        Attachment {
            subspace_id: *subspace_id,
            section_id: 0,
            post_id: *post_id,
            id: 1,
            content: PostAttachment::Media {
                uri: "ftp://domain.io/image2.png".to_string(),
                mime_type: "image/png".to_string(),
            }
            .into(),
        },
    ]
}

/// Functions that mocks the poll answers.
pub fn get_mocked_poll_answers(
    subspace_id: &Uint64,
    post_id: &Uint64,
    poll_id: &u32,
    user: &Option<Addr>,
) -> Vec<UserAnswer> {
    vec![UserAnswer {
        subspace_id: *subspace_id,
        section_id: 0,
        post_id: *post_id,
        poll_id: *poll_id,
        answers_indexes: vec![0],
        user: user.as_ref().map_or(
            Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
            |addr| addr.clone(),
        ),
    }]
}

/// Functions that mocks the posts query responses.
pub fn mock_posts_query_response(query: &PostsQuery) -> ContractResult<Binary> {
    let response = match query {
        PostsQuery::SubspacePosts { subspace_id, .. } => to_binary(&QuerySubspacePostsResponse {
            posts: get_mocked_subspace_posts(subspace_id),
            pagination: None,
        }),
        PostsQuery::SectionPosts {
            subspace_id,
            section_id,
            ..
        } => to_binary(&QuerySectionPostsResponse {
            posts: get_mocked_section_posts(subspace_id, section_id),
            pagination: None,
        }),
        PostsQuery::Post {
            subspace_id,
            post_id,
            ..
        } => to_binary(&QueryPostResponse {
            post: get_mocked_post(*subspace_id, *post_id),
        }),
        PostsQuery::PostAttachments {
            subspace_id,
            post_id,
            ..
        } => to_binary(&QueryPostAttachmentsResponse {
            attachments: get_mocked_post_attachments(subspace_id, post_id),
            pagination: None,
        }),
        PostsQuery::PollAnswers {
            subspace_id,
            post_id,
            poll_id,
            user,
            ..
        } => to_binary(&QueryPollAnswersResponse {
            answers: get_mocked_poll_answers(subspace_id, post_id, poll_id, user),
            pagination: None,
        }),
    };
    response.into()
}
