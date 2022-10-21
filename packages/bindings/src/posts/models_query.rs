//! Contains structures returned from the [PostsQuerier<'a>](crate::posts::querier::PostsQuerier).

use crate::posts::models::{Attachment, Post, UserAnswer};
use crate::types::PageResponse;
use cosmwasm_schema::cw_serde;

/// Response to [`PostsQuery::SubspacePosts`](crate::posts::query::PostsQuery::SubspacePosts).
#[cw_serde]
pub struct QuerySubspacePostsResponse {
    /// Queried posts.
    pub posts: Vec<Post>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`PostsQuery::SectionPosts`](crate::posts::query::PostsQuery::SectionPosts).
#[cw_serde]
pub struct QuerySectionPostsResponse {
    /// Queried posts.
    pub posts: Vec<Post>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`PostsQuery::Post`](crate::posts::query::PostsQuery::Post).
#[cw_serde]
pub struct QueryPostResponse {
    /// Queried post.
    pub post: Post,
}

/// Response to [`PostsQuery::PostAttachments`](crate::posts::query::PostsQuery::PostAttachments).
#[cw_serde]
pub struct QueryPostAttachmentsResponse {
    /// Queried attachments.
    pub attachments: Vec<Attachment>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`PostsQuery::PollAnswers`](crate::posts::query::PostsQuery::PollAnswers).
#[cw_serde]
pub struct QueryPollAnswersResponse {
    /// Queried answers.
    pub answers: Vec<UserAnswer>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}
