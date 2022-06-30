//! Contains structures returned from the [ProfilesQuerier<'a>](crate::profiles::querier::ProfilesQuerier).

use crate::{
    profiles::{
        models_app_links::{ApplicationLink, ApplicationLinkOwnerDetails},
        models_chain_links::{ChainLink, ChainLinkOwnerDetails},
        models_dtag_requests::DtagTransferRequest, models_profile::Profile,
    },
    types::PageResponse,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Response to [`ProfilesQuery::Profile`](crate::profiles::query::ProfilesQuery::Profile).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryProfileResponse {
    /// The queried profile.
    pub profile: Profile,
}

/// Response to [`ProfilesQuery::IncomingDtagTransferRequests`](crate::profiles::query::ProfilesQuery::IncomingDtagTransferRequests).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryIncomingDtagTransferRequestResponse {
    /// Queried dtag transfer requests.
    pub requests: Vec<DtagTransferRequest>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`ProfilesQuery::ChainLinks`](crate::profiles::query::ProfilesQuery::ChainLinks).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryChainLinksResponse {
    /// Queried chain links.
    pub links: Vec<ChainLink>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`ProfilesQuery::ChainLinkOwners`](crate::profiles::query::ProfilesQuery::ChainLinkOwners).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryChainLinkOwnersResponse {
    /// Queried owners with details.
    pub owners: Vec<ChainLinkOwnerDetails>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`ProfilesQuery::ApplicationLinks`](crate::profiles::query::ProfilesQuery::ApplicationLinks).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryApplicationLinksResponse {
    /// Queried application links.
    pub links: Vec<ApplicationLink>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`ProfilesQuery::ApplicationLinkByChainID`](crate::profiles::query::ProfilesQuery::ApplicationLinkByChainID).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryApplicationLinkByClientIDResponse {
    /// Queried application link.
    pub link: ApplicationLink,
}

/// Response to [`ProfilesQuery::ApplicationLinkOwners`](crate::profiles::query::ProfilesQuery::ApplicationLinkOwners).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryApplicationLinkOwnersResponse {
    /// Queried owners with details
    pub owners: Vec<ApplicationLinkOwnerDetails>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}