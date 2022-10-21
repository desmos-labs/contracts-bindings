//! Contains the query actions that can be sent to the chain in order to query data related
//! to the x/posts module.

use crate::posts::models_query::*;
use crate::types::PageRequest;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint64};

/// Query messages that can be sent to the x/relationships module.
#[cw_serde]
#[derive(QueryResponses)]
pub enum PostsQuery {
    /// Queries all the posts inside a given subspace.
    #[returns(QuerySubspacePostsResponse)]
    SubspacePosts {
        /// Id of the subspace to query the posts for.
        subspace_id: Uint64,
        /// Pagination configs.
        pagination: Option<PageRequest>,
    },
    /// Queries all the posts inside a given section.
    #[returns(QuerySectionPostsResponse)]
    SectionPosts {
        /// Id of the subspace to query the posts for.
        subspace_id: Uint64,
        /// Id of the section to query the posts for.
        section_id: u32,
        /// Pagination configs.
        pagination: Option<PageRequest>,
    },
    /// Queries for a single post inside a given subspace.
    #[returns(QueryPostResponse)]
    Post {
        /// Id of the subspace where the post is stored.
        subspace_id: Uint64,
        /// Id of the post to query for.
        post_id: Uint64,
    },
    /// Queries the attachments of the post having the given id.
    #[returns(QueryPostAttachmentsResponse)]
    PostAttachments {
        /// Id of the subspace where the post is stored.
        subspace_id: Uint64,
        /// Id of the post to query the attachments for.
        post_id: Uint64,
        /// Pagination configs.
        pagination: Option<PageRequest>,
    },
    /// Queries the answers for the poll having the given id.
    #[returns(QueryPollAnswersResponse)]
    PollAnswers {
        /// Id of the subspace where the post is stored.
        subspace_id: Uint64,
        /// Id of the post that holds the poll.
        post_id: Uint64,
        /// Id of the poll to query the answers for.
        poll_id: u32,
        /// Address of the user to query the responses for.
        user: Option<Addr>,
        /// Pagination configs.
        pagination: Option<PageRequest>,
    },
}
