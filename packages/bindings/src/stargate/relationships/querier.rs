//! Contains a querier to query data from the Desmos x/relationships module.

#[cfg(feature = "iterators")]
use crate::{
    iter::page_iterator::{Page, PageIterator},
    stargate::relationships::types::{Relationship, UserBlock},
};
#[cfg(feature = "iterators")]
use cosmwasm_std::Binary;

use crate::stargate::relationships::types::*;
use crate::stargate::types::PageRequest;
use cosmwasm_std::{Addr, Empty, QuerierWrapper, StdResult};

/// Querier allows to query data from the Desmos x/relationships module.
pub struct RelationshipsQuerier<'a> {
    querier: crate::stargate::relationships::types::RelationshipsQuerier<'a, Empty>,
}

impl<'a> RelationshipsQuerier<'a> {
    /// Creates a new instance of [`RelationshipsQuerier`].
    ///
    /// # Example
    /// ```
    /// use cosmwasm_std::{DepsMut, MessageInfo};
    /// use desmos_bindings::stargate::relationships::querier::RelationshipsQuerier;
    ///
    /// pub fn contract_action(deps: DepsMut, _: MessageInfo) {
    ///     let querier = RelationshipsQuerier::new(&deps.querier);
    /// }
    /// ```
    pub fn new(querier: &'a QuerierWrapper<'a, Empty>) -> Self {
        Self {
            querier: crate::stargate::relationships::types::RelationshipsQuerier::new(querier),
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
        self.querier.relationships(
            subspace_id,
            user.unwrap_or_else(|| Addr::unchecked("")).into(),
            counterparty.unwrap_or_else(|| Addr::unchecked("")).into(),
            pagination.map(Into::into),
        )
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
                    next_page_key: response.pagination.and_then(|response| {
                        (!response.next_key.is_empty()).then_some(Binary::from(response.next_key))
                    }),
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
        self.querier.blocks(
            subspace_id,
            blocker.unwrap_or_else(|| Addr::unchecked("")).into(),
            blocked.unwrap_or_else(|| Addr::unchecked("")).into(),
            pagination.map(Into::into),
        )
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
                    next_page_key: response.pagination.and_then(|response| {
                        (!response.next_key.is_empty()).then_some(Binary::from(response.next_key))
                    }),
                })
            }),
            page_size,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stargate::mocks::mock_queriers::mock_desmos_dependencies;
    use crate::stargate::relationships::mocks::{MockRelationshipsQueries, MOCK_TARGET, MOCK_USER};
    use cosmwasm_std::Addr;

    #[test]
    fn test_query_relationships() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = RelationshipsQuerier::new(&deps.querier);
        let response = querier
            .query_relationships(
                0,
                Some(Addr::unchecked(MOCK_USER)),
                Some(Addr::unchecked(MOCK_TARGET)),
                None,
            )
            .unwrap();
        let expected = MockRelationshipsQueries::get_mocked_relationships_response();
        assert_eq!(expected, response)
    }

    #[test]
    fn test_iterate_relationships() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = RelationshipsQuerier::new(&deps.querier);
        let mut it = querier.iterate_relationships(0, Some(Addr::unchecked("")), 10);
        let expected = MockRelationshipsQueries::get_mocked_relationships_response();
        assert_eq!(expected.relationships[0], it.next().unwrap().unwrap(),);
        assert!(it.next().is_none());
    }

    #[test]
    fn test_query_blocks() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = RelationshipsQuerier::new(&deps.querier);
        let response = querier
            .query_blocks(
                0,
                Some(Addr::unchecked(MOCK_USER)),
                Some(Addr::unchecked(MOCK_TARGET)),
                None,
            )
            .unwrap();
        let expected = MockRelationshipsQueries::get_mocked_blocks_response();
        assert_eq!(expected, response)
    }

    #[test]
    fn test_iterate_blocks() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = RelationshipsQuerier::new(&deps.querier);
        let mut it = querier.iterate_blocks(0, Some(Addr::unchecked("")), 10);
        let expected = MockRelationshipsQueries::get_mocked_blocks_response();
        assert_eq!(expected.blocks[0], it.next().unwrap().unwrap(),);
        assert!(it.next().is_none());
    }
}
