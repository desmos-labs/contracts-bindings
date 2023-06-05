//! Contains the types definitions of x/profiles.

pub use desmos_std::proto::desmos::profiles::v3::*;

use crate::cosmos_types::Any;

/// Represents a generic address data.
#[derive(Clone)]
pub enum AddressData {
    /// Represents the bech32 encoded address.
    Bech32Address(Bech32Address),

    /// Represents the hex encoded address.
    HexAddress(HexAddress),

    /// Represents the base58 encoded address.
    Base58Address(Base58Address),
}

impl Into<Any> for AddressData {
    fn into(self) -> Any {
        match self {
            AddressData::Bech32Address(address) => address.into(),
            AddressData::HexAddress(address) => address.into(),
            AddressData::Base58Address(address) => address.into(),
        }
    }
}
