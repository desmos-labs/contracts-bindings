//! Contains a querier to query data from the Desmos x/relationships module.

#[cfg(feature = "iterators")]
use crate::{
    iter::page_iterator::{Page, PageIterator},
    relationships::models::{Relationship, UserBlock},
};
#[cfg(feature = "iterators")]
use cosmwasm_std::Binary;

use crate::relationships::proto::*;
use crate::types::PageRequest;
use cosmwasm_std::{Addr, Empty, QuerierWrapper, StdResult};

/// Querier able to query data from the Desmos x/relationships module.
pub struct RelationshipsQuerier<'a> {
    querier: crate::relationships::proto::RelationshipsQuerier<'a, Empty>,
}

impl<'a> RelationshipsQuerier<'a> {
    /// Creates a new instance of [`RelationshipsQuerier`].
    ///
    /// # Example
    /// ```
    /// use std::ops::Deref;
    /// use cosmwasm_std::{DepsMut, MessageInfo};
    /// use desmos_bindings::relationships::querier::RelationshipsQuerier;
    ///
    /// pub fn contract_action(deps: DepsMut, _: MessageInfo) {
    ///     let querier = RelationshipsQuerier::new(&deps.querier);
    /// }
    /// ```
    pub fn new(querier: &'a QuerierWrapper<'a, Empty>) -> Self {
        Self {
            querier: crate::relationships::proto::RelationshipsQuerier::new(querier),
        }
    }

    /// Queries the relationships inside a subspaces.
    ///
    /// * `subspace_id` - Subspace to query the relationships for.
    /// * `user` - Optional address of the user for which to query the relationships.
    /// * `counterparty` - Optional address of the counterparty of the relationships (used only if the
    /// `user` is provided).
    /// * `pagination` - Optional pagination configs.
    pub fn query_relationships(
        &self,
        subspace_id: u64,
        user: Option<Addr>,
        counterparty: Option<Addr>,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryRelationshipsResponse> {
        Ok(self.querier.relationships(
            subspace_id,
            user.unwrap_or_else(|| Addr::unchecked("")).into(),
            counterparty.unwrap_or_else(|| Addr::unchecked("")).into(),
            pagination.map(Into::into),
        )?)
    }

    /// Gives an iterator to scan over a user's relationships created in a subspace or
    /// all the relationships created in a subspace.
    ///
    /// * `subspace_id` - Subspace to query the relationships for.
    /// * `user` - Optional address of the user for which to query the relationships.
    /// * `page_size` - Size of the page requested to the chain.
    #[cfg(feature = "iterators")]
    pub fn iterate_relationships(
        &self,
        subspace_id: u64,
        user: Option<Addr>,
        page_size: u64,
    ) -> PageIterator<Relationship, Binary> {
        PageIterator::new(
            Box::new(move |key, limit| {
                self.query_relationships(
                    subspace_id,
                    user.clone(),
                    None,
                    Some(PageRequest {
                        key,
                        limit: limit.into(),
                        reverse: false,
                        count_total: false,
                        offset: None,
                    }),
                )
                .map(|response| Page {
                    items: response.relationships,
                    next_page_key: response
                        .pagination
                        .and_then(|pagination| pagination.next_key),
                })
            }),
            page_size,
        )
    }

    /// Queries the blocks created inside a subspace.
    ///
    /// * `subspace_id` - Subspace to query the blocks for.
    /// * `blocker` - Optional address of the blocker to query the blocks for.
    /// * `blocked` - Optional address of the blocked user to query the block for (used only if
    /// the `blocker` is provided).
    /// * `pagination` - Optional pagination configs.
    pub fn query_blocks(
        &self,
        subspace_id: u64,
        blocker: Option<Addr>,
        blocked: Option<Addr>,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryBlocksResponse> {
        Ok(self.querier.blocks(
            subspace_id,
            blocker.unwrap_or_else(|| Addr::unchecked("")).into(),
            blocked.unwrap_or_else(|| Addr::unchecked("")).into(),
            pagination.map(Into::into),
        )?)
    }

    /// Gives an iterator to scan over the users blocked from a specific user in a subspace or
    /// all the blocks performed from the users in a subspace.
    ///
    /// * `subspace_id` - Subspace to query the blocks for.
    /// * `blocker` - Optional address of the blocker to query the blocks for.
    /// * `page_size` - Size of the page requested to the chain.
    #[cfg(feature = "iterators")]
    pub fn iterate_blocks(
        &self,
        subspace_id: u64,
        blocker: Option<Addr>,
        page_size: u64,
    ) -> PageIterator<UserBlock, Binary> {
        PageIterator::new(
            Box::new(move |key, limit| {
                self.query_blocks(
                    subspace_id,
                    blocker.clone(),
                    None,
                    Some(PageRequest {
                        key,
                        limit: limit.into(),
                        reverse: false,
                        count_total: false,
                        offset: None,
                    }),
                )
                .map(|response| Page {
                    items: response.blocks,
                    next_page_key: response
                        .pagination
                        .and_then(|pagination| pagination.next_key),
                })
            }),
            page_size,
        )
    }
}

#[cfg(test)]
mod tests {}
