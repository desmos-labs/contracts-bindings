use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use thiserror::Error;

const USER_REPORT_TARGET_TYPE_URI: &str = "/desmos.reports.v1.UserTarget";
const POST_REPORT_TARGET_TYPE_URI: &str = "/desmos.reports.v1.PostTarget";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Report {
    pub subspace_id: Uint64,
    pub id: Uint64,
    pub reasons_ids: Vec<u32>,
    pub message: Option<String>,
    pub reporter: Addr,
    pub target: RawReportTarget,
    pub creation_date: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct RawReportTarget {
    #[serde(rename = "@type")]
    type_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<Addr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_id: Option<Uint64>,
}

#[derive(Clone, Debug)]
pub enum ReportTarget {
    User { user: Addr },
    Post { post_id: Uint64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Reason {
    pub subspace_id: Uint64,
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
}

impl RawReportTarget {
    pub fn unwrap_raw(self) -> Result<ReportTarget, UnwrapReportTargetError> {
        ReportTarget::try_from(self)
    }
}

impl From<ReportTarget> for RawReportTarget {
    fn from(target: ReportTarget) -> Self {
        match target {
            ReportTarget::User { user } => RawReportTarget {
                type_uri: USER_REPORT_TARGET_TYPE_URI.to_string(),
                user: Some(user),
                post_id: None,
            },
            ReportTarget::Post { post_id } => RawReportTarget {
                type_uri: POST_REPORT_TARGET_TYPE_URI.to_string(),
                user: None,
                post_id: Some(post_id),
            },
        }
    }
}

/// Represents the errors that can occur when converting a [`RawReportTarget`] into a [`ReportTarget`].
#[derive(Error, Debug, Clone)]
pub enum UnwrapReportTargetError {
    /// Error that occur if [`RawReportTarget`] have an unknown type.
    #[error("unknown attachment type: {0}")]
    UnknownType(String),
    /// Error that occur if [`RawReportTarget`] have type `/desmos.reports.v1.UserTarget` but
    /// some fields are undefined.
    #[error("invalid user target report field {0} is none")]
    InvalidUserTarget(String),
    /// Error that occur if [`RawReportTarget`] have type `/desmos.reports.v1.PostTarget` but
    /// some fields are undefined.
    #[error("invalid post target report field {0} is none")]
    InvalidPostTarget(String),
}

impl TryFrom<RawReportTarget> for ReportTarget {
    type Error = UnwrapReportTargetError;

    fn try_from(value: RawReportTarget) -> Result<Self, Self::Error> {
        if value.type_uri == USER_REPORT_TARGET_TYPE_URI {
            Ok(ReportTarget::User {
                user: Addr::unchecked(value.user.ok_or(
                    UnwrapReportTargetError::InvalidUserTarget("user".to_string()),
                )?),
            })
        } else if value.type_uri == POST_REPORT_TARGET_TYPE_URI {
            Ok(ReportTarget::Post {
                post_id: value
                    .post_id
                    .ok_or(UnwrapReportTargetError::InvalidPostTarget(
                        "post_id".to_string(),
                    ))?,
            })
        } else {
            Err(UnwrapReportTargetError::UnknownType(value.type_uri))
        }
    }
}
