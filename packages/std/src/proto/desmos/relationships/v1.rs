/// Relationship is the struct of a relationship.
/// It represent the concept of "follow" of traditional social networks.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
    pub subspace_id: u64,
}
/// UserBlock represents the fact that the Blocker has blocked the given Blocked
/// user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
    pub subspace_id: u64,
}
/// QueryRelationshipsRequest is the request type for the
/// Query/Relationships RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.relationships.v1.QueryRelationshipsRequest")]
#[proto_query(
    path = "/desmos.relationships.v1.Query/Relationships",
    response_type = QueryRelationshipsResponse
)]
pub struct QueryRelationshipsRequest {
    /// subspace to query the relationships for
    #[prost(uint64, tag = "1")]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.relationships.v1.QueryBlocksRequest")]
#[proto_query(
    path = "/desmos.relationships.v1.Query/Blocks",
    response_type = QueryBlocksResponse
)]
pub struct QueryBlocksRequest {
    /// subspace to query the blocks for
    #[prost(uint64, tag = "1")]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
    pub subspace_id: u64,
}
/// MsgCreateRelationshipResponse defines the Msg/CreateRelationship response
/// type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.relationships.v1.MsgCreateRelationshipResponse")]
pub struct MsgCreateRelationshipResponse {}
/// MsgDeleteRelationship represents a message to delete the relationship
/// between two users.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
    pub subspace_id: u64,
}
/// MsgDeleteRelationshipResponse defines the Msg/DeleteRelationship response
/// type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.relationships.v1.MsgDeleteRelationshipResponse")]
pub struct MsgDeleteRelationshipResponse {}
/// MsgBlockUser represents a message to block another user specifying an
/// optional reason.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
    pub subspace_id: u64,
}
/// MsgBlockUserResponse defines the Msg/BlockUser response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.relationships.v1.MsgBlockUserResponse")]
pub struct MsgBlockUserResponse {}
/// MsgUnblockUser represents a message to unblock a previously blocked user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
    pub subspace_id: u64,
}
/// MsgUnblockUserResponse defines the Msg/UnblockUser response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
    ) -> std::result::Result<QueryRelationshipsResponse, cosmwasm_std::StdError> {
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
    ) -> std::result::Result<QueryBlocksResponse, cosmwasm_std::StdError> {
        QueryBlocksRequest {
            subspace_id,
            blocker,
            blocked,
            pagination,
        }
        .query(self.querier)
    }
}
impl serde::Serialize for MsgBlockUser {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.blocker.is_empty() {
            len += 1;
        }
        if !self.blocked.is_empty() {
            len += 1;
        }
        if !self.reason.is_empty() {
            len += 1;
        }
        if self.subspace_id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.relationships.v1.MsgBlockUser", len)?;
        if !self.blocker.is_empty() {
            struct_ser.serialize_field("blocker", &self.blocker)?;
        }
        if !self.blocked.is_empty() {
            struct_ser.serialize_field("blocked", &self.blocked)?;
        }
        if !self.reason.is_empty() {
            struct_ser.serialize_field("reason", &self.reason)?;
        }
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgBlockUser {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["blocker", "blocked", "reason", "subspace_id", "subspaceId"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Blocker,
            Blocked,
            Reason,
            SubspaceId,
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
                            "blocker" => Ok(GeneratedField::Blocker),
                            "blocked" => Ok(GeneratedField::Blocked),
                            "reason" => Ok(GeneratedField::Reason),
                            "subspaceId" | "subspace_id" => Ok(GeneratedField::SubspaceId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgBlockUser;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.relationships.v1.MsgBlockUser")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgBlockUser, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut blocker__ = None;
                let mut blocked__ = None;
                let mut reason__ = None;
                let mut subspace_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Blocker => {
                            if blocker__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blocker"));
                            }
                            blocker__ = Some(map.next_value()?);
                        }
                        GeneratedField::Blocked => {
                            if blocked__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blocked"));
                            }
                            blocked__ = Some(map.next_value()?);
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map.next_value()?);
                        }
                        GeneratedField::SubspaceId => {
                            if subspace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subspaceId"));
                            }
                            subspace_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgBlockUser {
                    blocker: blocker__.unwrap_or_default(),
                    blocked: blocked__.unwrap_or_default(),
                    reason: reason__.unwrap_or_default(),
                    subspace_id: subspace_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.relationships.v1.MsgBlockUser",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgBlockUserResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("desmos.relationships.v1.MsgBlockUserResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgBlockUserResponse {
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
            type Value = MsgBlockUserResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.relationships.v1.MsgBlockUserResponse")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgBlockUserResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgBlockUserResponse {})
            }
        }
        deserializer.deserialize_struct(
            "desmos.relationships.v1.MsgBlockUserResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgCreateRelationship {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.signer.is_empty() {
            len += 1;
        }
        if !self.counterparty.is_empty() {
            len += 1;
        }
        if self.subspace_id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.relationships.v1.MsgCreateRelationship", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if !self.counterparty.is_empty() {
            struct_ser.serialize_field("counterparty", &self.counterparty)?;
        }
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCreateRelationship {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "counterparty", "subspace_id", "subspaceId"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            Counterparty,
            SubspaceId,
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
                            "signer" => Ok(GeneratedField::Signer),
                            "counterparty" => Ok(GeneratedField::Counterparty),
                            "subspaceId" | "subspace_id" => Ok(GeneratedField::SubspaceId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgCreateRelationship;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.relationships.v1.MsgCreateRelationship")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgCreateRelationship, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut counterparty__ = None;
                let mut subspace_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map.next_value()?);
                        }
                        GeneratedField::Counterparty => {
                            if counterparty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("counterparty"));
                            }
                            counterparty__ = Some(map.next_value()?);
                        }
                        GeneratedField::SubspaceId => {
                            if subspace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subspaceId"));
                            }
                            subspace_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgCreateRelationship {
                    signer: signer__.unwrap_or_default(),
                    counterparty: counterparty__.unwrap_or_default(),
                    subspace_id: subspace_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.relationships.v1.MsgCreateRelationship",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgCreateRelationshipResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("desmos.relationships.v1.MsgCreateRelationshipResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCreateRelationshipResponse {
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
            type Value = MsgCreateRelationshipResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.relationships.v1.MsgCreateRelationshipResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgCreateRelationshipResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgCreateRelationshipResponse {})
            }
        }
        deserializer.deserialize_struct(
            "desmos.relationships.v1.MsgCreateRelationshipResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgDeleteRelationship {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.signer.is_empty() {
            len += 1;
        }
        if !self.counterparty.is_empty() {
            len += 1;
        }
        if self.subspace_id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.relationships.v1.MsgDeleteRelationship", len)?;
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        if !self.counterparty.is_empty() {
            struct_ser.serialize_field("counterparty", &self.counterparty)?;
        }
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgDeleteRelationship {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signer", "counterparty", "subspace_id", "subspaceId"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signer,
            Counterparty,
            SubspaceId,
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
                            "signer" => Ok(GeneratedField::Signer),
                            "counterparty" => Ok(GeneratedField::Counterparty),
                            "subspaceId" | "subspace_id" => Ok(GeneratedField::SubspaceId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgDeleteRelationship;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.relationships.v1.MsgDeleteRelationship")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgDeleteRelationship, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signer__ = None;
                let mut counterparty__ = None;
                let mut subspace_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map.next_value()?);
                        }
                        GeneratedField::Counterparty => {
                            if counterparty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("counterparty"));
                            }
                            counterparty__ = Some(map.next_value()?);
                        }
                        GeneratedField::SubspaceId => {
                            if subspace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subspaceId"));
                            }
                            subspace_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgDeleteRelationship {
                    signer: signer__.unwrap_or_default(),
                    counterparty: counterparty__.unwrap_or_default(),
                    subspace_id: subspace_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.relationships.v1.MsgDeleteRelationship",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgDeleteRelationshipResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("desmos.relationships.v1.MsgDeleteRelationshipResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgDeleteRelationshipResponse {
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
            type Value = MsgDeleteRelationshipResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.relationships.v1.MsgDeleteRelationshipResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgDeleteRelationshipResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgDeleteRelationshipResponse {})
            }
        }
        deserializer.deserialize_struct(
            "desmos.relationships.v1.MsgDeleteRelationshipResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUnblockUser {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.blocker.is_empty() {
            len += 1;
        }
        if !self.blocked.is_empty() {
            len += 1;
        }
        if self.subspace_id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.relationships.v1.MsgUnblockUser", len)?;
        if !self.blocker.is_empty() {
            struct_ser.serialize_field("blocker", &self.blocker)?;
        }
        if !self.blocked.is_empty() {
            struct_ser.serialize_field("blocked", &self.blocked)?;
        }
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUnblockUser {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["blocker", "blocked", "subspace_id", "subspaceId"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Blocker,
            Blocked,
            SubspaceId,
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
                            "blocker" => Ok(GeneratedField::Blocker),
                            "blocked" => Ok(GeneratedField::Blocked),
                            "subspaceId" | "subspace_id" => Ok(GeneratedField::SubspaceId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUnblockUser;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.relationships.v1.MsgUnblockUser")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgUnblockUser, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut blocker__ = None;
                let mut blocked__ = None;
                let mut subspace_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Blocker => {
                            if blocker__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blocker"));
                            }
                            blocker__ = Some(map.next_value()?);
                        }
                        GeneratedField::Blocked => {
                            if blocked__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blocked"));
                            }
                            blocked__ = Some(map.next_value()?);
                        }
                        GeneratedField::SubspaceId => {
                            if subspace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subspaceId"));
                            }
                            subspace_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgUnblockUser {
                    blocker: blocker__.unwrap_or_default(),
                    blocked: blocked__.unwrap_or_default(),
                    subspace_id: subspace_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.relationships.v1.MsgUnblockUser",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUnblockUserResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("desmos.relationships.v1.MsgUnblockUserResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUnblockUserResponse {
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
            type Value = MsgUnblockUserResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.relationships.v1.MsgUnblockUserResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgUnblockUserResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUnblockUserResponse {})
            }
        }
        deserializer.deserialize_struct(
            "desmos.relationships.v1.MsgUnblockUserResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryBlocksRequest {
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
        if !self.blocker.is_empty() {
            len += 1;
        }
        if !self.blocked.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.relationships.v1.QueryBlocksRequest", len)?;
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        if !self.blocker.is_empty() {
            struct_ser.serialize_field("blocker", &self.blocker)?;
        }
        if !self.blocked.is_empty() {
            struct_ser.serialize_field("blocked", &self.blocked)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBlocksRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subspace_id",
            "subspaceId",
            "blocker",
            "blocked",
            "pagination",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspaceId,
            Blocker,
            Blocked,
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
                            "blocker" => Ok(GeneratedField::Blocker),
                            "blocked" => Ok(GeneratedField::Blocked),
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
            type Value = QueryBlocksRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.relationships.v1.QueryBlocksRequest")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryBlocksRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
                let mut blocker__ = None;
                let mut blocked__ = None;
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
                        GeneratedField::Blocker => {
                            if blocker__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blocker"));
                            }
                            blocker__ = Some(map.next_value()?);
                        }
                        GeneratedField::Blocked => {
                            if blocked__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blocked"));
                            }
                            blocked__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryBlocksRequest {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    blocker: blocker__.unwrap_or_default(),
                    blocked: blocked__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.relationships.v1.QueryBlocksRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryBlocksResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.blocks.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.relationships.v1.QueryBlocksResponse", len)?;
        if !self.blocks.is_empty() {
            struct_ser.serialize_field("blocks", &self.blocks)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBlocksResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["blocks", "pagination"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Blocks,
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
                            "blocks" => Ok(GeneratedField::Blocks),
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
            type Value = QueryBlocksResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.relationships.v1.QueryBlocksResponse")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryBlocksResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut blocks__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Blocks => {
                            if blocks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blocks"));
                            }
                            blocks__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryBlocksResponse {
                    blocks: blocks__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.relationships.v1.QueryBlocksResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryRelationshipsRequest {
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
        if !self.user.is_empty() {
            len += 1;
        }
        if !self.counterparty.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("desmos.relationships.v1.QueryRelationshipsRequest", len)?;
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        if !self.counterparty.is_empty() {
            struct_ser.serialize_field("counterparty", &self.counterparty)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRelationshipsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subspace_id",
            "subspaceId",
            "user",
            "counterparty",
            "pagination",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspaceId,
            User,
            Counterparty,
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
                            "user" => Ok(GeneratedField::User),
                            "counterparty" => Ok(GeneratedField::Counterparty),
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
            type Value = QueryRelationshipsRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.relationships.v1.QueryRelationshipsRequest")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryRelationshipsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
                let mut user__ = None;
                let mut counterparty__ = None;
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
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = Some(map.next_value()?);
                        }
                        GeneratedField::Counterparty => {
                            if counterparty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("counterparty"));
                            }
                            counterparty__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryRelationshipsRequest {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    user: user__.unwrap_or_default(),
                    counterparty: counterparty__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.relationships.v1.QueryRelationshipsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryRelationshipsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.relationships.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("desmos.relationships.v1.QueryRelationshipsResponse", len)?;
        if !self.relationships.is_empty() {
            struct_ser.serialize_field("relationships", &self.relationships)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRelationshipsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["relationships", "pagination"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Relationships,
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
                            "relationships" => Ok(GeneratedField::Relationships),
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
            type Value = QueryRelationshipsResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.relationships.v1.QueryRelationshipsResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryRelationshipsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut relationships__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Relationships => {
                            if relationships__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationships"));
                            }
                            relationships__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryRelationshipsResponse {
                    relationships: relationships__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.relationships.v1.QueryRelationshipsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Relationship {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.creator.is_empty() {
            len += 1;
        }
        if !self.counterparty.is_empty() {
            len += 1;
        }
        if self.subspace_id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.relationships.v1.Relationship", len)?;
        if !self.creator.is_empty() {
            struct_ser.serialize_field("creator", &self.creator)?;
        }
        if !self.counterparty.is_empty() {
            struct_ser.serialize_field("counterparty", &self.counterparty)?;
        }
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Relationship {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["creator", "counterparty", "subspace_id", "subspaceId"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Creator,
            Counterparty,
            SubspaceId,
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
                            "creator" => Ok(GeneratedField::Creator),
                            "counterparty" => Ok(GeneratedField::Counterparty),
                            "subspaceId" | "subspace_id" => Ok(GeneratedField::SubspaceId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Relationship;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.relationships.v1.Relationship")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Relationship, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut creator__ = None;
                let mut counterparty__ = None;
                let mut subspace_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Creator => {
                            if creator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creator"));
                            }
                            creator__ = Some(map.next_value()?);
                        }
                        GeneratedField::Counterparty => {
                            if counterparty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("counterparty"));
                            }
                            counterparty__ = Some(map.next_value()?);
                        }
                        GeneratedField::SubspaceId => {
                            if subspace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subspaceId"));
                            }
                            subspace_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(Relationship {
                    creator: creator__.unwrap_or_default(),
                    counterparty: counterparty__.unwrap_or_default(),
                    subspace_id: subspace_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.relationships.v1.Relationship",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for UserBlock {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.blocker.is_empty() {
            len += 1;
        }
        if !self.blocked.is_empty() {
            len += 1;
        }
        if !self.reason.is_empty() {
            len += 1;
        }
        if self.subspace_id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.relationships.v1.UserBlock", len)?;
        if !self.blocker.is_empty() {
            struct_ser.serialize_field("blocker", &self.blocker)?;
        }
        if !self.blocked.is_empty() {
            struct_ser.serialize_field("blocked", &self.blocked)?;
        }
        if !self.reason.is_empty() {
            struct_ser.serialize_field("reason", &self.reason)?;
        }
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserBlock {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["blocker", "blocked", "reason", "subspace_id", "subspaceId"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Blocker,
            Blocked,
            Reason,
            SubspaceId,
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
                            "blocker" => Ok(GeneratedField::Blocker),
                            "blocked" => Ok(GeneratedField::Blocked),
                            "reason" => Ok(GeneratedField::Reason),
                            "subspaceId" | "subspace_id" => Ok(GeneratedField::SubspaceId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserBlock;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.relationships.v1.UserBlock")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<UserBlock, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut blocker__ = None;
                let mut blocked__ = None;
                let mut reason__ = None;
                let mut subspace_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Blocker => {
                            if blocker__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blocker"));
                            }
                            blocker__ = Some(map.next_value()?);
                        }
                        GeneratedField::Blocked => {
                            if blocked__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blocked"));
                            }
                            blocked__ = Some(map.next_value()?);
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map.next_value()?);
                        }
                        GeneratedField::SubspaceId => {
                            if subspace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subspaceId"));
                            }
                            subspace_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(UserBlock {
                    blocker: blocker__.unwrap_or_default(),
                    blocked: blocked__.unwrap_or_default(),
                    reason: reason__.unwrap_or_default(),
                    subspace_id: subspace_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.relationships.v1.UserBlock",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
