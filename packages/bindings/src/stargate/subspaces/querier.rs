//! Contains a querier to query data from the Desmos x/subspaces module.

use crate::stargate::subspaces::proto::*;
use crate::stargate::types::PageRequest;
use cosmwasm_std::{Addr, Empty, QuerierWrapper, StdResult};
#[cfg(feature = "iterators")]
use {
    crate::iter::page_iterator::{Page, PageIterator},
    crate::stargate::subspaces::proto::{Section, Subspace, UserGroup},
    cosmwasm_std::Binary,
};

/// Querier able to query data from the Desmos x/subspaces module.
pub struct SubspacesQuerier<'a> {
    querier: crate::stargate::subspaces::proto::SubspacesQuerier<'a, Empty>,
}

impl<'a> SubspacesQuerier<'a> {
    /// Create a new [SubspacesQuerier]
    ///
    /// # Example
    /// ```
    /// use cosmwasm_std::{DepsMut, MessageInfo};
    /// use desmos_bindings::subspaces::querier::SubspacesQuerier;
    ///
    /// pub fn contract_action(deps: DepsMut, _: MessageInfo) {
    ///     let querier = SubspacesQuerier::new(&deps.querier);
    ///     let subspaces_response = querier.query_subspaces(None);
    /// }
    /// ```
    pub fn new(querier: &'a QuerierWrapper<'a, Empty>) -> Self {
        Self {
            querier: crate::stargate::subspaces::proto::SubspacesQuerier::new(querier),
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
        self.querier.subspaces(pagination.map(Into::into))
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
                    next_page_key: response.pagination.and_then(|response| {
                        (!response.next_key.is_empty()).then_some(Binary::from(response.next_key))
                    }),
                })
            }),
            page_size,
        )
    }

    /// Queries the details of a subspace.
    ///
    /// * `subspace_id` - Subspace of interest.
    pub fn query_subspace(&self, subspace_id: u64) -> StdResult<QuerySubspaceResponse> {
        self.querier.subspace(subspace_id)
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
        self.querier
            .sections(subspace_id, pagination.map(Into::into))
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
                    next_page_key: response.pagination.and_then(|response| {
                        (!response.next_key.is_empty()).then_some(Binary::from(response.next_key))
                    }),
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
        self.querier.section(subspace_id, section_id)
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
        self.querier.user_groups(
            subspace_id,
            section_id.unwrap_or_default(),
            pagination.map(Into::into),
        )
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
                    next_page_key: response.pagination.and_then(|response| {
                        (!response.next_key.is_empty()).then_some(Binary::from(response.next_key))
                    }),
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
        self.querier.user_group(subspace_id, group_id)
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
        self.querier
            .user_group_members(subspace_id, group_id, pagination.map(Into::into))
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
    ) -> PageIterator<String, Binary> {
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
                    next_page_key: response.pagination.and_then(|response| {
                        (!response.next_key.is_empty()).then_some(Binary::from(response.next_key))
                    }),
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
        self.querier
            .user_permissions(subspace_id, section_id.unwrap_or_default(), user.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stargate::mocks::mock_queriers::mock_desmos_dependencies;
    use crate::stargate::subspaces::mocks::MockSubspacesQueries;

    #[test]
    fn test_query_subspaces() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(&deps.querier);
        let response = querier.query_subspaces(None).unwrap();
        let expected = MockSubspacesQueries::get_mocked_subspaces_response();
        assert_eq!(expected, response);
    }

    #[test]
    fn test_iterate_subspaces() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(&deps.querier);
        let mut it = querier.iterate_subspaces(10);
        let expected = MockSubspacesQueries::get_mocked_subspaces_response();
        assert_eq!(expected.subspaces[0], it.next().unwrap().unwrap(),);
        assert!(it.next().is_none());
    }

    #[test]
    fn test_query_subspace() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(&deps.querier);
        let response = querier.query_subspace(1).unwrap();
        let expected = MockSubspacesQueries::get_mocked_subspace_response();
        assert_eq!(expected, response);
    }

    #[test]
    fn test_query_sections() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(&deps.querier);
        let response = querier.query_sections(1, None).unwrap();
        let expected = MockSubspacesQueries::get_mocked_sections_response();
        assert_eq!(expected, response);
    }

    #[test]
    fn test_iterate_sections() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(&deps.querier);
        let mut it = querier.iterate_sections(1, 10);
        let expected = MockSubspacesQueries::get_mocked_sections_response();
        assert_eq!(expected.sections[0], it.next().unwrap().unwrap(),);
        assert!(it.next().is_none());
    }

    #[test]
    fn test_query_section() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(&deps.querier);
        let response = querier.query_section(1, 1).unwrap();
        let expected = MockSubspacesQueries::get_mocked_section_response();
        assert_eq!(expected, response);
    }

    #[test]
    fn test_query_user_groups() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(&deps.querier);
        let response = querier.query_user_groups(1, None, None).unwrap();
        let expected = MockSubspacesQueries::get_mocked_user_groups_response();
        assert_eq!(expected, response);
    }

    #[test]
    fn test_iterate_user_groups() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(&deps.querier);
        let mut it = querier.iterate_user_groups(1, None, 10);
        let expected = MockSubspacesQueries::get_mocked_user_groups_response();
        assert_eq!(expected.groups[0], it.next().unwrap().unwrap(),);
        assert!(it.next().is_none());
    }

    #[test]
    fn test_query_user_group() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(&deps.querier);
        let response = querier.query_user_group(1, 1).unwrap();
        let expected = MockSubspacesQueries::get_mocked_user_group_response();
        assert_eq!(expected, response);
    }

    #[test]
    fn test_query_user_group_members() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(&deps.querier);
        let response = querier.query_user_group_members(1, 1, None).unwrap();
        let expected = MockSubspacesQueries::get_mocked_user_group_members_response();
        assert_eq!(expected, response);
    }

    #[test]
    fn test_iterate_user_group_members() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(&deps.querier);
        let mut it = querier.iterate_user_group_members(1, 1, 10);
        let expected = MockSubspacesQueries::get_mocked_user_group_members_response();
        assert_eq!(expected.members[0], it.next().unwrap().unwrap(),);
        assert!(it.next().is_none());
    }

    #[test]
    fn test_query_user_permissions() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(&deps.querier);
        let response = querier
            .query_user_permissions(
                1,
                None,
                Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            )
            .unwrap();
        let expected = MockSubspacesQueries::get_mocked_user_permissions_response();
        assert_eq!(expected, response);
    }
}
