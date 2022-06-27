//! Contains the query actions that can be sent to the chain in order to query data related
//! to the x/posts module.

use crate::types::PageRequest;
use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Query messages that can be sent to the x/relationships module.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PostsQuery {
    /// Queries all the posts inside a given subspace.
    SubspacePosts {
        /// Id of the subspace to query the posts for.
        subspace_id: Uint64,
        /// Pagination configs.
        pagination: Option<PageRequest>,
    },
    /// Queries all the posts inside a given section.
    SectionPosts {
        /// Id of the subspace to query the posts for.
        subspace_id: Uint64,
        /// Id of the section to query the posts for.
        section_id: u32,
        /// Pagination configs.
        pagination: Option<PageRequest>,
    },
    /// Queries for a single post inside a given subspace.
    Post {
        /// Id of the subspace where the post is stored.
        subspace_id: Uint64,
        /// Id of the post to query for.
        post_id: Uint64,
    },
    /// Queries the attachments of the post having the given id.
    PostAttachments {
        /// Id of the subspace where the post is stored.
        subspace_id: Uint64,
        /// Id of the post to query the attachments for.
        post_id: Uint64,
        /// Pagination configs.
        pagination: Option<PageRequest>,
    },
    /// Queries the answers for the poll having the given id.
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
