//! Contains the query actions that can be sent to the chain in order to query data related
//! to the x/profiles module.

use crate::types::PageRequest;
use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Query messages that can be sent to the x/profiles module.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProfilesQuery {
    /// Message to query a user profile.
    Profile {
        /// Address or DTag of the user to query the profile for.
        user: String,
    },
    /// Message to query the dtag transfer requests.
    IncomingDtagTransferRequests {
        /// The address of the user to which query the incoming requests for.
        receiver: Addr,
        /// Optional pagination configs.
        pagination: Option<PageRequest>,
    },
    /// Message to query the chain links.
    ChainLinks {
        /// Optional address of the user to which search the link for.
        user: Option<Addr>,
        /// name of the chain to which search the link for.
        /// Used only if `user` not `None`.
        chain_name: Option<String>,
        /// the external address to which query the link for.
        /// Used only if `chain_name` is not `None`.
        target: Option<String>,
        /// Optional pagination configs.
        pagination: Option<PageRequest>,
    },
    /// Message to query the owners of chain links.
    ChainLinkOwners {
        /// (Optional) Chain name to search link owners of.
        chain_name: Option<String>,
        /// (Optional) External address to search for.
        /// Used only if `chain_name` is not `None`.
        target: Option<String>,
        /// Optional pagination configs.
        pagination: Option<PageRequest>,
    },
    /// Message to query the default external addresses.
    DefaultExternalAddresses {
        /// (Optional) Owner for which to query the default addresses.
        owner: Option<Addr>,
        /// (Optional) Chain name to query the default addresses for.
        chain_name: Option<String>,
        /// Optional pagination configs.
        pagination: Option<PageRequest>,
    },
    /// Message to query the application links.
    ApplicationLinks {
        /// Address associated for which the link should be searched.
        /// If `None` queries all the performed application links.
        user: Option<Addr>,
        /// Application name associated with the link.
        /// Used only if `user` not `None`.
        application: Option<String>,
        /// Username inside the application associated with the link.
        /// Used only if `application` not `None`.
        username: Option<String>,
        /// Optional pagination configs.
        pagination: Option<PageRequest>,
    },
    /// Message to query the app link through the client id that has performed the call to the oracle.
    ApplicationLinkByChainID {
        /// Id of the client to which search the link for.
        client_id: String,
    },
    /// Message to queries the owners of application links.
    ApplicationLinkOwners {
        /// (Optional) Application name to search link owners of.
        application: Option<String>,
        /// (Optional) Username to search for.
        /// Used only if `application` not `None`.
        username: Option<String>,
        /// Optional pagination configs.
        pagination: Option<PageRequest>,
    },
}
