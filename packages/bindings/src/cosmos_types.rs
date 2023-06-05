//! Contains basic types of a cosmos sdk based chain.

pub use desmos_std::proto::cosmos::authz::v1beta1::{GenericAuthorization, Grant as AuthzGrant};
pub use desmos_std::proto::cosmos::base::query::v1beta1::PageRequest;
pub use desmos_std::proto::cosmos::crypto::{ed25519, secp256k1, secp256r1};
pub use desmos_std::proto::cosmos::feegrant::v1beta1::{
    AllowedMsgAllowance, BasicAllowance, PeriodicAllowance,
};
pub use desmos_std::proto::ibc::core::client::v1::Height;
pub use desmos_std::shim::{Any, Timestamp};

/// Represents a generic fee allowance.
pub enum Allowance {
    /// Represents a basic allowance
    BasicAllowance(BasicAllowance),

    /// Represents a periodic allowance
    PeriodicAllowance(PeriodicAllowance),

    /// Represents a msg allowance
    AllowedMsgAllowance(AllowedMsgAllowance),
}

impl Into<Any> for Allowance {
    fn into(self) -> Any {
        match self {
            Allowance::BasicAllowance(allowance) => allowance.into(),
            Allowance::PeriodicAllowance(allowance) => allowance.into(),
            Allowance::AllowedMsgAllowance(allowance) => allowance.into(),
        }
    }
}