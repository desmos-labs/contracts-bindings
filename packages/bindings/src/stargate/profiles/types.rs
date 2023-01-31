use crate::stargate::profiles::proto::{Base58Address, Bech32Address, HexAddress};
use desmos_std::shim::Any;

#[derive(Clone)]
pub enum AddressData {
    Bech32Address(Bech32Address),
    HexAddress(HexAddress),
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
