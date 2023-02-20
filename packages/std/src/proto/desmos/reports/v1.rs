/// Report contains the data of a generic report
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reports.v1.Report")]
pub struct Report {
    /// Id of the subspace for which the report has been created
    #[prost(uint64, tag = "1")]
    pub subspace_id: u64,
    /// Id of the report
    #[prost(uint64, tag = "2")]
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
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reports.v1.UserTarget")]
pub struct UserTarget {
    /// Address of the reported user
    #[prost(string, tag = "1")]
    pub user: ::prost::alloc::string::String,
}
/// PostTarget contains the data of a report about a post
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reports.v1.PostTarget")]
pub struct PostTarget {
    /// Id of the reported post
    #[prost(uint64, tag = "1")]
    pub post_id: u64,
}
/// Reason contains the data about a reporting reason
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reports.v1.Reason")]
pub struct Reason {
    /// Id of the subspace for which this reason is valid
    #[prost(uint64, tag = "1")]
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
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reports.v1.Params")]
pub struct Params {
    /// List of available reasons from which new subspaces can pick their default
    /// ones
    #[prost(message, repeated, tag = "1")]
    pub standard_reasons: ::prost::alloc::vec::Vec<StandardReason>,
}
/// StandardReason contains the data of a standard reason that can be picked and
/// used from different subspaces
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reports.v1.StandardReason")]
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
/// QueryReportsResponse is the request type for Query/Reports RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reports.v1.QueryReportsRequest")]
#[proto_query(
    path = "/desmos.reports.v1.Query/Reports",
    response_type = QueryReportsResponse
)]
pub struct QueryReportsRequest {
    /// Id of the subspace to query the reports for
    #[prost(uint64, tag = "1")]
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
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reports.v1.QueryReportsResponse")]
pub struct QueryReportsResponse {
    #[prost(message, repeated, tag = "1")]
    pub reports: ::prost::alloc::vec::Vec<Report>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryReportRequest is the request type for Query/Report RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reports.v1.QueryReportRequest")]
#[proto_query(
    path = "/desmos.reports.v1.Query/Report",
    response_type = QueryReportResponse
)]
pub struct QueryReportRequest {
    /// Id of the subspace that holds the report to query for
    #[prost(uint64, tag = "1")]
    pub subspace_id: u64,
    /// Id of the report to query for
    #[prost(uint64, tag = "2")]
    pub report_id: u64,
}
/// QueryReportResponse is the response type for Query/Report RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reports.v1.QueryReportResponse")]
pub struct QueryReportResponse {
    #[prost(message, optional, tag = "1")]
    pub report: ::core::option::Option<Report>,
}
/// QueryReasonsRequest is the request type for Query/Reasons RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reports.v1.QueryReasonsRequest")]
#[proto_query(
    path = "/desmos.reports.v1.Query/Reasons",
    response_type = QueryReasonsResponse
)]
pub struct QueryReasonsRequest {
    /// Id of the subspace to query the supported reporting reasons for
    #[prost(uint64, tag = "1")]
    pub subspace_id: u64,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryReasonsResponse is the response type for Query/Reasons RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reports.v1.QueryReasonsResponse")]
pub struct QueryReasonsResponse {
    #[prost(message, repeated, tag = "1")]
    pub reasons: ::prost::alloc::vec::Vec<Reason>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryReasonRequest is the request type for Query/Reason RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reports.v1.QueryReasonRequest")]
#[proto_query(
    path = "/desmos.reports.v1.Query/Reason",
    response_type = QueryReasonResponse
)]
pub struct QueryReasonRequest {
    /// Id of the subspace that holds the reason to query for
    #[prost(uint64, tag = "1")]
    pub subspace_id: u64,
    /// Id of the reason to query for
    #[prost(uint32, tag = "2")]
    pub reason_id: u32,
}
/// QueryReasonResponse is the response type for Query/Reason RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reports.v1.QueryReasonResponse")]
pub struct QueryReasonResponse {
    #[prost(message, optional, tag = "1")]
    pub reason: ::core::option::Option<Reason>,
}
/// QueryParamsRequest is the request type for Query/Params RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reports.v1.QueryParamsRequest")]
#[proto_query(
    path = "/desmos.reports.v1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for Query/Params RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reports.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// MsgCreateReport represents the message to be used to create a report
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reports.v1.MsgCreateReport")]
pub struct MsgCreateReport {
    /// Id of the subspace for which the report should be stored
    #[prost(uint64, tag = "1")]
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
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reports.v1.MsgCreateReportResponse")]
pub struct MsgCreateReportResponse {
    /// Id of the newly created report
    #[prost(uint64, tag = "1")]
    pub report_id: u64,
    /// Time in which the report was created
    #[prost(message, optional, tag = "2")]
    pub creation_date: ::core::option::Option<crate::shim::Timestamp>,
}
/// MsgDeleteReport represents the message to be used when deleting a report
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reports.v1.MsgDeleteReport")]
pub struct MsgDeleteReport {
    /// Id of the subspace that contains the report to be deleted
    #[prost(uint64, tag = "1")]
    pub subspace_id: u64,
    /// Id of the report to be deleted
    #[prost(uint64, tag = "2")]
    pub report_id: u64,
    /// Address of the user deleting the report
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgDeleteReportResponse represents the Msg/DeleteReport response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reports.v1.MsgDeleteReportResponse")]
pub struct MsgDeleteReportResponse {}
/// MsgSupportStandardReason represents the message to be used when wanting to
/// support one reason from the module params
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reports.v1.MsgSupportStandardReason")]
pub struct MsgSupportStandardReason {
    /// Id of the subspace for which to support the reason
    #[prost(uint64, tag = "1")]
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
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reports.v1.MsgSupportStandardReasonResponse")]
pub struct MsgSupportStandardReasonResponse {
    /// Id of the newly added reason
    #[prost(uint32, tag = "1")]
    pub reasons_ids: u32,
}
/// MsgAddReason represents the message to be used when adding a new supported
/// reason
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reports.v1.MsgAddReason")]
pub struct MsgAddReason {
    /// Id of the subspace for which to add the reason
    #[prost(uint64, tag = "1")]
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
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reports.v1.MsgAddReasonResponse")]
pub struct MsgAddReasonResponse {
    /// Id of the newly supported reason
    #[prost(uint32, tag = "1")]
    pub reason_id: u32,
}
/// MsgRemoveReason represents the message to be used when removing an exiting
/// reporting reason
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reports.v1.MsgRemoveReason")]
pub struct MsgRemoveReason {
    /// Id of the subspace from which to remove the reason
    #[prost(uint64, tag = "1")]
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
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reports.v1.MsgRemoveReasonResponse")]
pub struct MsgRemoveReasonResponse {}
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
impl serde::Serialize for MsgAddReason {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.subspace_id != 0 {
            len += 1;
        }
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.signer.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.reports.v1.MsgAddReason", len)?;
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddReason {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subspace_id",
            "subspaceId",
            "title",
            "description",
            "signer",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspaceId,
            Title,
            Description,
            Signer,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "subspaceId" | "subspace_id" => Ok(GeneratedField::SubspaceId),
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "signer" => Ok(GeneratedField::Signer),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgAddReason;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reports.v1.MsgAddReason")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgAddReason, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
                let mut title__ = None;
                let mut description__ = None;
                let mut signer__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SubspaceId => {
                            if subspace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subspaceId"));
                            }
                            subspace_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map.next_value()?);
                        }
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgAddReason {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    signer: signer__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("desmos.reports.v1.MsgAddReason", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgAddReasonResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reason_id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reports.v1.MsgAddReasonResponse", len)?;
        if self.reason_id != 0 {
            struct_ser.serialize_field("reasonId", &self.reason_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddReasonResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["reason_id", "reasonId"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReasonId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "reasonId" | "reason_id" => Ok(GeneratedField::ReasonId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgAddReasonResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reports.v1.MsgAddReasonResponse")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgAddReasonResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut reason_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ReasonId => {
                            if reason_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reasonId"));
                            }
                            reason_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgAddReasonResponse {
                    reason_id: reason_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reports.v1.MsgAddReasonResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgCreateReport {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.subspace_id != 0 {
            len += 1;
        }
        if !self.reasons_ids.is_empty() {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        if !self.reporter.is_empty() {
            len += 1;
        }
        if self.target.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reports.v1.MsgCreateReport", len)?;
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        if !self.reasons_ids.is_empty() {
            struct_ser.serialize_field("reasonsIds", &self.reasons_ids)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        if !self.reporter.is_empty() {
            struct_ser.serialize_field("reporter", &self.reporter)?;
        }
        if let Some(v) = self.target.as_ref() {
            struct_ser.serialize_field("target", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCreateReport {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subspace_id",
            "subspaceId",
            "reasons_ids",
            "reasonsIds",
            "message",
            "reporter",
            "target",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspaceId,
            ReasonsIds,
            Message,
            Reporter,
            Target,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "subspaceId" | "subspace_id" => Ok(GeneratedField::SubspaceId),
                            "reasonsIds" | "reasons_ids" => Ok(GeneratedField::ReasonsIds),
                            "message" => Ok(GeneratedField::Message),
                            "reporter" => Ok(GeneratedField::Reporter),
                            "target" => Ok(GeneratedField::Target),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgCreateReport;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reports.v1.MsgCreateReport")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgCreateReport, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
                let mut reasons_ids__ = None;
                let mut message__ = None;
                let mut reporter__ = None;
                let mut target__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SubspaceId => {
                            if subspace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subspaceId"));
                            }
                            subspace_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ReasonsIds => {
                            if reasons_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reasonsIds"));
                            }
                            reasons_ids__ = Some(
                                map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter()
                                    .map(|x| x.0)
                                    .collect(),
                            );
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map.next_value()?);
                        }
                        GeneratedField::Reporter => {
                            if reporter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reporter"));
                            }
                            reporter__ = Some(map.next_value()?);
                        }
                        GeneratedField::Target => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            target__ = map.next_value()?;
                        }
                    }
                }
                Ok(MsgCreateReport {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    reasons_ids: reasons_ids__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                    reporter: reporter__.unwrap_or_default(),
                    target: target__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reports.v1.MsgCreateReport",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgCreateReportResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.report_id != 0 {
            len += 1;
        }
        if self.creation_date.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reports.v1.MsgCreateReportResponse", len)?;
        if self.report_id != 0 {
            struct_ser
                .serialize_field("reportId", ToString::to_string(&self.report_id).as_str())?;
        }
        if let Some(v) = self.creation_date.as_ref() {
            struct_ser.serialize_field("creationDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCreateReportResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["report_id", "reportId", "creation_date", "creationDate"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReportId,
            CreationDate,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "reportId" | "report_id" => Ok(GeneratedField::ReportId),
                            "creationDate" | "creation_date" => Ok(GeneratedField::CreationDate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgCreateReportResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reports.v1.MsgCreateReportResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgCreateReportResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut report_id__ = None;
                let mut creation_date__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ReportId => {
                            if report_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportId"));
                            }
                            report_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::CreationDate => {
                            if creation_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationDate"));
                            }
                            creation_date__ = map.next_value()?;
                        }
                    }
                }
                Ok(MsgCreateReportResponse {
                    report_id: report_id__.unwrap_or_default(),
                    creation_date: creation_date__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reports.v1.MsgCreateReportResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgDeleteReport {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.subspace_id != 0 {
            len += 1;
        }
        if self.report_id != 0 {
            len += 1;
        }
        if !self.signer.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reports.v1.MsgDeleteReport", len)?;
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        if self.report_id != 0 {
            struct_ser
                .serialize_field("reportId", ToString::to_string(&self.report_id).as_str())?;
        }
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgDeleteReport {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subspace_id",
            "subspaceId",
            "report_id",
            "reportId",
            "signer",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspaceId,
            ReportId,
            Signer,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "subspaceId" | "subspace_id" => Ok(GeneratedField::SubspaceId),
                            "reportId" | "report_id" => Ok(GeneratedField::ReportId),
                            "signer" => Ok(GeneratedField::Signer),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgDeleteReport;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reports.v1.MsgDeleteReport")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgDeleteReport, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
                let mut report_id__ = None;
                let mut signer__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SubspaceId => {
                            if subspace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subspaceId"));
                            }
                            subspace_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ReportId => {
                            if report_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportId"));
                            }
                            report_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgDeleteReport {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    report_id: report_id__.unwrap_or_default(),
                    signer: signer__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reports.v1.MsgDeleteReport",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgDeleteReportResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("desmos.reports.v1.MsgDeleteReportResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgDeleteReportResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgDeleteReportResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reports.v1.MsgDeleteReportResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgDeleteReportResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgDeleteReportResponse {})
            }
        }
        deserializer.deserialize_struct(
            "desmos.reports.v1.MsgDeleteReportResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgRemoveReason {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.subspace_id != 0 {
            len += 1;
        }
        if self.reason_id != 0 {
            len += 1;
        }
        if !self.signer.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reports.v1.MsgRemoveReason", len)?;
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        if self.reason_id != 0 {
            struct_ser.serialize_field("reasonId", &self.reason_id)?;
        }
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRemoveReason {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subspace_id",
            "subspaceId",
            "reason_id",
            "reasonId",
            "signer",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspaceId,
            ReasonId,
            Signer,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "subspaceId" | "subspace_id" => Ok(GeneratedField::SubspaceId),
                            "reasonId" | "reason_id" => Ok(GeneratedField::ReasonId),
                            "signer" => Ok(GeneratedField::Signer),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRemoveReason;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reports.v1.MsgRemoveReason")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgRemoveReason, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
                let mut reason_id__ = None;
                let mut signer__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SubspaceId => {
                            if subspace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subspaceId"));
                            }
                            subspace_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ReasonId => {
                            if reason_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reasonId"));
                            }
                            reason_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgRemoveReason {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    reason_id: reason_id__.unwrap_or_default(),
                    signer: signer__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reports.v1.MsgRemoveReason",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgRemoveReasonResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("desmos.reports.v1.MsgRemoveReasonResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRemoveReasonResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRemoveReasonResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reports.v1.MsgRemoveReasonResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgRemoveReasonResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRemoveReasonResponse {})
            }
        }
        deserializer.deserialize_struct(
            "desmos.reports.v1.MsgRemoveReasonResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSupportStandardReason {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.subspace_id != 0 {
            len += 1;
        }
        if self.standard_reason_id != 0 {
            len += 1;
        }
        if !self.signer.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reports.v1.MsgSupportStandardReason", len)?;
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        if self.standard_reason_id != 0 {
            struct_ser.serialize_field("standardReasonId", &self.standard_reason_id)?;
        }
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSupportStandardReason {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subspace_id",
            "subspaceId",
            "standard_reason_id",
            "standardReasonId",
            "signer",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspaceId,
            StandardReasonId,
            Signer,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "subspaceId" | "subspace_id" => Ok(GeneratedField::SubspaceId),
                            "standardReasonId" | "standard_reason_id" => {
                                Ok(GeneratedField::StandardReasonId)
                            }
                            "signer" => Ok(GeneratedField::Signer),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSupportStandardReason;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reports.v1.MsgSupportStandardReason")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgSupportStandardReason, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
                let mut standard_reason_id__ = None;
                let mut signer__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SubspaceId => {
                            if subspace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subspaceId"));
                            }
                            subspace_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::StandardReasonId => {
                            if standard_reason_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("standardReasonId"));
                            }
                            standard_reason_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgSupportStandardReason {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    standard_reason_id: standard_reason_id__.unwrap_or_default(),
                    signer: signer__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reports.v1.MsgSupportStandardReason",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSupportStandardReasonResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reasons_ids != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("desmos.reports.v1.MsgSupportStandardReasonResponse", len)?;
        if self.reasons_ids != 0 {
            struct_ser.serialize_field("reasonsIds", &self.reasons_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSupportStandardReasonResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["reasons_ids", "reasonsIds"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReasonsIds,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "reasonsIds" | "reasons_ids" => Ok(GeneratedField::ReasonsIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSupportStandardReasonResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reports.v1.MsgSupportStandardReasonResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgSupportStandardReasonResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut reasons_ids__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ReasonsIds => {
                            if reasons_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reasonsIds"));
                            }
                            reasons_ids__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgSupportStandardReasonResponse {
                    reasons_ids: reasons_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reports.v1.MsgSupportStandardReasonResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Params {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.standard_reasons.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.reports.v1.Params", len)?;
        if !self.standard_reasons.is_empty() {
            struct_ser.serialize_field("standardReasons", &self.standard_reasons)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Params {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["standard_reasons", "standardReasons"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StandardReasons,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "standardReasons" | "standard_reasons" => {
                                Ok(GeneratedField::StandardReasons)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Params;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reports.v1.Params")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Params, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut standard_reasons__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StandardReasons => {
                            if standard_reasons__.is_some() {
                                return Err(serde::de::Error::duplicate_field("standardReasons"));
                            }
                            standard_reasons__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Params {
                    standard_reasons: standard_reasons__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("desmos.reports.v1.Params", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PostTarget {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.post_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.reports.v1.PostTarget", len)?;
        if self.post_id != 0 {
            struct_ser.serialize_field("postId", ToString::to_string(&self.post_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PostTarget {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["post_id", "postId"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PostId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "postId" | "post_id" => Ok(GeneratedField::PostId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PostTarget;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reports.v1.PostTarget")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<PostTarget, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut post_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PostId => {
                            if post_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("postId"));
                            }
                            post_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(PostTarget {
                    post_id: post_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("desmos.reports.v1.PostTarget", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryParamsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("desmos.reports.v1.QueryParamsRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryParamsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryParamsRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reports.v1.QueryParamsRequest")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryParamsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryParamsRequest {})
            }
        }
        deserializer.deserialize_struct(
            "desmos.reports.v1.QueryParamsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryParamsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.params.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reports.v1.QueryParamsResponse", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryParamsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["params"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "params" => Ok(GeneratedField::Params),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryParamsResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reports.v1.QueryParamsResponse")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryParamsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryParamsResponse { params: params__ })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reports.v1.QueryParamsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryReasonRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.subspace_id != 0 {
            len += 1;
        }
        if self.reason_id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reports.v1.QueryReasonRequest", len)?;
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        if self.reason_id != 0 {
            struct_ser.serialize_field("reasonId", &self.reason_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryReasonRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["subspace_id", "subspaceId", "reason_id", "reasonId"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspaceId,
            ReasonId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "subspaceId" | "subspace_id" => Ok(GeneratedField::SubspaceId),
                            "reasonId" | "reason_id" => Ok(GeneratedField::ReasonId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryReasonRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reports.v1.QueryReasonRequest")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryReasonRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
                let mut reason_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SubspaceId => {
                            if subspace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subspaceId"));
                            }
                            subspace_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ReasonId => {
                            if reason_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reasonId"));
                            }
                            reason_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryReasonRequest {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    reason_id: reason_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reports.v1.QueryReasonRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryReasonResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reason.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reports.v1.QueryReasonResponse", len)?;
        if let Some(v) = self.reason.as_ref() {
            struct_ser.serialize_field("reason", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryReasonResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["reason"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reason,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "reason" => Ok(GeneratedField::Reason),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryReasonResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reports.v1.QueryReasonResponse")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryReasonResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut reason__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryReasonResponse { reason: reason__ })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reports.v1.QueryReasonResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryReasonsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.subspace_id != 0 {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reports.v1.QueryReasonsRequest", len)?;
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryReasonsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["subspace_id", "subspaceId", "pagination"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspaceId,
            Pagination,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "subspaceId" | "subspace_id" => Ok(GeneratedField::SubspaceId),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryReasonsRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reports.v1.QueryReasonsRequest")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryReasonsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SubspaceId => {
                            if subspace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subspaceId"));
                            }
                            subspace_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryReasonsRequest {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reports.v1.QueryReasonsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryReasonsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.reasons.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reports.v1.QueryReasonsResponse", len)?;
        if !self.reasons.is_empty() {
            struct_ser.serialize_field("reasons", &self.reasons)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryReasonsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["reasons", "pagination"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reasons,
            Pagination,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "reasons" => Ok(GeneratedField::Reasons),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryReasonsResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reports.v1.QueryReasonsResponse")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryReasonsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut reasons__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Reasons => {
                            if reasons__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reasons"));
                            }
                            reasons__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryReasonsResponse {
                    reasons: reasons__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reports.v1.QueryReasonsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryReportRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.subspace_id != 0 {
            len += 1;
        }
        if self.report_id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reports.v1.QueryReportRequest", len)?;
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        if self.report_id != 0 {
            struct_ser
                .serialize_field("reportId", ToString::to_string(&self.report_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryReportRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["subspace_id", "subspaceId", "report_id", "reportId"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspaceId,
            ReportId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "subspaceId" | "subspace_id" => Ok(GeneratedField::SubspaceId),
                            "reportId" | "report_id" => Ok(GeneratedField::ReportId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryReportRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reports.v1.QueryReportRequest")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryReportRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
                let mut report_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SubspaceId => {
                            if subspace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subspaceId"));
                            }
                            subspace_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ReportId => {
                            if report_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportId"));
                            }
                            report_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryReportRequest {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    report_id: report_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reports.v1.QueryReportRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryReportResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.report.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reports.v1.QueryReportResponse", len)?;
        if let Some(v) = self.report.as_ref() {
            struct_ser.serialize_field("report", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryReportResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["report"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Report,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "report" => Ok(GeneratedField::Report),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryReportResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reports.v1.QueryReportResponse")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryReportResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut report__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Report => {
                            if report__.is_some() {
                                return Err(serde::de::Error::duplicate_field("report"));
                            }
                            report__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryReportResponse { report: report__ })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reports.v1.QueryReportResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryReportsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.subspace_id != 0 {
            len += 1;
        }
        if self.target.is_some() {
            len += 1;
        }
        if !self.reporter.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reports.v1.QueryReportsRequest", len)?;
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        if let Some(v) = self.target.as_ref() {
            struct_ser.serialize_field("target", v)?;
        }
        if !self.reporter.is_empty() {
            struct_ser.serialize_field("reporter", &self.reporter)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryReportsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subspace_id",
            "subspaceId",
            "target",
            "reporter",
            "pagination",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspaceId,
            Target,
            Reporter,
            Pagination,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "subspaceId" | "subspace_id" => Ok(GeneratedField::SubspaceId),
                            "target" => Ok(GeneratedField::Target),
                            "reporter" => Ok(GeneratedField::Reporter),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryReportsRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reports.v1.QueryReportsRequest")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryReportsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
                let mut target__ = None;
                let mut reporter__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SubspaceId => {
                            if subspace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subspaceId"));
                            }
                            subspace_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Target => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            target__ = map.next_value()?;
                        }
                        GeneratedField::Reporter => {
                            if reporter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reporter"));
                            }
                            reporter__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryReportsRequest {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    target: target__,
                    reporter: reporter__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reports.v1.QueryReportsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryReportsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.reports.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reports.v1.QueryReportsResponse", len)?;
        if !self.reports.is_empty() {
            struct_ser.serialize_field("reports", &self.reports)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryReportsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["reports", "pagination"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reports,
            Pagination,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "reports" => Ok(GeneratedField::Reports),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryReportsResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reports.v1.QueryReportsResponse")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryReportsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut reports__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Reports => {
                            if reports__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reports"));
                            }
                            reports__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryReportsResponse {
                    reports: reports__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reports.v1.QueryReportsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Reason {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.subspace_id != 0 {
            len += 1;
        }
        if self.id != 0 {
            len += 1;
        }
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.reports.v1.Reason", len)?;
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Reason {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["subspace_id", "subspaceId", "id", "title", "description"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspaceId,
            Id,
            Title,
            Description,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "subspaceId" | "subspace_id" => Ok(GeneratedField::SubspaceId),
                            "id" => Ok(GeneratedField::Id),
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Reason;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reports.v1.Reason")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Reason, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
                let mut id__ = None;
                let mut title__ = None;
                let mut description__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SubspaceId => {
                            if subspace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subspaceId"));
                            }
                            subspace_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Reason {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("desmos.reports.v1.Reason", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Report {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.subspace_id != 0 {
            len += 1;
        }
        if self.id != 0 {
            len += 1;
        }
        if !self.reasons_ids.is_empty() {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        if !self.reporter.is_empty() {
            len += 1;
        }
        if self.target.is_some() {
            len += 1;
        }
        if self.creation_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.reports.v1.Report", len)?;
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        if self.id != 0 {
            struct_ser.serialize_field("id", ToString::to_string(&self.id).as_str())?;
        }
        if !self.reasons_ids.is_empty() {
            struct_ser.serialize_field("reasonsIds", &self.reasons_ids)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        if !self.reporter.is_empty() {
            struct_ser.serialize_field("reporter", &self.reporter)?;
        }
        if let Some(v) = self.target.as_ref() {
            struct_ser.serialize_field("target", v)?;
        }
        if let Some(v) = self.creation_date.as_ref() {
            struct_ser.serialize_field("creationDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Report {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subspace_id",
            "subspaceId",
            "id",
            "reasons_ids",
            "reasonsIds",
            "message",
            "reporter",
            "target",
            "creation_date",
            "creationDate",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspaceId,
            Id,
            ReasonsIds,
            Message,
            Reporter,
            Target,
            CreationDate,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "subspaceId" | "subspace_id" => Ok(GeneratedField::SubspaceId),
                            "id" => Ok(GeneratedField::Id),
                            "reasonsIds" | "reasons_ids" => Ok(GeneratedField::ReasonsIds),
                            "message" => Ok(GeneratedField::Message),
                            "reporter" => Ok(GeneratedField::Reporter),
                            "target" => Ok(GeneratedField::Target),
                            "creationDate" | "creation_date" => Ok(GeneratedField::CreationDate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Report;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reports.v1.Report")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Report, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
                let mut id__ = None;
                let mut reasons_ids__ = None;
                let mut message__ = None;
                let mut reporter__ = None;
                let mut target__ = None;
                let mut creation_date__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SubspaceId => {
                            if subspace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subspaceId"));
                            }
                            subspace_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ReasonsIds => {
                            if reasons_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reasonsIds"));
                            }
                            reasons_ids__ = Some(
                                map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter()
                                    .map(|x| x.0)
                                    .collect(),
                            );
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map.next_value()?);
                        }
                        GeneratedField::Reporter => {
                            if reporter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reporter"));
                            }
                            reporter__ = Some(map.next_value()?);
                        }
                        GeneratedField::Target => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            target__ = map.next_value()?;
                        }
                        GeneratedField::CreationDate => {
                            if creation_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationDate"));
                            }
                            creation_date__ = map.next_value()?;
                        }
                    }
                }
                Ok(Report {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    reasons_ids: reasons_ids__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                    reporter: reporter__.unwrap_or_default(),
                    target: target__,
                    creation_date: creation_date__,
                })
            }
        }
        deserializer.deserialize_struct("desmos.reports.v1.Report", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StandardReason {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reports.v1.StandardReason", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StandardReason {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["id", "title", "description"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Title,
            Description,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StandardReason;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reports.v1.StandardReason")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<StandardReason, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut title__ = None;
                let mut description__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(StandardReason {
                    id: id__.unwrap_or_default(),
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reports.v1.StandardReason",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for UserTarget {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.reports.v1.UserTarget", len)?;
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserTarget {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["user"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "user" => Ok(GeneratedField::User),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserTarget;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reports.v1.UserTarget")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<UserTarget, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UserTarget {
                    user: user__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("desmos.reports.v1.UserTarget", FIELDS, GeneratedVisitor)
    }
}
