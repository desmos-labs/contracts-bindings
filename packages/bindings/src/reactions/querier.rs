//! Contains a querier to query data from the Desmos x/reactions module.

#[cfg(feature = "iterators")]
use crate::{
    iter::page_iterator::{Page, PageIterator},
    reactions::proto::{Reaction, RegisteredReaction},
};
#[cfg(feature = "iterators")]
use cosmwasm_std::Binary;

use crate::reactions::proto::*;
use crate::types::PageRequest;
use cosmwasm_std::{Addr, Empty, QuerierWrapper, StdResult};

/// Querier able to query data from the Desmos x/reactions module.
pub struct ReactionsQuerier<'a> {
    querier: crate::reactions::proto::ReactionsQuerier<'a, Empty>,
}

impl<'a> ReactionsQuerier<'a> {
    /// Create a new [ReactionsQuerier]
    ///
    /// # Example
    /// ```
    /// use std::ops::Deref;
    /// use cosmwasm_std::{DepsMut, MessageInfo};
    /// use desmos_bindings::reactions::querier::ReactionsQuerier;
    ///
    /// pub fn contract_action(deps: DepsMut, _: MessageInfo) {
    ///     let querier = ReactionsQuerier::new(&deps.querier);
    ///     let reactions_response = querier.query_reactions(1, 1, None, None);
    /// }
    /// ```
    pub fn new(querier: &'a QuerierWrapper<'a, Empty>) -> Self {
        Self {
            querier: crate::reactions::proto::ReactionsQuerier::new(querier),
        }
    }
}

impl<'a> ReactionsQuerier<'a> {
    /// Queries all the reactions created inside a post.
    ///
    /// * `subspace_id` - Id of the subspace where the post stored.
    /// * `post_id` - Id of the post to query the reactions for.
    /// * `user` - Optional address of the user to query the reactions for.
    /// * `pagination` - Optional pagination configs.
    pub fn query_reactions(
        &self,
        subspace_id: u64,
        post_id: u64,
        user: Option<Addr>,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryReactionsResponse> {
        self.querier.reactions(
            subspace_id,
            post_id,
            user.unwrap_or_else(|| Addr::unchecked("")).into(),
            pagination.map(Into::into),
        )
    }

    /// Gives an iterator to scan over a reactions created in a post
    ///
    /// * `subspace_id` - Id of the subspace where the post stored.
    /// * `post_id` - Id of the post to query the reactions for.
    /// * `user` - Optional address of the user to query the reactions for.
    /// * `page_size` - Size of the page requested to the chain.
    #[cfg(feature = "iterators")]
    pub fn iterate_reactions(
        &self,
        subspace_id: u64,
        post_id: u64,
        user: Option<Addr>,
        page_size: u64,
    ) -> PageIterator<Reaction, Binary> {
        PageIterator::new(
            Box::new(move |key, limit| {
                self.query_reactions(
                    subspace_id,
                    post_id,
                    user.clone(),
                    Some(PageRequest {
                        key,
                        limit: limit.into(),
                        reverse: false,
                        count_total: false,
                        offset: None,
                    }),
                )
                .map(|response| Page {
                    items: response.reactions,
                    next_page_key: response.pagination.and_then(|response| {
                        (!response.next_key.is_empty()).then_some(Binary::from(response.next_key))
                    }),
                })
            }),
            page_size,
        )
    }

    /// Queries a reaction with given id.
    ///
    /// * `subspace_id` - Id of the subspace where the post stored.
    /// * `post_id` - Id of the post to query the reactions for.
    /// * `reaction_id` - Id of the reaction to query.
    /// * `pagination` - Optional pagination configs.
    pub fn query_reaction(
        &self,
        subspace_id: u64,
        post_id: u64,
        reaction_id: u32,
    ) -> StdResult<QueryReactionResponse> {
        self.querier.reaction(subspace_id, post_id, reaction_id)
    }

    /// Queries all the reactions registered inside a subspace.
    ///
    /// * `subspace_id` - Id of the subspace to query the registered reactions for.
    /// * `pagination` - Optional pagination configs.
    pub fn query_registered_reactions(
        &self,
        subspace_id: u64,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryRegisteredReactionsResponse> {
        self.querier
            .registered_reactions(subspace_id, pagination.map(Into::into))
    }

    /// Gives an iterator to scan over reactions registered in a subspace
    ///
    /// * `subspace_id` - Id of the subspace to query the registered reactions for.
    /// * `page_size` - Size of the page requested to the chain.
    #[cfg(feature = "iterators")]
    pub fn iterate_registered_reactions(
        &self,
        subspace_id: u64,
        page_size: u64,
    ) -> PageIterator<RegisteredReaction, Binary> {
        PageIterator::new(
            Box::new(move |key, limit| {
                self.query_registered_reactions(
                    subspace_id,
                    Some(PageRequest {
                        key,
                        limit: limit.into(),
                        reverse: false,
                        count_total: false,
                        offset: None,
                    }),
                )
                .map(|response| Page {
                    items: response.registered_reactions,
                    next_page_key: response.pagination.and_then(|response| {
                        (!response.next_key.is_empty()).then_some(Binary::from(response.next_key))
                    }),
                })
            }),
            page_size,
        )
    }

    /// Queries the reaction registered with given id.
    ///
    /// * `subspace_id` - Id of the subspace to query the registered reaction for.
    /// * `reaction_id` - Id of the registered reaction to query.
    pub fn query_registered_reaction(
        &self,
        subspace_id: u64,
        reaction_id: u32,
    ) -> StdResult<QueryRegisteredReactionResponse> {
        self.querier.registered_reaction(subspace_id, reaction_id)
    }

    /// Queries the reactions parameters inside the given subspace.
    ///
    /// * `subspace_id` - Id of the subspace to query the reactions parameters for.
    pub fn query_reactions_params(
        &self,
        subspace_id: u64,
    ) -> StdResult<QueryReactionsParamsResponse> {
        self.querier.reactions_params(subspace_id)
    }
}

#[cfg(test)]
mod tests {}
