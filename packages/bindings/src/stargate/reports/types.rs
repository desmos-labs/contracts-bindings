//! Contains the types definitions of x/reports.

use crate::stargate::types::Any;

pub use desmos_std::proto::desmos::reports::v1::*;

/// Represents a generic report target.
#[derive(Clone)]
pub enum ReportTarget {
    /// Represents the user target.
    UserTarget(UserTarget),

    /// Represents the post target.
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
