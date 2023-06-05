//! Contains the types definitions of x/reports.

use crate::cosmos_types::Any;

pub use desmos_std::proto::desmos::reports::v1::*;

/// Represents a generic report target.
#[derive(Clone)]
pub enum ReportTarget {
    /// Represents the user target.
    User(UserTarget),

    /// Represents the post target.
    Post(PostTarget),
}

impl Into<Any> for ReportTarget {
    fn into(self) -> Any {
        match self {
            ReportTarget::User(target) => target.into(),
            ReportTarget::Post(target) => target.into(),
        }
    }
}
