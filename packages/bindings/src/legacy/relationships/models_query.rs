//! Contains the types definitions of all the responses to the x/relationships query messages.

use crate::legacy::{
    relationships::models::{Relationship, UserBlock},
    types::PageResponse,
};
use cosmwasm_schema::cw_serde;

/// Response to the [`RelationshipsQuery::Relationships`](crate::legacy::relationships::query::RelationshipsQuery::Relationships).
#[cw_serde]
pub struct QueryRelationshipsResponse {
    /// The queried relationships.
    pub relationships: Vec<Relationship>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to the [`RelationshipsQuery::Blocks`](crate::legacy::relationships::query::RelationshipsQuery::Blocks).
#[cw_serde]
pub struct QueryBlocksResponse {
    /// The queried blocks.
    pub blocks: Vec<UserBlock>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}
