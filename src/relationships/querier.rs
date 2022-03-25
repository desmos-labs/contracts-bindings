use crate::iter::page_iterator::{Page, PageIterator};
use crate::relationships::models::{Relationship, UserBlock};
use crate::{
    query::DesmosQuery,
    relationships::{
        models_query::{QueryBlocksResponse, QueryRelationshipsResponse},
        query::RelationshipsQuery,
    },
    types::PageRequest,
};
use cosmwasm_std::{Addr, Binary, Querier, QuerierWrapper, StdResult};

pub struct RelationshipsQuerier<'a> {
    querier: QuerierWrapper<'a, DesmosQuery>,
}

impl<'a> RelationshipsQuerier<'a> {
    pub fn new(querier: &'a dyn Querier) -> Self {
        Self {
            querier: QuerierWrapper::<'a, DesmosQuery>::new(querier),
        }
    }

    pub fn query_relationships(
        &self,
        subspace_id: u64,
        user: Option<Addr>,
        counterparty: Option<Addr>,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryRelationshipsResponse> {
        let request = DesmosQuery::Relationships(RelationshipsQuery::Relationships {
            subspace_id: subspace_id.into(),
            user,
            counterparty,
            pagination,
        });

        let res: QueryRelationshipsResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    /// Gives an iterator to scan over a user's relationships created in a subspace or
    /// all the relationships created in a subspace.
    ///
    /// * `subspace_id` - Subspace to query the relationships for.
    /// * `user` - Optional address of the user for which to query the relationships.
    /// * `page_size` - Size of the page requested to the chain.
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
                    next_page_key: response.pagination.next_key,
                })
            }),
            page_size,
        )
    }

    pub fn query_blocks(
        &self,
        subspace_id: u64,
        blocker: Option<Addr>,
        blocked: Option<Addr>,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryBlocksResponse> {
        let request = DesmosQuery::Relationships(RelationshipsQuery::Blocks {
            subspace_id: subspace_id.into(),
            blocker,
            blocked,
            pagination,
        });

        let res: QueryBlocksResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    /// Gives an iterator to scan over the users blocked from a specific user in a subspace or
    /// all the blocks performed from the users in a subspace.
    ///
    /// * `subspace_id` - Subspace to query the blocks for.
    /// * `blocker` - Optional address of the blocker to query the blocks for.
    /// * `page_size` - Size of the page requested to the chain.
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
                    next_page_key: response.pagination.next_key,
                })
            }),
            page_size,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        mock::mock_dependencies_with_custom_querier,
        relationships::{
            mock::MockRelationshipsQueries,
            models_query::{QueryBlocksResponse, QueryRelationshipsResponse},
            querier::RelationshipsQuerier,
        },
    };
    use cosmwasm_std::Addr;
    use std::ops::Deref;

    #[test]
    fn test_query_relationships() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let relationships_querier = RelationshipsQuerier::new(deps.querier.deref());

        let response = relationships_querier
            .query_relationships(
                0,
                Some(Addr::unchecked("")),
                Some(Addr::unchecked("")),
                None,
            )
            .unwrap();
        let expected = QueryRelationshipsResponse {
            relationships: vec![MockRelationshipsQueries::get_mock_relationship()],
            pagination: Default::default(),
        };

        assert_eq!(response, expected)
    }

    #[test]
    fn test_iterate_relationships() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let relationships_querier = RelationshipsQuerier::new(deps.querier.deref());

        let mut it = relationships_querier.iterate_relationships(0, Some(Addr::unchecked("")), 10);

        assert_eq!(
            it.next().unwrap().unwrap(),
            MockRelationshipsQueries::get_mock_relationship()
        );
        assert!(it.next().is_none());
    }

    #[test]
    fn test_query_blocks() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let relationships_querier = RelationshipsQuerier::new(deps.querier.deref());

        let response = relationships_querier
            .query_blocks(
                0,
                Some(Addr::unchecked("")),
                Some(Addr::unchecked("")),
                None,
            )
            .unwrap();
        let expected = QueryBlocksResponse {
            blocks: vec![MockRelationshipsQueries::get_mock_user_block()],
            pagination: Default::default(),
        };

        assert_eq!(response, expected)
    }

    #[test]
    fn test_iterate_blocks() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let relationships_querier = RelationshipsQuerier::new(deps.querier.deref());

        let mut it = relationships_querier.iterate_blocks(0, Some(Addr::unchecked("")), 10);

        assert_eq!(
            it.next().unwrap().unwrap(),
            MockRelationshipsQueries::get_mock_user_block()
        );
        assert!(it.next().is_none());
    }
}
