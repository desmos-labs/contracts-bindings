/// Report contains the data of a generic report
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.Report")]
#[serde(rename_all = "snake_case")]
pub struct Report {
    /// Id of the subspace for which the report has been created
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the report
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    /// Id of the reason this report has been created for
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub reasons_ids: ::prost::alloc::vec::Vec<u32>,
    /// (optional) Message attached to this report
    #[prost(string, tag = "4")]
    pub message: ::prost::alloc::string::String,
    /// Address of the reporter
    #[prost(string, tag = "5")]
    pub reporter: ::prost::alloc::string::String,
    /// Target of the report
    #[prost(message, optional, tag = "6")]
    pub target: ::core::option::Option<crate::shim::Any>,
    /// Time in which the report was created
    #[prost(message, optional, tag = "7")]
    pub creation_date: ::core::option::Option<crate::shim::Timestamp>,
}
/// UserTarget contains the data of a report about a user
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.UserTarget")]
#[serde(rename_all = "snake_case")]
pub struct UserTarget {
    /// Address of the reported user
    #[prost(string, tag = "1")]
    pub user: ::prost::alloc::string::String,
}
/// PostTarget contains the data of a report about a post
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.PostTarget")]
#[serde(rename_all = "snake_case")]
pub struct PostTarget {
    /// Id of the reported post
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub post_id: u64,
}
/// Reason contains the data about a reporting reason
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.Reason")]
#[serde(rename_all = "snake_case")]
pub struct Reason {
    /// Id of the subspace for which this reason is valid
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the reason inside the subspace
    #[prost(uint32, tag = "2")]
    pub id: u32,
    /// Title of the reason
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    /// (optional) Extended description of the reason and the cases it applies to
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
}
/// Params contains the module parameters
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.Params")]
#[serde(rename_all = "snake_case")]
pub struct Params {
    /// List of available reasons from which new subspaces can pick their default
    /// ones
    #[prost(message, repeated, tag = "1")]
    pub standard_reasons: ::prost::alloc::vec::Vec<StandardReason>,
}
/// StandardReason contains the data of a standard reason that can be picked and
/// used from different subspaces
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.StandardReason")]
#[serde(rename_all = "snake_case")]
pub struct StandardReason {
    /// Id of the reason inside the subspace
    #[prost(uint32, tag = "1")]
    pub id: u32,
    /// Title of the reason
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// (optional) Extended description of the reason and the cases it applies to
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
}
/// GenesisState defines the reports module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.GenesisState")]
#[serde(rename_all = "snake_case")]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub subspaces_data: ::prost::alloc::vec::Vec<SubspaceDataEntry>,
    #[prost(message, repeated, tag = "2")]
    pub reasons: ::prost::alloc::vec::Vec<Reason>,
    #[prost(message, repeated, tag = "3")]
    pub reports: ::prost::alloc::vec::Vec<Report>,
    #[prost(message, optional, tag = "4")]
    pub params: ::core::option::Option<Params>,
}
/// SubspaceDataEntry contains the data related to a single subspace
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.SubspaceDataEntry")]
#[serde(rename_all = "snake_case")]
pub struct SubspaceDataEntry {
    /// Id of the subspace to which the data relates
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the next reason inside the subspace
    #[prost(uint32, tag = "2")]
    pub reason_id: u32,
    /// Id of the next report inside the subspace
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub report_id: u64,
}
/// MsgCreateReport represents the message to be used to create a report
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.MsgCreateReport")]
#[serde(rename_all = "snake_case")]
pub struct MsgCreateReport {
    /// Id of the subspace for which the report should be stored
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the reason this report has been created for
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub reasons_ids: ::prost::alloc::vec::Vec<u32>,
    /// (optional) Message attached to this report
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
    /// Address of the reporter
    #[prost(string, tag = "4")]
    pub reporter: ::prost::alloc::string::String,
    /// Target of the report
    #[prost(message, optional, tag = "5")]
    pub target: ::core::option::Option<crate::shim::Any>,
}
/// MsgCreateReportResponse represents the Msg/CreateReport response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.MsgCreateReportResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgCreateReportResponse {
    /// Id of the newly created report
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub report_id: u64,
    /// Time in which the report was created
    #[prost(message, optional, tag = "2")]
    pub creation_date: ::core::option::Option<crate::shim::Timestamp>,
}
/// MsgDeleteReport represents the message to be used when deleting a report
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.MsgDeleteReport")]
#[serde(rename_all = "snake_case")]
pub struct MsgDeleteReport {
    /// Id of the subspace that contains the report to be deleted
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the report to be deleted
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub report_id: u64,
    /// Address of the user deleting the report
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgDeleteReportResponse represents the Msg/DeleteReport response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.MsgDeleteReportResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgDeleteReportResponse {}
/// MsgSupportStandardReason represents the message to be used when wanting to
/// support one reason from the module params
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.MsgSupportStandardReason")]
#[serde(rename_all = "snake_case")]
pub struct MsgSupportStandardReason {
    /// Id of the subspace for which to support the reason
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the reason that should be supported
    #[prost(uint32, tag = "2")]
    pub standard_reason_id: u32,
    /// Address of the user signing the message
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgSupportStandardReasonResponse represents the Msg/SupportStandardReason
/// response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.MsgSupportStandardReasonResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgSupportStandardReasonResponse {
    /// Id of the newly added reason
    #[prost(uint32, tag = "1")]
    pub reasons_ids: u32,
}
/// MsgAddReason represents the message to be used when adding a new supported
/// reason
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.MsgAddReason")]
#[serde(rename_all = "snake_case")]
pub struct MsgAddReason {
    /// Id of the subspace for which to add the reason
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Title of the reason
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// (optional) Extended description of the reason and the cases it applies to
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Address of the user adding the supported reason
    #[prost(string, tag = "4")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgAddReasonResponse represents the Msg/AddReason response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.MsgAddReasonResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgAddReasonResponse {
    /// Id of the newly supported reason
    #[prost(uint32, tag = "1")]
    pub reason_id: u32,
}
/// MsgRemoveReason represents the message to be used when removing an exiting
/// reporting reason
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.MsgRemoveReason")]
#[serde(rename_all = "snake_case")]
pub struct MsgRemoveReason {
    /// Id of the subspace from which to remove the reason
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the reason to be deleted
    #[prost(uint32, tag = "2")]
    pub reason_id: u32,
    /// Address of the user adding the supported reason
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgRemoveReasonResponse represents the Msg/RemoveReason response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.MsgRemoveReasonResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgRemoveReasonResponse {}
/// MsgUpdateParams is the Msg/UpdateParams request type.
///
/// Since: Desmos 5.0.0
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.MsgUpdateParams")]
#[serde(rename_all = "snake_case")]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless
    /// overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
///
/// Since: Desmos 5.0.0
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.MsgUpdateParamsResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgUpdateParamsResponse {}
/// QueryReportsResponse is the request type for Query/Reports RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.QueryReportsRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/desmos.reports.v1.Query/Reports",
    response_type = QueryReportsResponse
)]
pub struct QueryReportsRequest {
    /// Id of the subspace to query the reports for
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// (optional) Target to query the reports for
    #[prost(message, optional, tag = "2")]
    pub target: ::core::option::Option<crate::shim::Any>,
    /// (optional) User that reported the target.
    /// This is going to be used only if the target is also specified
    #[prost(string, tag = "3")]
    pub reporter: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "4")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryReportsResponse is the response type for Query/Reports RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.QueryReportsResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryReportsResponse {
    #[prost(message, repeated, tag = "1")]
    pub reports: ::prost::alloc::vec::Vec<Report>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryReportRequest is the request type for Query/Report RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.QueryReportRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/desmos.reports.v1.Query/Report",
    response_type = QueryReportResponse
)]
pub struct QueryReportRequest {
    /// Id of the subspace that holds the report to query for
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the report to query for
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub report_id: u64,
}
/// QueryReportResponse is the response type for Query/Report RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.QueryReportResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryReportResponse {
    #[prost(message, optional, tag = "1")]
    pub report: ::core::option::Option<Report>,
}
/// QueryReasonsRequest is the request type for Query/Reasons RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.QueryReasonsRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/desmos.reports.v1.Query/Reasons",
    response_type = QueryReasonsResponse
)]
pub struct QueryReasonsRequest {
    /// Id of the subspace to query the supported reporting reasons for
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryReasonsResponse is the response type for Query/Reasons RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.QueryReasonsResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryReasonsResponse {
    #[prost(message, repeated, tag = "1")]
    pub reasons: ::prost::alloc::vec::Vec<Reason>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryReasonRequest is the request type for Query/Reason RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.QueryReasonRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/desmos.reports.v1.Query/Reason",
    response_type = QueryReasonResponse
)]
pub struct QueryReasonRequest {
    /// Id of the subspace that holds the reason to query for
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the reason to query for
    #[prost(uint32, tag = "2")]
    pub reason_id: u32,
}
/// QueryReasonResponse is the response type for Query/Reason RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.QueryReasonResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryReasonResponse {
    #[prost(message, optional, tag = "1")]
    pub reason: ::core::option::Option<Reason>,
}
/// QueryParamsRequest is the request type for Query/Params RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.QueryParamsRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/desmos.reports.v1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for Query/Params RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reports.v1.QueryParamsResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
pub struct ReportsQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> ReportsQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn reports(
        &self,
        subspace_id: u64,
        target: ::core::option::Option<crate::shim::Any>,
        reporter: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> std::result::Result<QueryReportsResponse, cosmwasm_std::StdError> {
        QueryReportsRequest {
            subspace_id,
            target,
            reporter,
            pagination,
        }
        .query(self.querier)
    }
    pub fn report(
        &self,
        subspace_id: u64,
        report_id: u64,
    ) -> std::result::Result<QueryReportResponse, cosmwasm_std::StdError> {
        QueryReportRequest {
            subspace_id,
            report_id,
        }
        .query(self.querier)
    }
    pub fn reasons(
        &self,
        subspace_id: u64,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> std::result::Result<QueryReasonsResponse, cosmwasm_std::StdError> {
        QueryReasonsRequest {
            subspace_id,
            pagination,
        }
        .query(self.querier)
    }
    pub fn reason(
        &self,
        subspace_id: u64,
        reason_id: u32,
    ) -> std::result::Result<QueryReasonResponse, cosmwasm_std::StdError> {
        QueryReasonRequest {
            subspace_id,
            reason_id,
        }
        .query(self.querier)
    }
    pub fn params(&self) -> std::result::Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
}
