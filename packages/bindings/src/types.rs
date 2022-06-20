//! Contains some basic types of a cosmos sdk based chain.

use cosmwasm_std::{Binary, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Represents a chain block height.
/// Normally the `revision_height` is incremented at each height while keeping `revision_number` the same.
/// However some consensus algorithms may choose to reset the height in
/// certain conditions e.g. hard forks, state-machine breaking changes in these cases,
/// the `revision_number` is incremented so that height continues to be monitonically increasing
/// even as the `revision_height` gets reset.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Height {
    /// The revision that the client is currently on.
    pub revision_number: Uint64,
    /// The height within the given revision.
    pub revision_height: Uint64,
}

/// Represents a generic public key.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PubKey {
    /// Public key type.
    #[serde(rename = "@type")]
    pub proto_type: String,
    /// Base64 encoded public key.
    pub key: Binary,
}

/// Represents the configurations that tell the application which page of data to fetch.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PageRequest {
    /// Value returned in PageResponse.next_key to begin querying the next page most efficiently.
    /// Only one of offset or key should be set.
    pub key: Option<Binary>,
    /// A numeric offset that can be used when key is unavailable. It is less efficient than using key.
    /// Only one of offset or key should be set.
    pub offset: Option<Uint64>,
    /// The total number of results to be returned in the result page.
    pub limit: Uint64,
    /// Set to true to indicate that the result set should include
    /// a count of the total number of items available for pagination.
    /// `count_total` is only respected when offset is used.
    /// It is ignored when key is set.
    pub count_total: bool,
    /// Set to true if results are to be returned in the descending order.
    pub reverse: bool,
}

/// Response returned from a query method that had used [PageRequest].
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PageResponse {
    /// Key to be passed to [PageRequest::key] to query the next page most efficiently.
    pub next_key: Option<Binary>,
    /// Total number of results available if [PageRequest::count_total] was set, its value is `None` otherwise.
    pub total: Option<Uint64>,
}
