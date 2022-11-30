use crate::reports::proto::{PostTarget, UserTarget};
use crate::types::Any;
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
        if let Ok(content) = UserTarget::decode(value.value.as_slice()) {
            return Ok(ReportTarget::UserTarget(content));
        }
        if let Ok(content) = PostTarget::decode(value.value.as_slice()) {
            return Ok(ReportTarget::PostTarget(content));
        }
        Err(StdError::ParseErr {
            target_type: "ReportTarget".to_string(),
            msg: "Unmatched type: must be either `UserTarget`, `PostTarget` or `Base58Address`."
                .to_string(),
        })
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
