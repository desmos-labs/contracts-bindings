//! Contains a querier to query data from the Desmos x/subspaces module.

use crate::subspaces::models::Section;
use crate::subspaces::query_types::{QuerySectionResponse, QuerySectionsResponse};
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
use cosmwasm_std::{Addr, Querier, QuerierWrapper, StdResult, Uint64};
#[cfg(feature = "iterators")]
use {
    crate::iter::page_iterator::{Page, PageIterator},
    crate::subspaces::models::{Subspace, UserGroup},
    cosmwasm_std::Binary,
};

/// Querier able to query data from the Desmos x/subspaces module.
pub struct SubspacesQuerier<'a> {
    querier: QuerierWrapper<'a, DesmosQuery>,
}

impl<'a> SubspacesQuerier<'a> {
    /// Create a new [SubspacesQuerier]
    ///
    /// # Example
    /// ```
    /// use std::ops::Deref;
    /// use cosmwasm_std::{DepsMut, MessageInfo};
    /// use desmos_bindings::subspaces::querier::SubspacesQuerier;
    ///
    /// pub fn contract_action(deps: DepsMut, _: MessageInfo) {
    ///     let querier = SubspacesQuerier::new(deps.querier.deref());
    ///     let subspaces_response = querier.query_subspaces(None);
    /// }
    /// ```
    pub fn new(querier: &'a dyn Querier) -> Self {
        Self {
            querier: QuerierWrapper::<'a, DesmosQuery>::new(querier),
        }
    }
}

impl<'a> SubspacesQuerier<'a> {
    /// Queries all the subspaces created.
    ///
    /// * `pagination` - Optional pagination configs.
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
    #[cfg(feature = "iterators")]
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
                    next_page_key: response
                        .pagination
                        .map_or(None, |response| response.next_key),
                })
            }),
            page_size,
        )
    }

    /// Queries the details of a subspace.
    ///
    /// * `subspace_id` - Subspace of interest.
    pub fn query_subspace(&self, subspace_id: u64) -> StdResult<QuerySubspaceResponse> {
        let request = DesmosQuery::from(SubspacesQuery::Subspace {
            subspace_id: subspace_id.into(),
        });
        let res: QuerySubspaceResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    /// Queries all the sections created inside a subspace.
    ///
    /// * `subspace_id` - Subspace to which the sections belong.
    /// * `pagination` - Optional pagination configs.
    pub fn query_sections(
        &self,
        subspace_id: u64,
        pagination: Option<PageRequest>,
    ) -> StdResult<QuerySectionsResponse> {
        let request = DesmosQuery::from(SubspacesQuery::Sections {
            subspace_id: Uint64::new(subspace_id),
            pagination,
        });
        let res: QuerySectionsResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    /// Gives an iterator to scan over all the sections inside a subspace.
    ///
    /// * `subspace_id` - Subspace to which the sections belong.
    /// * `page_size` - Size of the page requested to the chain.
    #[cfg(feature = "iterators")]
    pub fn iterate_sections(
        &self,
        subspace_id: u64,
        page_size: u64,
    ) -> PageIterator<Section, Binary> {
        PageIterator::new(
            Box::new(move |key, limit| {
                self.query_sections(
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
                    items: response.sections,
                    next_page_key: response
                        .pagination
                        .map_or(None, |response| response.next_key),
                })
            }),
            page_size,
        )
    }

    /// Queries the details of a section.
    ///
    /// * `subspace_id` - Subspace to which the section belong.
    /// * `section_id` - Section of interest.
    pub fn query_section(
        &self,
        subspace_id: u64,
        section_id: u32,
    ) -> StdResult<QuerySectionResponse> {
        let request = DesmosQuery::from(SubspacesQuery::Section {
            subspace_id: subspace_id.into(),
            section_id,
        });
        let res: QuerySectionResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    /// Queries the user groups created in a subspace.
    ///
    /// * `subspace_id` - Subspace to which the groups belong.
    /// * `section_id` - Section id to query the groups for.
    /// * `pagination` - Optional pagination configs.
    pub fn query_user_groups(
        &self,
        subspace_id: u64,
        section_id: Option<u32>,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryUserGroupsResponse> {
        let request = DesmosQuery::from(SubspacesQuery::UserGroups {
            subspace_id: subspace_id.into(),
            section_id,
            pagination,
        });
        let res: QueryUserGroupsResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    /// Gives an iterator to scan over all the user groups created in a subspace.
    ///
    /// * `subspace_id` - Subspace to which the groups belong.
    /// * `section_id` - Section id to query the groups for.
    /// * `page_size` - Size of the page requested to the chain.
    #[cfg(feature = "iterators")]
    pub fn iterate_user_groups(
        &self,
        subspace_id: u64,
        section_id: Option<u32>,
        page_size: u64,
    ) -> PageIterator<UserGroup, Binary> {
        PageIterator::new(
            Box::new(move |key, limit| {
                self.query_user_groups(
                    subspace_id,
                    section_id,
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
                    next_page_key: response
                        .pagination
                        .map_or(None, |response| response.next_key),
                })
            }),
            page_size,
        )
    }

    /// Queries the details of a user group.
    ///
    /// * `subspace_id` - Subspace to which the group belong.
    /// * `group_id` - Group of interest.
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

    /// Queries the members of a group.
    ///
    /// * `subspace_id` - Subspace to which the group belong.
    /// * `group_id` - Group to which the users belong.
    /// * `pagination` - Optional pagination configs.
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
    /// * `subspace_id` - Subspace to which the group belong.
    /// * `group_id` - Group to which the users belong.
    /// * `page_size` - Size of the page requested to the chain.
    #[cfg(feature = "iterators")]
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
                    next_page_key: response
                        .pagination
                        .map_or(None, |response| response.next_key),
                })
            }),
            page_size,
        )
    }

    /// Queries the permissions that an user have in a subspace.
    ///
    /// * `subspace_id` - Subspace to which the user belong.
    /// * `user` - User address.
    pub fn query_user_permissions(
        &self,
        subspace_id: u64,
        section_id: Option<u32>,
        user: Addr,
    ) -> StdResult<QueryUserPermissionsResponse> {
        let request = DesmosQuery::from(SubspacesQuery::UserPermissions {
            subspace_id: subspace_id.into(),
            section_id,
            user,
        });
        let res: QueryUserPermissionsResponse = self.querier.query(&request.into())?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::mock_dependencies_with_custom_querier;
    use crate::subspaces::mock::MockSubspacesQueries;
    use std::ops::Deref;

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
    fn test_query_sections() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(deps.querier.deref());
        let response = querier.query_sections(1, Default::default());
        let expected = QuerySectionsResponse {
            sections: vec![MockSubspacesQueries::get_mock_section()],
            pagination: Default::default(),
        };
        assert_eq!(response.ok(), Some(expected));
    }

    #[test]
    fn test_iterate_sections() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(deps.querier.deref());

        let mut it = querier.iterate_sections(1, 10);

        assert_eq!(
            it.next().unwrap().unwrap(),
            MockSubspacesQueries::get_mock_section()
        );
        assert!(it.next().is_none());
    }

    #[test]
    fn test_query_section() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(deps.querier.deref());
        let response = querier.query_section(1, 1);
        let expected = QuerySectionResponse {
            section: MockSubspacesQueries::get_mock_section(),
        };
        assert_eq!(response.ok(), Some(expected));
    }

    #[test]
    fn test_query_user_groups() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(deps.querier.deref());
        let response = querier.query_user_groups(1, None, Default::default());
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

        let mut it = querier.iterate_user_groups(1, None, 10);

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
            None,
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        );
        let expected = QueryUserPermissionsResponse {
            permissions: Default::default(),
            details: vec![MockSubspacesQueries::get_mock_permission_detail()],
        };
        assert_eq!(response.ok(), Some(expected));
    }
}
