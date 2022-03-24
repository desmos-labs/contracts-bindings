use cosmwasm_std::{Addr, Binary, Querier, QuerierWrapper, StdResult};

use crate::iter::page_iterator::{Page, PageIterator};
use crate::subspaces::models::{Subspace, UserGroup};
use crate::{
    query::DesmosQuery,
    subspaces::{
        query::SubspacesQuery,
        query_types::{
            QuerySubspaceResponse, QuerySubspacesResponse, QueryUserGroupMembersResponse,
            QueryUserGroupResponse, QueryUserGroupsResponse, QueryUserPermissionsResponse,
        },
    },
    types::PageRequest,
};

pub struct SubspacesQuerier<'a> {
    querier: QuerierWrapper<'a, DesmosQuery>,
}

impl<'a> SubspacesQuerier<'a> {
    pub fn new(querier: &'a dyn Querier) -> Self {
        Self {
            querier: QuerierWrapper::<'a, DesmosQuery>::new(querier),
        }
    }
}

impl<'a> SubspacesQuerier<'a> {
    pub fn query_subspaces(
        &self,
        pagination: Option<PageRequest>,
    ) -> StdResult<QuerySubspacesResponse> {
        let request = DesmosQuery::from(SubspacesQuery::Subspaces { pagination });
        let res: QuerySubspacesResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    /// Gives an iterator to scan over all the subspaces.
    ///
    /// * `page_size` - Size of the page requested to the chain.
    pub fn iterate_subspaces(&self, page_size: u64) -> PageIterator<Subspace, Binary> {
        PageIterator::new(
            Box::new(move |key, limit| {
                self.query_subspaces(Some(PageRequest {
                    key,
                    limit: limit.into(),
                    reverse: false,
                    count_total: false,
                    offset: None,
                }))
                .map(|response| Page {
                    items: response.subspaces,
                    next_page_key: response.pagination.next_key,
                })
            }),
            page_size,
        )
    }

    pub fn query_subspace(&self, subspace_id: u64) -> StdResult<QuerySubspaceResponse> {
        let request = DesmosQuery::from(SubspacesQuery::Subspace {
            subspace_id: subspace_id.into(),
        });
        let res: QuerySubspaceResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_user_groups(
        &self,
        subspace_id: u64,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryUserGroupsResponse> {
        let request = DesmosQuery::from(SubspacesQuery::UserGroups {
            subspace_id: subspace_id.into(),
            pagination,
        });
        let res: QueryUserGroupsResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    /// Gives an iterator to scan over all the user groups created in a subspace.
    ///
    /// * `subspace_id` - Subspace to query the user groups for.
    /// * `page_size` - Size of the page requested to the chain.
    pub fn iterate_user_groups(
        &self,
        subspace_id: u64,
        page_size: u64,
    ) -> PageIterator<UserGroup, Binary> {
        PageIterator::new(
            Box::new(move |key, limit| {
                self.query_user_groups(
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
                    items: response.groups,
                    next_page_key: response.pagination.next_key,
                })
            }),
            page_size,
        )
    }

    pub fn query_user_group(
        &self,
        subspace_id: u64,
        group_id: u32,
    ) -> StdResult<QueryUserGroupResponse> {
        let request = DesmosQuery::from(SubspacesQuery::UserGroup {
            subspace_id: subspace_id.into(),
            group_id,
        });
        let res: QueryUserGroupResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_user_group_members(
        &self,
        subspace_id: u64,
        group_id: u32,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryUserGroupMembersResponse> {
        let request = DesmosQuery::from(SubspacesQuery::UserGroupMembers {
            subspace_id: subspace_id.into(),
            group_id,
            pagination,
        });
        let res: QueryUserGroupMembersResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    /// Gives an iterator to scan over all the members of a user group created in a subspace.
    ///
    /// * `subspace_id` - Subspace to query the user members for.
    /// * `group_id` - Group to query the user members for.
    /// * `page_size` - Size of the page requested to the chain.
    pub fn iterate_user_group_members(
        &self,
        subspace_id: u64,
        group_id: u32,
        page_size: u64,
    ) -> PageIterator<Addr, Binary> {
        PageIterator::new(
            Box::new(move |key, limit| {
                self.query_user_group_members(
                    subspace_id,
                    group_id,
                    Some(PageRequest {
                        key,
                        limit: limit.into(),
                        reverse: false,
                        count_total: false,
                        offset: None,
                    }),
                )
                .map(|response| Page {
                    items: response.members,
                    next_page_key: response.pagination.next_key,
                })
            }),
            page_size,
        )
    }

    pub fn query_user_permissions(
        &self,
        subspace_id: u64,
        user: Addr,
    ) -> StdResult<QueryUserPermissionsResponse> {
        let request = DesmosQuery::from(SubspacesQuery::UserPermissions {
            subspace_id: subspace_id.into(),
            user,
        });
        let res: QueryUserPermissionsResponse = self.querier.query(&request.into())?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use super::*;
    use crate::mock::mock_dependencies_with_custom_querier;
    use crate::subspaces::mock::MockSubspacesQueries;

    #[test]
    fn test_query_subspaces() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(deps.querier.deref());
        let response = querier.query_subspaces(Default::default());
        let expected = QuerySubspacesResponse {
            subspaces: vec![MockSubspacesQueries::get_mock_subspace()],
            pagination: Default::default(),
        };
        assert_eq!(response.ok(), Some(expected));
    }

    #[test]
    fn test_iterate_subspaces() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(deps.querier.deref());

        let mut it = querier.iterate_subspaces(10);

        assert_eq!(
            it.next().unwrap().unwrap(),
            MockSubspacesQueries::get_mock_subspace()
        );
        assert!(it.next().is_none());
    }

    #[test]
    fn test_query_subspace() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(deps.querier.deref());
        let response = querier.query_subspace(1);
        let expected = QuerySubspaceResponse {
            subspace: MockSubspacesQueries::get_mock_subspace(),
        };
        assert_eq!(response.ok(), Some(expected));
    }

    #[test]
    fn test_query_user_groups() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(deps.querier.deref());
        let response = querier.query_user_groups(1, Default::default());
        let expected = QueryUserGroupsResponse {
            groups: vec![MockSubspacesQueries::get_mock_user_group()],
            pagination: Default::default(),
        };
        assert_eq!(response.ok(), Some(expected));
    }

    #[test]
    fn test_iterate_user_groups() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(deps.querier.deref());

        let mut it = querier.iterate_user_groups(1, 10);

        assert_eq!(
            it.next().unwrap().unwrap(),
            MockSubspacesQueries::get_mock_user_group()
        );
        assert!(it.next().is_none());
    }

    #[test]
    fn test_query_user_group() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(deps.querier.deref());
        let response = querier.query_user_group(1, 1);
        let expected = QueryUserGroupResponse {
            group: MockSubspacesQueries::get_mock_user_group(),
        };
        assert_eq!(response.ok(), Some(expected));
    }

    #[test]
    fn test_query_user_group_members() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(deps.querier.deref());
        let response = querier.query_user_group_members(1, 1, Default::default());
        let expected = QueryUserGroupMembersResponse {
            members: vec![MockSubspacesQueries::get_mock_group_member()],
            pagination: Default::default(),
        };
        assert_eq!(response.ok(), Some(expected));
    }

    #[test]
    fn test_iterate_user_group_members() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(deps.querier.deref());

        let mut it = querier.iterate_user_group_members(1, 1, 10);

        assert_eq!(
            it.next().unwrap().unwrap(),
            MockSubspacesQueries::get_mock_group_member()
        );
        assert!(it.next().is_none());
    }

    #[test]
    fn test_query_user_permissions() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(deps.querier.deref());
        let response = querier.query_user_permissions(
            1,
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        );
        let expected = QueryUserPermissionsResponse {
            permissions: Default::default(),
            details: vec![MockSubspacesQueries::get_mock_permission_detail()],
        };
        assert_eq!(response.ok(), Some(expected));
    }
}
