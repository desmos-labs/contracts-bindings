//! Contains a querier to query data from the Desmos x/reactions module.

#[cfg(feature = "iterators")]
use crate::{
    iter::page_iterator::{Page, PageIterator},
    reactions::models::{Reaction, RegisteredReaction},
};
#[cfg(feature = "iterators")]
use cosmwasm_std::Binary;

use crate::{
    query::DesmosQuery,
    reactions::{
        query::ReactionsQuery,
        models_query::{
            QueryReactionsResponse, QueryReactionResponse,
            QueryRegisteredReactionsResponse, QueryRegisteredReactionResponse,
            QueryReactionsParamsResponse,
        },
    },
    types::PageRequest,
};
use cosmwasm_std::{Addr, Querier, QuerierWrapper, StdResult};

/// Querier able to query data from the Desmos x/reactions module.
pub struct ReactionsQuerier<'a> {
    querier: QuerierWrapper<'a, DesmosQuery>,
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
    ///     let querier = ReactionsQuerier::new(deps.querier.deref());
    ///     let reactions_response = querier.query_reactions(1, 1, None, None);
    /// }
    /// ```
    pub fn new(querier: &'a dyn Querier) -> Self {
        Self {
            querier: QuerierWrapper::<'a, DesmosQuery>::new(querier),
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
        let request = DesmosQuery::from(ReactionsQuery::Reactions { 
            subspace_id: subspace_id.into(), 
            post_id: post_id.into(), 
            user: user, 
            pagination: pagination,
        });
        let res: QueryReactionsResponse = self.querier.query(&request.into())?;
        Ok(res)
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
                    next_page_key: response
                        .pagination
                        .map_or(None, |pagination| pagination.next_key),
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
        let request = DesmosQuery::from(ReactionsQuery::Reaction { 
            subspace_id: subspace_id.into(), 
            post_id: post_id.into(), 
            reaction_id: reaction_id, 
        });
        let res: QueryReactionResponse = self.querier.query(&request.into())?;
        Ok(res)
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
        let request = DesmosQuery::from(ReactionsQuery::RegisteredReactions {
            subspace_id: subspace_id.into(),
            pagination: pagination,
        });
        let res: QueryRegisteredReactionsResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    /// Gives an iterator to scan over reactions registered in a subspace
    ///
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
                    next_page_key: response
                        .pagination
                        .map_or(None, |pagination| pagination.next_key),
                })
            }),
            page_size,
        )
    }

    /// Queries the reaction registered with given id.
    ///
    /// * `subspace_id` - Id of the subspace to query the registered reaction for.
    /// * `reaction_id` - Id of the registered reaction to query.
    /// * `pagination` - Optional pagination configs.
    pub fn query_registered_reaction(
        &self,
        subspace_id: u64,
        reaction_id: u32
    ) -> StdResult<QueryRegisteredReactionResponse> {
        let request = DesmosQuery::from(ReactionsQuery::RegisteredReaction {
            subspace_id: subspace_id.into(),
            reaction_id: reaction_id,
        });
        let res: QueryRegisteredReactionResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    /// Queries the reactions parameters inside the given subspace.
    ///
    /// * `subspace_id` - Id of the subspace to query the reactions parameters for.
    pub fn query_reactions_params(
        &self,
        subspace_id: u64,
    ) -> StdResult<QueryReactionsParamsResponse> {
        let request = DesmosQuery::from(ReactionsQuery::ReactionsParams {
            subspace_id: subspace_id.into(),
        });
        let res: QueryReactionsParamsResponse = self.querier.query(&request.into())?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::mock_dependencies_with_custom_querier;
    use crate::reactions::mocks::MockReactionsQueries;
    use std::ops::Deref;

    #[test]
    fn test_query_reactions() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = ReactionsQuerier::new(deps.querier.deref());
        let response = querier.query_reactions(1, 1, None, Default::default());
        let expected = QueryReactionsResponse {
            reactions: vec![MockReactionsQueries::get_mock_reaction()],
            pagination: Default::default(),
        };
        assert_eq!(response.ok(), Some(expected));
    }

    #[test]
    fn test_query_reaction() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = ReactionsQuerier::new(deps.querier.deref());
        let response = querier.query_reaction(1, 1, 1);
        let expected = QueryReactionResponse {
            reaction: MockReactionsQueries::get_mock_reaction(),
        };
        assert_eq!(response.ok(), Some(expected));
    }

    #[test]
    fn test_query_registered_reactions() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = ReactionsQuerier::new(deps.querier.deref());
        let response = querier.query_registered_reactions(1, Default::default());
        let expected = QueryRegisteredReactionsResponse {
            registered_reactions: vec![MockReactionsQueries::get_mock_registered_reaction()],
            pagination: Default::default(),
        };
        assert_eq!(response.ok(), Some(expected));
    }

    #[test]
    fn test_query_registered_reaction() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = ReactionsQuerier::new(deps.querier.deref());
        let response = querier.query_registered_reaction(1, 1);
        let expected = QueryRegisteredReactionResponse {
            registered_reaction: MockReactionsQueries::get_mock_registered_reaction(),
        };
        assert_eq!(response.ok(), Some(expected));
    }

    #[test]
    fn test_query_reactions_params() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = ReactionsQuerier::new(deps.querier.deref());
        let response = querier.query_reactions_params(1);
        let expected = QueryReactionsParamsResponse {
            params: MockReactionsQueries::get_mock_reactions_parameters(),
        };
        assert_eq!(response.ok(), Some(expected));
    }
}