//! Contains structures returned from the [PostsQuerier<'a>](crate::posts::querier::PostsQuerier).

use crate::posts::models::{Attachment, Post, UserAnswer};
use crate::types::PageResponse;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Response to [`PostsQuery::SubspacePosts`](crate::posts::query::PostsQuery::SubspacePosts).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QuerySubspacePostsResponse {
    /// Queried posts.
    pub posts: Vec<Post>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`PostsQuery::SectionPosts`](crate::posts::query::PostsQuery::SectionPosts).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QuerySectionPostsResponse {
    /// Queried posts.
    pub posts: Vec<Post>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`PostsQuery::Post`](crate::posts::query::PostsQuery::Post).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryPostResponse {
    /// Queried post.
    pub post: Post,
}

/// Response to [`PostsQuery::PostAttachments`](crate::posts::query::PostsQuery::PostAttachments).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryPostAttachmentsResponse {
    /// Queried attachments.
    pub attachments: Vec<Attachment>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`PostsQuery::PollAnswers`](crate::posts::query::PostsQuery::PollAnswers).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryPollAnswersResponse {
    /// Queried answers.
    pub answers: Vec<UserAnswer>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}
