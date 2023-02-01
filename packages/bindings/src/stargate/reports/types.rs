use crate::stargate::types::Any;

pub use desmos_std::proto::desmos::reports::v1::*;

// Represents the target of the report.
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
