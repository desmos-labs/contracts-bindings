//! Contains some basic types of a cosmos sdk based chain.

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Uint64};
use desmos_std::proto::cosmos::base::query::v1beta1::PageRequest as ProtoPageRequest;
pub use desmos_std::public_keys::{Ed25519PublicKey, Secp256k1PublicKey, Secp256r1PublicKey};

/// Represents the configurations that tell the application which page of data to fetch.
#[cw_serde]
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

impl Into<ProtoPageRequest> for PageRequest {
    fn into(self) -> ProtoPageRequest {
        ProtoPageRequest {
            key: self.key.unwrap_or_default().to_vec(),
            offset: self.offset.unwrap_or_default().into(),
            limit: self.limit.into(),
            count_total: self.count_total,
            reverse: self.reverse,
        }
    }
}

pub use desmos_std::proto::ibc::core::client::v1::Height;
pub use desmos_std::shim::Any;
