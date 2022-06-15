//! Contains the querier that can be used to query data related to the x/profiles module.

#[cfg(feature = "iterators")]
use crate::{
    iter::page_iterator::{Page, PageIterator},
    profiles::models_app_links::ApplicationLink,
    profiles::models_chain_links::ChainLink,
    profiles::models_dtag_requests::DtagTransferRequest,
};
#[cfg(feature = "iterators")]
use cosmwasm_std::Binary;

use crate::{
    profiles::{
        models_query::{
            QueryApplicationLinkByClientIDResponse, QueryApplicationLinksResponse,
            QueryChainLinksResponse, QueryIncomingDtagTransferRequestResponse,
            QueryProfileResponse,
        },
        query::ProfilesQuery,
    },
    query::DesmosQuery,
    types::PageRequest,
};
use cosmwasm_std::{Addr, Querier, QuerierWrapper, StdResult};

/// Querier able to query data from the Desmos x/profiles module.
pub struct ProfilesQuerier<'a> {
    querier: QuerierWrapper<'a, DesmosQuery>,
}

impl<'a> ProfilesQuerier<'a> {
    /// Creates a new instance of [`ProfilesQuerier`].
    ///
    /// # Example
    /// ```
    /// use std::ops::Deref;
    /// use cosmwasm_std::{DepsMut, MessageInfo};
    /// use desmos_bindings::profiles::querier::ProfilesQuerier;
    ///
    /// pub fn contract_action(deps: DepsMut, _: MessageInfo) {
    ///     let querier = ProfilesQuerier::new(deps.querier.deref());
    /// }
    /// ```
    pub fn new(querier: &'a dyn Querier) -> Self {
        Self {
            querier: QuerierWrapper::<'a, DesmosQuery>::new(querier),
        }
    }

    /// Gives the Desmos profile associated to an user.
    ///
    /// * `user` - Address of the user to query the profile for.
    pub fn query_profile(&self, user: Addr) -> StdResult<QueryProfileResponse> {
        let request = DesmosQuery::Profiles(ProfilesQuery::Profile {
            user: user.to_string(),
        });

        let res: QueryProfileResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    /// Queries the user's dtag transfer requests.
    ///
    /// * `receiver` - Address of the user to which query the incoming requests for.
    /// * `pagination` - Optional pagination configs.
    pub fn query_incoming_dtag_transfer_requests(
        &self,
        receiver: Addr,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryIncomingDtagTransferRequestResponse> {
        let request = DesmosQuery::Profiles(ProfilesQuery::IncomingDtagTransferRequests {
            receiver,
            pagination,
        });

        let res: QueryIncomingDtagTransferRequestResponse = self.querier.query(&request.into())?;
        Ok(res)
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
    ) -> PageIterator<DtagTransferRequest, Binary> {
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
                    next_page_key: response.pagination.and_then(|response| response.next_key),
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
        chain_name: Option<String>,
        target: Option<String>,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryChainLinksResponse> {
        let request = DesmosQuery::Profiles(ProfilesQuery::ChainLinks {
            user,
            chain_name,
            target,
            pagination,
        });

        let res: QueryChainLinksResponse = self.querier.query(&request.into())?;
        Ok(res)
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
    pub fn iterate_chain_links(
        &self,
        user: Option<Addr>,
        chain_name: Option<String>,
        target: Option<String>,
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
                    next_page_key: response.pagination.and_then(|response| response.next_key),
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
        application: Option<String>,
        username: Option<String>,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryApplicationLinksResponse> {
        let request = DesmosQuery::Profiles(ProfilesQuery::ApplicationLinks {
            user,
            application,
            username,
            pagination,
        });

        let res: QueryApplicationLinksResponse = self.querier.query(&request.into())?;
        Ok(res)
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
    pub fn iterate_application_links(
        &self,
        user: Option<Addr>,
        application: Option<String>,
        username: Option<String>,
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
                    next_page_key: response.pagination.and_then(|response| response.next_key),
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
    ) -> StdResult<QueryApplicationLinkByClientIDResponse> {
        let request = DesmosQuery::Profiles(ProfilesQuery::ApplicationLinkByChainID {
            client_id: client_id.to_owned(),
        });

        let res: QueryApplicationLinkByClientIDResponse = self.querier.query(&request.into())?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        mock::mock_dependencies_with_custom_querier,
        profiles::{
            mock::MockProfilesQueries,
            models_query::{
                QueryApplicationLinkByClientIDResponse, QueryApplicationLinksResponse,
                QueryChainLinksResponse, QueryIncomingDtagTransferRequestResponse,
                QueryProfileResponse,
            },
            querier::ProfilesQuerier,
        },
    };
    use cosmwasm_std::Addr;
    use std::ops::Deref;

    #[test]
    fn test_query_profile() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let profiles_querier = ProfilesQuerier::new(deps.querier.deref());

        let response = profiles_querier.query_profile(Addr::unchecked("")).unwrap();
        let expected = QueryProfileResponse {
            profile: MockProfilesQueries::get_mock_profile(),
        };

        assert_eq!(response, expected)
    }

    #[test]
    fn test_query_incoming_dtag_transfer_requests() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let profiles_querier = ProfilesQuerier::new(deps.querier.deref());

        let response = profiles_querier
            .query_incoming_dtag_transfer_requests(Addr::unchecked(""), None)
            .unwrap();
        let expected = QueryIncomingDtagTransferRequestResponse {
            requests: vec![MockProfilesQueries::get_mock_dtag_transfer_request()],
            pagination: Default::default(),
        };

        assert_eq!(response, expected)
    }

    #[test]
    fn test_iterate_incoming_dtag_transfer_requests() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let profiles_querier = ProfilesQuerier::new(deps.querier.deref());

        let mut it =
            profiles_querier.iterate_incoming_dtag_transfer_requests(Addr::unchecked(""), 10);

        assert_eq!(
            it.next().unwrap().unwrap(),
            MockProfilesQueries::get_mock_dtag_transfer_request()
        );
        assert!(it.next().is_none());
    }

    #[test]
    fn test_query_chain_links() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let profiles_querier = ProfilesQuerier::new(deps.querier.deref());

        let response = profiles_querier
            .query_chain_links(
                Some(Addr::unchecked("")),
                Some("cosmos".to_string()),
                Some("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2".to_string()),
                None,
            )
            .unwrap();
        let expected = QueryChainLinksResponse {
            links: vec![MockProfilesQueries::get_mock_chain_link()],
            pagination: Default::default(),
        };

        assert_eq!(response, expected)
    }

    #[test]
    fn test_iterate_chain_links() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let profiles_querier = ProfilesQuerier::new(deps.querier.deref());

        let mut it = profiles_querier.iterate_chain_links(
            Some(Addr::unchecked("")),
            Some("cosmos".to_string()),
            Some("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2".to_string()),
            10,
        );

        assert_eq!(
            it.next().unwrap().unwrap(),
            MockProfilesQueries::get_mock_chain_link()
        );
        assert!(it.next().is_none());
    }

    #[test]
    fn test_query_app_links() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let profiles_querier = ProfilesQuerier::new(deps.querier.deref());

        let response = profiles_querier
            .query_application_links(
                Some(Addr::unchecked("")),
                Some("twitter".to_string()),
                Some("goldrake".to_string()),
                None,
            )
            .unwrap();
        let expected = QueryApplicationLinksResponse {
            links: vec![MockProfilesQueries::get_mock_application_link()],
            pagination: Default::default(),
        };

        assert_eq!(response, expected)
    }

    #[test]
    fn test_iterate_app_links() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let profiles_querier = ProfilesQuerier::new(deps.querier.deref());

        let mut it = profiles_querier.iterate_application_links(
            Some(Addr::unchecked("")),
            Some("twitter".to_string()),
            Some("goldrake".to_string()),
            10,
        );

        assert_eq!(
            it.next().unwrap().unwrap(),
            MockProfilesQueries::get_mock_application_link()
        );
        assert!(it.next().is_none());
    }

    #[test]
    fn test_query_application_link_by_chain_id() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let profiles_querier = ProfilesQuerier::new(deps.querier.deref());

        let response = profiles_querier
            .query_application_link_by_client_id("")
            .unwrap();
        let expected = QueryApplicationLinkByClientIDResponse {
            link: MockProfilesQueries::get_mock_application_link(),
        };

        assert_eq!(response, expected)
    }
}
