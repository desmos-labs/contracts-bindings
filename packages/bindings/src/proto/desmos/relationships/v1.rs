use std_derive::CosmwasmExt;
/// Relationship is the struct of a relationship.
/// It represent the concept of "follow" of traditional social networks.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.relationships.v1.Relationship")]
pub struct Relationship {
    /// Creator represents the creator of the relationship
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    /// Counterparty represents the other user involved in the relationship
    #[prost(string, tag = "2")]
    pub counterparty: ::prost::alloc::string::String,
    /// SubspaceID represents the id of the subspace for which the relationship is
    /// valid
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
}
/// UserBlock represents the fact that the Blocker has blocked the given Blocked
/// user.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.relationships.v1.UserBlock")]
pub struct UserBlock {
    /// Blocker represents the address of the user blocking another one
    #[prost(string, tag = "1")]
    pub blocker: ::prost::alloc::string::String,
    /// Blocked represents the address of the blocked user
    #[prost(string, tag = "2")]
    pub blocked: ::prost::alloc::string::String,
    /// Reason represents the optional reason the user has been blocked for.
    #[prost(string, tag = "3")]
    pub reason: ::prost::alloc::string::String,
    /// SubspaceID represents the ID of the subspace inside which the user should
    /// be blocked
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
}
/// QueryRelationshipsRequest is the request type for the
/// Query/Relationships RPC method.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.relationships.v1.QueryRelationshipsRequest")]
#[proto_query(
    path = "/desmos.relationships.v1.Query/Relationships",
    response_type = QueryRelationshipsResponse
)]
pub struct QueryRelationshipsRequest {
    /// subspace to query the relationships for
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// optional address of the user for which to query the relationships
    #[prost(string, tag = "2")]
    pub user: ::prost::alloc::string::String,
    /// optional address of the counterparty of the relationships (used only if the
    /// user is provided)
    #[prost(string, tag = "3")]
    pub counterparty: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "4")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryRelationshipsResponse is the response type for the
/// Query/Relationships RPC method.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.relationships.v1.QueryRelationshipsResponse")]
pub struct QueryRelationshipsResponse {
    #[prost(message, repeated, tag = "1")]
    pub relationships: ::prost::alloc::vec::Vec<Relationship>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryBlocksRequest is the request type for the Query/Blocks RPC
/// endpoint
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.relationships.v1.QueryBlocksRequest")]
#[proto_query(
    path = "/desmos.relationships.v1.Query/Blocks",
    response_type = QueryBlocksResponse
)]
pub struct QueryBlocksRequest {
    /// subspace to query the blocks for
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// optional address of the blocker to query the blocks for
    #[prost(string, tag = "2")]
    pub blocker: ::prost::alloc::string::String,
    /// optional address of the blocked user to query the block for (used only if
    /// the blocker is provided)
    #[prost(string, tag = "3")]
    pub blocked: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "4")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryBlocksResponse is the response type for the Query/Blocks RPC
/// method.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.relationships.v1.QueryBlocksResponse")]
pub struct QueryBlocksResponse {
    #[prost(message, repeated, tag = "1")]
    pub blocks: ::prost::alloc::vec::Vec<UserBlock>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// MsgCreateRelationship represents a message to create a relationship
/// between two users on a specific subspace.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.relationships.v1.MsgCreateRelationship")]
pub struct MsgCreateRelationship {
    /// User creating the relationship
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    /// Counterparty of the relationship (i.e. user to be followed)
    #[prost(string, tag = "2")]
    pub counterparty: ::prost::alloc::string::String,
    /// Subspace id inside which the relationship will be valid
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
}
/// MsgCreateRelationshipResponse defines the Msg/CreateRelationship response
/// type.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.relationships.v1.MsgCreateRelationshipResponse")]
pub struct MsgCreateRelationshipResponse {}
/// MsgDeleteRelationship represents a message to delete the relationship
/// between two users.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.relationships.v1.MsgDeleteRelationship")]
pub struct MsgDeleteRelationship {
    /// User that created the relationship
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    /// Counterparty of the relationship that should be deleted
    #[prost(string, tag = "2")]
    pub counterparty: ::prost::alloc::string::String,
    /// Id of the subspace inside which the relationship to delete exists
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
}
/// MsgDeleteRelationshipResponse defines the Msg/DeleteRelationship response
/// type.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.relationships.v1.MsgDeleteRelationshipResponse")]
pub struct MsgDeleteRelationshipResponse {}
/// MsgBlockUser represents a message to block another user specifying an
/// optional reason.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.relationships.v1.MsgBlockUser")]
pub struct MsgBlockUser {
    /// Address of the user blocking the other user
    #[prost(string, tag = "1")]
    pub blocker: ::prost::alloc::string::String,
    /// Address of the user that should be blocked
    #[prost(string, tag = "2")]
    pub blocked: ::prost::alloc::string::String,
    /// (optional) Reason why the user has been blocked
    #[prost(string, tag = "3")]
    pub reason: ::prost::alloc::string::String,
    /// Id of the subspace inside which the user should be blocked
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
}
/// MsgBlockUserResponse defines the Msg/BlockUser response type.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.relationships.v1.MsgBlockUserResponse")]
pub struct MsgBlockUserResponse {}
/// MsgUnblockUser represents a message to unblock a previously blocked user.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.relationships.v1.MsgUnblockUser")]
pub struct MsgUnblockUser {
    /// Address of the user that blocked another user
    #[prost(string, tag = "1")]
    pub blocker: ::prost::alloc::string::String,
    /// Address of the user that should be unblocked
    #[prost(string, tag = "2")]
    pub blocked: ::prost::alloc::string::String,
    /// Id of the subspace inside which the user should be unblocked
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
}
/// MsgUnblockUserResponse defines the Msg/UnblockUser response type.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.relationships.v1.MsgUnblockUserResponse")]
pub struct MsgUnblockUserResponse {}
pub struct RelationshipsQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> RelationshipsQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn relationships(
        &self,
        subspace_id: u64,
        user: ::prost::alloc::string::String,
        counterparty: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryRelationshipsResponse, cosmwasm_std::StdError> {
        QueryRelationshipsRequest {
            subspace_id,
            user,
            counterparty,
            pagination,
        }
        .query(self.querier)
    }
    pub fn blocks(
        &self,
        subspace_id: u64,
        blocker: ::prost::alloc::string::String,
        blocked: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryBlocksResponse, cosmwasm_std::StdError> {
        QueryBlocksRequest {
            subspace_id,
            blocker,
            blocked,
            pagination,
        }
        .query(self.querier)
    }
}
