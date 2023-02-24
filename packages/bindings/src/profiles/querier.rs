//! Contains the querier that can be used to query data related to the x/profiles module.

#[cfg(feature = "iterators")]
use crate::iter::page_iterator::{Page, PageIterator};
#[cfg(feature = "iterators")]
use cosmwasm_std::Binary;

use crate::profiles::types::*;
use crate::types::PageRequest;
use cosmwasm_std::{Addr, Empty, QuerierWrapper, StdResult};

/// Querier allows to query data from the Desmos x/profiles module.
pub struct ProfilesQuerier<'a> {
    querier: crate::profiles::types::ProfilesQuerier<'a, Empty>,
}

impl<'a> ProfilesQuerier<'a> {
    /// Creates a new instance of [`ProfilesQuerier`].
    ///
    /// # Example
    /// ```
    /// use cosmwasm_std::{DepsMut, MessageInfo};
    /// use desmos_bindings::profiles::querier::ProfilesQuerier;
    ///
    /// pub fn contract_action(deps: DepsMut, _: MessageInfo) {
    ///     let querier = ProfilesQuerier::new(&deps.querier);
    /// }
    /// ```
    pub fn new(querier: &'a QuerierWrapper<'a, Empty>) -> Self {
        Self {
            querier: crate::profiles::types::ProfilesQuerier::new(querier),
        }
    }

    /// Gives the Desmos profile associated to an user.
    ///
    /// * `user` - Address of the user to query the profile for.
    pub fn query_profile(&self, user: Addr) -> StdResult<QueryProfileResponse> {
        self.querier.profile(user.into())
    }

    /// Queries the user's dtag transfer requests.
    ///
    /// * `receiver` - Address of the user to which query the incoming requests for.
    /// * `pagination` - Optional pagination configs.
    pub fn query_incoming_dtag_transfer_requests(
        &self,
        receiver: Addr,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryIncomingDTagTransferRequestsResponse> {
        self.querier
            .incoming_d_tag_transfer_requests(receiver.into(), pagination.map(Into::into))
    }

    /// Gives an iterator to scan over a user's dtag transfer requests.
    ///
    /// * `receiver` - Address of the user to which query the incoming requests for.
    /// * `page_size` - Size of the page requested to the chain.
    #[cfg(feature = "iterators")]
    pub fn iterate_incoming_dtag_transfer_requests(
        &self,
        receiver: Addr,
        page_size: u64,
    ) -> PageIterator<DTagTransferRequest, Binary> {
        PageIterator::new(
            Box::new(move |key, limit| {
                self.query_incoming_dtag_transfer_requests(
                    receiver.clone(),
                    Some(PageRequest {
                        key,
                        limit: limit.into(),
                        reverse: false,
                        count_total: false,
                        offset: None,
                    }),
                )
                .map(|response| Page {
                    items: response.requests,
                    next_page_key: response.pagination.and_then(|response| {
                        (!response.next_key.is_empty()).then_some(Binary::from(response.next_key))
                    }),
                })
            }),
            page_size,
        )
    }

    /// Queries a user's chain links or all the performed chain links.
    ///
    /// * `user` - Optional Desmos address of the user to which search the link for, if it's None
    /// queries all the performed chain links.
    /// * `chain_name` - Optional name of the chain to which search the link for.
    /// Used only if user is also set.
    /// * `target` - Optional external address to which query the link for.
    /// Used only if chain_name is also set.
    /// * `pagination` - Optional pagination configs.
    pub fn query_chain_links(
        &self,
        user: Option<Addr>,
        chain_name: Option<&str>,
        target: Option<&str>,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryChainLinksResponse> {
        self.querier.chain_links(
            user.unwrap_or_else(|| Addr::unchecked("")).into(),
            chain_name.unwrap_or_default().into(),
            target.unwrap_or_default().into(),
            pagination.map(Into::into),
        )
    }

    /// Gives an iterator to scan over a user's chain links or all the performed chain links.
    ///
    /// * `user` - Optional Desmos address of the user to which search the link for, if is None
    /// queries all the performed chain links.
    /// * `chain_name` - Optional name of the chain to which search the link for.
    /// Used only if user is also set.
    /// * `target` - Optional external address to which query the link for.
    /// Used only if chain_name is also set.
    /// * `page_size` - Size of the page requested to the chain.
    #[cfg(feature = "iterators")]
    pub fn iterate_chain_links<'b>(
        &'b self,
        user: Option<Addr>,
        chain_name: Option<&'b str>,
        target: Option<&'b str>,
        page_size: u64,
    ) -> PageIterator<ChainLink, Binary> {
        PageIterator::new(
            Box::new(move |key, limit| {
                self.query_chain_links(
                    user.clone(),
                    chain_name.clone(),
                    target.clone(),
                    Some(PageRequest {
                        key,
                        limit: limit.into(),
                        reverse: false,
                        count_total: false,
                        offset: None,
                    }),
                )
                .map(|response| Page {
                    items: response.links,
                    next_page_key: response.pagination.and_then(|response| {
                        (!response.next_key.is_empty()).then_some(Binary::from(response.next_key))
                    }),
                })
            }),
            page_size,
        )
    }

    /// Queries chain link owners.
    ///
    /// * `chain_name` - Optional name of the chain to which search the link owner for.
    /// * `target` - Optional external address to which search the link owner for.
    /// Used only if chain_name is also set.
    /// * `pagination` - Optional pagination configs.
    pub fn query_chain_link_owners(
        &self,
        chain_name: Option<&str>,
        target: Option<&str>,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryChainLinkOwnersResponse> {
        self.querier.chain_link_owners(
            chain_name.unwrap_or_default().into(),
            target.unwrap_or_default().into(),
            pagination.map(Into::into),
        )
    }

    /// Gives an iterator to scan over chain link owners.
    ///
    /// * `chain_name` - Optional name of the chain to which search the link owner for.
    /// * `target` - Optional external address to which search the link owner for.
    /// Used only if chain_name is also set.
    /// * `page_size` - Size of the page requested to the chain.
    #[cfg(feature = "iterators")]
    pub fn iterate_chain_link_owners<'b>(
        &'b self,
        chain_name: Option<&'b str>,
        target: Option<&'b str>,
        page_size: u64,
    ) -> PageIterator<query_chain_link_owners_response::ChainLinkOwnerDetails, Binary> {
        PageIterator::new(
            Box::new(move |key, limit| {
                self.query_chain_link_owners(
                    chain_name.clone(),
                    target.clone(),
                    Some(PageRequest {
                        key,
                        limit: limit.into(),
                        reverse: false,
                        count_total: false,
                        offset: None,
                    }),
                )
                .map(|response| Page {
                    items: response.owners,
                    next_page_key: response.pagination.and_then(|response| {
                        (!response.next_key.is_empty()).then_some(Binary::from(response.next_key))
                    }),
                })
            }),
            page_size,
        )
    }

    /// Queries default external addresses.
    ///
    /// * `owner` - Optional address of the owner to which search the default external addresses for.
    /// * `chain_name` - Optional chain name to which search the default external addresses for.
    /// Used only if owner is also set.
    /// * `pagination` - Optional pagination configs.
    pub fn query_default_external_addresses(
        &self,
        owner: Option<Addr>,
        chain_name: Option<&str>,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryDefaultExternalAddressesResponse> {
        self.querier.default_external_addresses(
            owner.unwrap_or_else(|| Addr::unchecked("")).into(),
            chain_name.unwrap_or_default().into(),
            pagination.map(Into::into),
        )
    }

    /// Gives an iterator to scan over chain link owners.
    ///
    /// * `owner` - Optional address of the owner to which search the default external addresses for.
    /// * `chain_name` - Optional chain name to which search the default external addresses for.
    /// Used only if owner is also set.
    /// * `page_size` - Size of the page requested to the chain.
    #[cfg(feature = "iterators")]
    pub fn iterate_default_external_addresses<'b>(
        &'b self,
        owner: Option<Addr>,
        chain_name: Option<&'b str>,
        page_size: u64,
    ) -> PageIterator<ChainLink, Binary> {
        PageIterator::new(
            Box::new(move |key, limit| {
                self.query_default_external_addresses(
                    owner.clone(),
                    chain_name.clone(),
                    Some(PageRequest {
                        key,
                        limit: limit.into(),
                        reverse: false,
                        count_total: false,
                        offset: None,
                    }),
                )
                .map(|response| Page {
                    items: response.links,
                    next_page_key: response.pagination.and_then(|response| {
                        (!response.next_key.is_empty()).then_some(Binary::from(response.next_key))
                    }),
                })
            }),
            page_size,
        )
    }

    /// Queries a user's app links or all the performed app links.
    ///
    /// * `user` - Optional Desmos address of the user to which search the link for, if it's None
    /// queries all the performed app links.
    /// * `application` - Optional name of the application to which search the link for.
    /// Used only if user is also set.
    /// * `username` - Optional username inside the application associated with the link.
    /// Used only if application is also set.
    /// * `page_size` - Size of the page requested to the chain.
    pub fn query_application_links(
        &self,
        user: Option<Addr>,
        application: Option<&str>,
        username: Option<&str>,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryApplicationLinksResponse> {
        self.querier.application_links(
            user.unwrap_or_else(|| Addr::unchecked("")).into(),
            application.unwrap_or_default().into(),
            username.unwrap_or_default().into(),
            pagination.map(Into::into),
        )
    }

    /// Gives an iterator to scan over a user's app links or all the performed app links.
    ///
    /// * `user` - Optional Desmos address of the user to which search the link for, if is None
    /// queries all the performed app links.
    /// * `application` - Optional name of the application to which search the link for.
    /// Used only if user is also set.
    /// * `username` - Optional username inside the application associated with the link.
    /// Used only if application is also set.
    /// * `page_size` - Size of the page requested to the chain.
    #[cfg(feature = "iterators")]
    pub fn iterate_application_links<'b>(
        &'b self,
        user: Option<Addr>,
        application: Option<&'b str>,
        username: Option<&'b str>,
        page_size: u64,
    ) -> PageIterator<ApplicationLink, Binary> {
        PageIterator::new(
            Box::new(move |key, limit| {
                self.query_application_links(
                    user.clone(),
                    application.clone(),
                    username.clone(),
                    Some(PageRequest {
                        key,
                        limit: limit.into(),
                        reverse: false,
                        count_total: false,
                        offset: None,
                    }),
                )
                .map(|response| Page {
                    items: response.links,
                    next_page_key: response.pagination.and_then(|response| {
                        (!response.next_key.is_empty()).then_some(Binary::from(response.next_key))
                    }),
                })
            }),
            page_size,
        )
    }

    /// Queries the app link through the client id that has performed the call to the oracle.
    ///
    /// * `client_id` - id of the client to which search the link for.
    pub fn query_application_link_by_client_id(
        &self,
        client_id: &str,
    ) -> StdResult<QueryApplicationLinkByClientIdResponse> {
        self.querier.application_link_by_client_id(client_id.into())
    }

    /// Queries app link owners.
    ///
    /// * `application` - Optional name of the application to which search the link owner for.
    /// * `username` - Optional username to which the link owner search for.
    /// Used only if application is also set.
    /// * `page_size` - Size of the page requested to the chain.
    pub fn query_application_link_owners(
        &self,
        application: Option<&str>,
        username: Option<&str>,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryApplicationLinkOwnersResponse> {
        self.querier.application_link_owners(
            application.unwrap_or_default().into(),
            username.unwrap_or_default().into(),
            pagination.map(Into::into),
        )
    }

    /// Gives an iterator to scan over app link owners.
    ///
    /// * `application` - Optional name of the application to which search the link owner for.
    /// Used only if user is also set.
    /// * `username` - Optional username inside the application associated with the link.
    /// Used only if application is also set.
    /// * `page_size` - Size of the page requested to the chain.
    #[cfg(feature = "iterators")]
    pub fn iterate_application_link_owners<'b>(
        &'b self,
        application: Option<&'b str>,
        username: Option<&'b str>,
        page_size: u64,
    ) -> PageIterator<query_application_link_owners_response::ApplicationLinkOwnerDetails, Binary>
    {
        PageIterator::new(
            Box::new(move |key, limit| {
                self.query_application_link_owners(
                    application.map(Into::into).clone(),
                    username.map(Into::into).clone(),
                    Some(PageRequest {
                        key,
                        limit: limit.into(),
                        reverse: false,
                        count_total: false,
                        offset: None,
                    }),
                )
                .map(|response| Page {
                    items: response.owners,
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
    use crate::mocks::mock_queriers::mock_desmos_dependencies;
    use crate::profiles::mocks::{
        MockProfilesQueries, MOCK_APPLICATION_LINK_APPLICATION, MOCK_APPLICATION_LINK_CLIENT_ID,
        MOCK_APPLICATION_LINK_USERNAME, MOCK_CHAIN_LINK_ADDRESS, MOCK_CHAIN_LINK_CHAIN_NAME,
        MOCK_USER,
    };
    #[test]
    fn test_query_profile() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = ProfilesQuerier::new(&deps.querier);

        let response = querier.query_profile(Addr::unchecked(MOCK_USER)).unwrap();
        let expected = MockProfilesQueries::get_mocked_profile_response();

        assert_eq!(expected, response)
    }

    #[test]
    fn test_query_incoming_dtag_transfer_requests() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = ProfilesQuerier::new(&deps.querier);

        let response = querier
            .query_incoming_dtag_transfer_requests(Addr::unchecked(MOCK_USER), None)
            .unwrap();
        let expected = MockProfilesQueries::get_mocked_incoming_dtag_transfer_requests_response();

        assert_eq!(expected, response)
    }

    #[test]
    fn test_iterate_incoming_dtag_transfer_requests() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = ProfilesQuerier::new(&deps.querier);

        let mut it =
            querier.iterate_incoming_dtag_transfer_requests(Addr::unchecked(MOCK_USER), 10);
        let expected = MockProfilesQueries::get_mocked_incoming_dtag_transfer_requests_response();

        assert_eq!(expected.requests[0], it.next().unwrap().unwrap(),);
        assert!(it.next().is_none());
    }

    #[test]
    fn test_query_chain_links() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = ProfilesQuerier::new(&deps.querier);

        let response = querier
            .query_chain_links(
                Some(Addr::unchecked(MOCK_USER)),
                Some(MOCK_CHAIN_LINK_CHAIN_NAME),
                Some(MOCK_CHAIN_LINK_ADDRESS),
                None,
            )
            .unwrap();
        let expected = MockProfilesQueries::get_mocked_chain_links_response();

        assert_eq!(expected, response);
    }

    #[test]
    fn test_iterate_chain_links() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = ProfilesQuerier::new(&deps.querier);

        let mut it = querier.iterate_chain_links(
            Some(Addr::unchecked(MOCK_USER)),
            Some(MOCK_CHAIN_LINK_CHAIN_NAME),
            Some(MOCK_CHAIN_LINK_ADDRESS),
            10,
        );
        let expected = MockProfilesQueries::get_mocked_chain_links_response();

        assert_eq!(expected.links[0], it.next().unwrap().unwrap());
        assert!(it.next().is_none());
    }

    #[test]
    fn test_query_chain_link_owners() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = ProfilesQuerier::new(&deps.querier);

        let response = querier
            .query_chain_link_owners(
                Some(MOCK_CHAIN_LINK_CHAIN_NAME),
                Some(MOCK_CHAIN_LINK_ADDRESS),
                None,
            )
            .unwrap();
        let expected = MockProfilesQueries::get_mocked_chain_link_owners_response();

        assert_eq!(expected, response)
    }

    #[test]
    fn test_iterate_chain_link_owners() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = ProfilesQuerier::new(&deps.querier);

        let mut it = querier.iterate_chain_link_owners(
            Some(MOCK_CHAIN_LINK_CHAIN_NAME),
            Some(MOCK_CHAIN_LINK_ADDRESS),
            10,
        );
        let expected = MockProfilesQueries::get_mocked_chain_link_owners_response();

        assert_eq!(expected.owners[0], it.next().unwrap().unwrap());
        assert!(it.next().is_none());
    }

    #[test]
    fn test_query_default_external_addresses() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = ProfilesQuerier::new(&deps.querier);

        let response = querier
            .query_default_external_addresses(
                Some(Addr::unchecked(MOCK_USER)),
                Some(MOCK_CHAIN_LINK_CHAIN_NAME),
                None,
            )
            .unwrap();
        let expected = MockProfilesQueries::get_mocked_default_external_addresses_response();

        assert_eq!(expected, response)
    }

    #[test]
    fn test_iterate_default_external_addresses() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = ProfilesQuerier::new(&deps.querier);

        let mut it = querier.iterate_default_external_addresses(
            Some(Addr::unchecked(MOCK_USER)),
            Some(MOCK_CHAIN_LINK_CHAIN_NAME),
            10,
        );
        let expected = MockProfilesQueries::get_mocked_default_external_addresses_response();

        assert_eq!(expected.links[0], it.next().unwrap().unwrap(),);
        assert!(it.next().is_none());
    }

    #[test]
    fn test_query_query_application_links() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = ProfilesQuerier::new(&deps.querier);

        let response = querier
            .query_application_links(
                Some(Addr::unchecked(MOCK_USER)),
                Some(MOCK_APPLICATION_LINK_APPLICATION),
                Some(MOCK_APPLICATION_LINK_USERNAME),
                Default::default(),
            )
            .unwrap();

        let expected = MockProfilesQueries::get_mocked_application_links_response();
        assert_eq!(expected, response)
    }

    #[test]
    fn test_iterate_application_links() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = ProfilesQuerier::new(&deps.querier);

        let mut it = querier.iterate_application_links(
            Some(Addr::unchecked(MOCK_USER)),
            Some(MOCK_APPLICATION_LINK_APPLICATION),
            Some(MOCK_APPLICATION_LINK_USERNAME),
            10,
        );
        let expected = MockProfilesQueries::get_mocked_application_links_response();

        assert_eq!(expected.links[0], it.next().unwrap().unwrap(),);
        assert!(it.next().is_none());
    }

    #[test]
    fn test_query_application_link_by_client_id() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = ProfilesQuerier::new(&deps.querier);

        let response = querier
            .query_application_link_by_client_id(MOCK_APPLICATION_LINK_CLIENT_ID)
            .unwrap();
        let expected = MockProfilesQueries::get_mocked_application_link_by_client_id_response();

        assert_eq!(expected, response)
    }

    #[test]
    fn test_query_app_link_owners() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = ProfilesQuerier::new(&deps.querier);

        let response = querier
            .query_application_link_owners(
                Some(MOCK_APPLICATION_LINK_APPLICATION),
                Some(MOCK_APPLICATION_LINK_USERNAME),
                None,
            )
            .unwrap();
        let expected = MockProfilesQueries::get_mocked_application_link_owners_response();

        assert_eq!(expected, response)
    }

    #[test]
    fn test_iterate_app_link_owners() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();

        let querier = ProfilesQuerier::new(&deps.querier);
        let mut it = querier.iterate_application_link_owners(
            Some(MOCK_APPLICATION_LINK_APPLICATION),
            Some(MOCK_APPLICATION_LINK_USERNAME),
            10,
        );
        let expected = MockProfilesQueries::get_mocked_application_link_owners_response();

        assert_eq!(expected.owners[0], it.next().unwrap().unwrap(),);
        assert!(it.next().is_none())
    }
}
