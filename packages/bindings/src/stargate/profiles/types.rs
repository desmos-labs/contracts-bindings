use crate::stargate::profiles::proto::{Base58Address, Bech32Address, HexAddress};
use cosmwasm_std::StdError;
use desmos_std::shim::Any;
use prost::Message;

#[derive(Clone)]
pub enum AddressData {
    Bech32Address(Bech32Address),
    HexAddress(HexAddress),
    Base58Address(Base58Address),
}

impl TryFrom<Any> for AddressData {
    type Error = StdError;
    fn try_from(any: Any) -> Result<Self, Self::Error> {
        match any.type_url {
            Bech32Address::TYPE_URL => Bech32Address::try_from(any),
            HexAddress::TYPE_URL =>HexAddress::try_from(any),
            Base58Address::TYPE_URL =>  Base58Address::try_from(any),
            _ => Err(StdError::parse_err("AddressData",  "Unmatched type: must be either `Bech32Address`, `HexAddress` or `Base58Address`."))
        }
    }
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
