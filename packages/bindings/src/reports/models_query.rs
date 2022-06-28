use crate::reports::models::{Reason, Report};
use crate::types::PageResponse;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryReportsResponse {
    pub reports: Vec<Report>,
    pub pagination: Option<PageResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryReportResponse {
    pub report: Report,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryReasonsResponse {
    pub reasons: Vec<Reason>,
    pub pagination: Option<PageResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryReasonResponse {
    pub reason: Reason,
}
