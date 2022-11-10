use crate::profiles::proto::{Base58Address, Bech32Address, HexAddress};
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
    fn try_from(value: Any) -> Result<Self, Self::Error> {
        if let Ok(address) = Bech32Address::decode(value.value.as_slice()) {
            return Ok(AddressData::Bech32Address(address));
        }
        if let Ok(address) = HexAddress::decode(value.value.as_slice()) {
            return Ok(AddressData::HexAddress(address));
        }
        if let Ok(address) = Base58Address::decode(value.value.as_slice()) {
            return Ok(AddressData::Base58Address(address));
        }
        Err(StdError::ParseErr {
            target_type: "AddressData".to_string(),
            msg: "Unmatched type: must be either `Bech32Address`, `HexAddress` or `Base58Address`."
                .to_string(),
        })
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
