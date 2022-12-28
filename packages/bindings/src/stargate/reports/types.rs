use crate::stargate::reports::proto::{PostTarget, UserTarget};
use crate::stargate::types::Any;
use cosmwasm_std::StdError;
use prost::Message;

#[derive(Clone)]
pub enum ReportTarget {
    UserTarget(UserTarget),
    PostTarget(PostTarget),
}

impl TryFrom<Any> for ReportTarget {
    type Error = StdError;
    fn try_from(value: Any) -> Result<Self, Self::Error> {
        match any.type_url {
            UserTarget::TYPE_URL => UserTarget::try_from(any),
            PostTarget::TYPE_URL => PostTarget::try_from(any),
            _ => Err(StdError::ParseErr {
                target_type: "AddressData".to_string(),
                msg: "Unmatched type: must be either `UserTarget`, `PostTarget` or `Base58Address`."
                    .to_string(),
            })
        }
    }
}

impl Into<Any> for ReportTarget {
    fn into(self) -> Any {
        match self {
            ReportTarget::UserTarget(content) => content.into(),
            ReportTarget::PostTarget(content) => content.into(),
        }
    }
}
