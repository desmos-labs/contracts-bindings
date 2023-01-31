use crate::stargate::reports::proto::{PostTarget, UserTarget};
use crate::stargate::types::Any;
use cosmwasm_std::StdError;
use prost::Message;

#[derive(Clone)]
pub enum ReportTarget {
    UserTarget(UserTarget),
    PostTarget(PostTarget),
}

impl Into<Any> for ReportTarget {
    fn into(self) -> Any {
        match self {
            ReportTarget::UserTarget(target) => target.into(),
            ReportTarget::PostTarget(target) => target.into(),
        }
    }
}
