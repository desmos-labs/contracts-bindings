//! Contains structs and enums related to the Desmos profile.

use crate::legacy::types::PubKey;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint64};

/// Represents a Desmos profile, contains the information of a single user.
#[cw_serde]
pub struct Profile {
    /// The profile proto type.
    #[serde(rename = "@type")]
    pub proto_type: String,
    /// The base Cosmos account associated with this profile.
    pub account: Account,
    /// Unique tag of this profile.
    pub dtag: String,
    /// Custom human readable name of the profile.
    pub nickname: String,
    /// Biography of the profile.
    pub bio: String,
    /// Data about the pictures associated with he profile.
    pub pictures: Pictures,
    /// The time in which the profile has been created,
    pub creation_date: String,
}

/// Represents the base Cosmos account.
#[cw_serde]
pub struct Account {
    /// Account type.
    #[serde(rename = "@type")]
    pub proto_type: String,
    /// Account address.
    pub address: Addr,
    /// Public key of this account.
    pub pub_key: PubKey,
    /// Number that identifies this account.
    pub account_number: Uint64,
    /// Number that should be used as sequence number when performing a transaction.
    pub sequence: Uint64,
}

/// Data of a user profile's related pictures.
#[cw_serde]
pub struct Pictures {
    /// URL to the profile picture.
    pub profile: String,
    /// URL to the cover picture.
    pub cover: String,
}
