//! Contains structures returned from the [ProfilesQuerier<'a>](crate::profiles::querier::ProfilesQuerier).

use crate::{
    profiles::{
        models_app_links::{ApplicationLink, ApplicationLinkOwnerDetails},
        models_chain_links::{ChainLink, ChainLinkOwnerDetails},
        models_dtag_requests::DtagTransferRequest,
        models_profile::Profile,
    },
    types::PageResponse,
};
use cosmwasm_schema::cw_serde;

/// Response to [`ProfilesQuery::Profile`](crate::profiles::query::ProfilesQuery::Profile).
#[cw_serde]
pub struct QueryProfileResponse {
    /// The queried profile.
    pub profile: Profile,
}

/// Response to [`ProfilesQuery::IncomingDtagTransferRequests`](crate::profiles::query::ProfilesQuery::IncomingDtagTransferRequests).
#[cw_serde]
pub struct QueryIncomingDtagTransferRequestsResponse {
    /// Queried dtag transfer requests.
    pub requests: Vec<DtagTransferRequest>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`ProfilesQuery::ChainLinks`](crate::profiles::query::ProfilesQuery::ChainLinks).
#[cw_serde]
pub struct QueryChainLinksResponse {
    /// Queried chain links.
    pub links: Vec<ChainLink>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`ProfilesQuery::ChainLinkOwners`](crate::profiles::query::ProfilesQuery::ChainLinkOwners).
#[cw_serde]
pub struct QueryChainLinkOwnersResponse {
    /// Queried owners with details.
    pub owners: Vec<ChainLinkOwnerDetails>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`ProfilesQuery::DefaultExternalAddresses`](crate::profiles::query::ProfilesQuery::DefaultExternalAddresses).
#[cw_serde]
pub struct QueryDefaultExternalAddressesResponse {
    /// List of default addresses, each one represented by the associated chain link.
    pub links: Vec<ChainLink>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`ProfilesQuery::ApplicationLinks`](crate::profiles::query::ProfilesQuery::ApplicationLinks).
#[cw_serde]
pub struct QueryApplicationLinksResponse {
    /// Queried application links.
    pub links: Vec<ApplicationLink>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`ProfilesQuery::ApplicationLinkByChainID`](crate::profiles::query::ProfilesQuery::ApplicationLinkByChainID).
#[cw_serde]
pub struct QueryApplicationLinkByClientIDResponse {
    /// Queried application link.
    pub link: ApplicationLink,
}

/// Response to [`ProfilesQuery::ApplicationLinkOwners`](crate::profiles::query::ProfilesQuery::ApplicationLinkOwners).
#[cw_serde]
pub struct QueryApplicationLinkOwnersResponse {
    /// Queried owners with details
    pub owners: Vec<ApplicationLinkOwnerDetails>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}
