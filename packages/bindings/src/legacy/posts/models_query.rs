//! Contains structures returned from the [PostsQuerier<'a>](crate::legacy::posts::querier::PostsQuerier).

use crate::legacy::posts::models::{Attachment, Post, UserAnswer};
use crate::legacy::types::PageResponse;
use cosmwasm_schema::cw_serde;

/// Response to [`PostsQuery::SubspacePosts`](crate::legacy::posts::query::PostsQuery::SubspacePosts).
#[cw_serde]
pub struct QuerySubspacePostsResponse {
    /// Queried posts.
    pub posts: Vec<Post>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`PostsQuery::SectionPosts`](crate::legacy::posts::query::PostsQuery::SectionPosts).
#[cw_serde]
pub struct QuerySectionPostsResponse {
    /// Queried posts.
    pub posts: Vec<Post>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`PostsQuery::Post`](crate::legacy::posts::query::PostsQuery::Post).
#[cw_serde]
pub struct QueryPostResponse {
    /// Queried post.
    pub post: Post,
}

/// Response to [`PostsQuery::PostAttachments`](crate::legacy::posts::query::PostsQuery::PostAttachments).
#[cw_serde]
pub struct QueryPostAttachmentsResponse {
    /// Queried attachments.
    pub attachments: Vec<Attachment>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`PostsQuery::PollAnswers`](crate::legacy::posts::query::PostsQuery::PollAnswers).
#[cw_serde]
pub struct QueryPollAnswersResponse {
    /// Queried answers.
    pub answers: Vec<UserAnswer>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}
