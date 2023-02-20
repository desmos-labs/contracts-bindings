/// Reaction contains the data of a single post reaction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reactions.v1.Reaction")]
pub struct Reaction {
    /// Id of the subspace inside which the reaction has been put
    #[prost(uint64, tag = "1")]
    pub subspace_id: u64,
    /// Id of the post to which the reaction is associated
    #[prost(uint64, tag = "2")]
    pub post_id: u64,
    /// Id of the reaction within the post
    #[prost(uint32, tag = "3")]
    pub id: u32,
    /// Value of the reaction.
    #[prost(message, optional, tag = "4")]
    pub value: ::core::option::Option<crate::shim::Any>,
    /// Author of the reaction
    #[prost(string, tag = "5")]
    pub author: ::prost::alloc::string::String,
}
/// RegisteredReactionValue contains the details of a reaction value that
/// references a reaction registered within the subspace
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reactions.v1.RegisteredReactionValue")]
pub struct RegisteredReactionValue {
    /// Id of the registered reaction
    #[prost(uint32, tag = "1")]
    pub registered_reaction_id: u32,
}
/// FreeTextValue contains the details of a reaction value that
/// is made of free text
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reactions.v1.FreeTextValue")]
pub struct FreeTextValue {
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
}
/// RegisteredReaction contains the details of a registered reaction within a
/// subspace
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reactions.v1.RegisteredReaction")]
pub struct RegisteredReaction {
    /// Id of the subspace for which this reaction has been registered
    #[prost(uint64, tag = "1")]
    pub subspace_id: u64,
    /// Id of the registered reaction
    #[prost(uint32, tag = "2")]
    pub id: u32,
    /// Unique shorthand code associated to this reaction
    #[prost(string, tag = "3")]
    pub shorthand_code: ::prost::alloc::string::String,
    /// Value that should be displayed when using this reaction
    #[prost(string, tag = "4")]
    pub display_value: ::prost::alloc::string::String,
}
/// SubspaceReactionsParams contains the params related to a single subspace
/// reactions
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reactions.v1.SubspaceReactionsParams")]
pub struct SubspaceReactionsParams {
    /// Id of the subspace for which these params are valid
    #[prost(uint64, tag = "1")]
    pub subspace_id: u64,
    /// Params related to RegisteredReactionValue reactions
    #[prost(message, optional, tag = "2")]
    pub registered_reaction: ::core::option::Option<RegisteredReactionValueParams>,
    /// Params related to FreeTextValue reactions
    #[prost(message, optional, tag = "3")]
    pub free_text: ::core::option::Option<FreeTextValueParams>,
}
/// FreeTextValueParams contains the params for FreeTextValue based reactions
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reactions.v1.FreeTextValueParams")]
pub struct FreeTextValueParams {
    /// Whether FreeTextValue reactions should be enabled
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    /// The max length that FreeTextValue reactions should have
    #[prost(uint32, tag = "2")]
    pub max_length: u32,
    /// RegEx that each FreeTextValue should respect.
    /// This is useful to limit what characters can be used as a reaction.
    #[prost(string, tag = "3")]
    pub reg_ex: ::prost::alloc::string::String,
}
/// RegisteredReactionValueParams contains the params for RegisteredReactionValue
/// based reactions
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reactions.v1.RegisteredReactionValueParams")]
pub struct RegisteredReactionValueParams {
    /// Whether RegisteredReactionValue reactions should be enabled
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
/// QueryReactionsRequest is the request type for the Query/Reactions RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reactions.v1.QueryReactionsRequest")]
#[proto_query(
    path = "/desmos.reactions.v1.Query/Reactions",
    response_type = QueryReactionsResponse
)]
pub struct QueryReactionsRequest {
    /// Id of the subspace that contains the post to query the reactions for
    #[prost(uint64, tag = "1")]
    pub subspace_id: u64,
    /// Post id to query the reactions for
    #[prost(uint64, tag = "2")]
    pub post_id: u64,
    /// (optional) User to query the reactions for.
    /// This is going to be used only if a post id is specified as well.
    #[prost(string, tag = "3")]
    pub user: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "4")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryReactionsResponse is the response type for the Query/Reactions RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reactions.v1.QueryReactionsResponse")]
pub struct QueryReactionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub reactions: ::prost::alloc::vec::Vec<Reaction>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryReactionRequest is the request type for the Query/ReactionRequest RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reactions.v1.QueryReactionRequest")]
#[proto_query(
    path = "/desmos.reactions.v1.Query/Reaction",
    response_type = QueryReactionResponse
)]
pub struct QueryReactionRequest {
    /// Id of the subspace that contains the post to query the reactions for
    #[prost(uint64, tag = "1")]
    pub subspace_id: u64,
    /// Post id to query the reactions for
    #[prost(uint64, tag = "2")]
    pub post_id: u64,
    /// Id of the reaction to query
    #[prost(uint32, tag = "3")]
    pub reaction_id: u32,
}
/// QueryReactionResponse is the response type for the Query/Reaction RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reactions.v1.QueryReactionResponse")]
pub struct QueryReactionResponse {
    #[prost(message, optional, tag = "1")]
    pub reaction: ::core::option::Option<Reaction>,
}
/// QueryRegisteredReactionsRequest is the request type for the
/// Query/RegisteredReactions RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reactions.v1.QueryRegisteredReactionsRequest")]
#[proto_query(
    path = "/desmos.reactions.v1.Query/RegisteredReactions",
    response_type = QueryRegisteredReactionsResponse
)]
pub struct QueryRegisteredReactionsRequest {
    /// Id of the subspace to query the registered reactions for
    #[prost(uint64, tag = "1")]
    pub subspace_id: u64,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryRegisteredReactionsResponse is the response type for the
/// Query/RegisteredReactions RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reactions.v1.QueryRegisteredReactionsResponse")]
pub struct QueryRegisteredReactionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub registered_reactions: ::prost::alloc::vec::Vec<RegisteredReaction>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryRegisteredReactionRequest is the request type for the
/// Query/RegisteredReaction RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reactions.v1.QueryRegisteredReactionRequest")]
#[proto_query(
    path = "/desmos.reactions.v1.Query/RegisteredReaction",
    response_type = QueryRegisteredReactionResponse
)]
pub struct QueryRegisteredReactionRequest {
    /// Id of the subspace to query the registered reactions for
    #[prost(uint64, tag = "1")]
    pub subspace_id: u64,
    /// Id of the registered reaction to query for
    #[prost(uint32, tag = "2")]
    pub reaction_id: u32,
}
/// QueryRegisteredReactionResponse is the response type for the
/// Query/RegisteredReaction RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reactions.v1.QueryRegisteredReactionResponse")]
pub struct QueryRegisteredReactionResponse {
    #[prost(message, optional, tag = "1")]
    pub registered_reaction: ::core::option::Option<RegisteredReaction>,
}
/// QueryReactionsParamsRequest is the request type for the Query/ReactionsParams
/// RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reactions.v1.QueryReactionsParamsRequest")]
#[proto_query(
    path = "/desmos.reactions.v1.Query/ReactionsParams",
    response_type = QueryReactionsParamsResponse
)]
pub struct QueryReactionsParamsRequest {
    /// Id of the subspace for which to query the params
    #[prost(uint64, tag = "1")]
    pub subspace_id: u64,
}
/// QueryReactionsParamsResponse is the response type for the
/// Query/ReactionsParam RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reactions.v1.QueryReactionsParamsResponse")]
pub struct QueryReactionsParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<SubspaceReactionsParams>,
}
/// MsgAddReaction represents the message to be used to add a post reaction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reactions.v1.MsgAddReaction")]
pub struct MsgAddReaction {
    /// Id of the subspace inside which the post to react to is
    #[prost(uint64, tag = "1")]
    pub subspace_id: u64,
    /// Id of the post to react to
    #[prost(uint64, tag = "2")]
    pub post_id: u64,
    /// Value of the reaction
    #[prost(message, optional, tag = "3")]
    pub value: ::core::option::Option<crate::shim::Any>,
    /// User reacting to the post
    #[prost(string, tag = "4")]
    pub user: ::prost::alloc::string::String,
}
/// MsgAddReactionResponse represents the Msg/AddReaction response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reactions.v1.MsgAddReactionResponse")]
pub struct MsgAddReactionResponse {
    /// Id of the newly added reaction
    #[prost(uint32, tag = "1")]
    pub reaction_id: u32,
}
/// MsgRemoveReaction represents the message to be used to remove an
/// existing reaction from a post
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reactions.v1.MsgRemoveReaction")]
pub struct MsgRemoveReaction {
    /// Id of the subspace inside which the reaction to remove is
    #[prost(uint64, tag = "1")]
    pub subspace_id: u64,
    /// Id of the post from which to remove the reaction
    #[prost(uint64, tag = "2")]
    pub post_id: u64,
    /// Id of the reaction to be removed
    #[prost(uint32, tag = "3")]
    pub reaction_id: u32,
    /// User removing the reaction
    #[prost(string, tag = "4")]
    pub user: ::prost::alloc::string::String,
}
/// MsgRemoveReactionResponse represents the Msg/RemoveReaction response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reactions.v1.MsgRemoveReactionResponse")]
pub struct MsgRemoveReactionResponse {}
/// MsgAddRegisteredReaction represents the message to be used to
/// register a new supported reaction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reactions.v1.MsgAddRegisteredReaction")]
pub struct MsgAddRegisteredReaction {
    /// Id of the subspace inside which this reaction should be registered
    #[prost(uint64, tag = "1")]
    pub subspace_id: u64,
    /// Shorthand code of the reaction
    #[prost(string, tag = "2")]
    pub shorthand_code: ::prost::alloc::string::String,
    /// Display value of the reaction
    #[prost(string, tag = "3")]
    pub display_value: ::prost::alloc::string::String,
    /// User adding the supported reaction
    #[prost(string, tag = "4")]
    pub user: ::prost::alloc::string::String,
}
/// MsgAddRegisteredReactionResponse represents the
/// Msg/AddRegisteredReaction response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reactions.v1.MsgAddRegisteredReactionResponse")]
pub struct MsgAddRegisteredReactionResponse {
    /// Id of the newly registered reaction
    #[prost(uint32, tag = "1")]
    pub registered_reaction_id: u32,
}
/// MsgEditRegisteredReaction represents the message to be used to edit a
/// registered reaction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reactions.v1.MsgEditRegisteredReaction")]
pub struct MsgEditRegisteredReaction {
    /// Id of the subspace inside which the reaction to edit is
    #[prost(uint64, tag = "1")]
    pub subspace_id: u64,
    /// Id of the registered reaction to edit
    #[prost(uint32, tag = "2")]
    pub registered_reaction_id: u32,
    /// New shorthand code to be set
    #[prost(string, tag = "3")]
    pub shorthand_code: ::prost::alloc::string::String,
    /// Display value to be set
    #[prost(string, tag = "4")]
    pub display_value: ::prost::alloc::string::String,
    /// User editing the registered reaction
    #[prost(string, tag = "5")]
    pub user: ::prost::alloc::string::String,
}
/// MsgEditRegisteredReactionResponse represents the Msg/EditRegisteredReaction
/// response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reactions.v1.MsgEditRegisteredReactionResponse")]
pub struct MsgEditRegisteredReactionResponse {}
/// MsgRemoveRegisteredReaction represents the message to be used to
/// remove an existing registered reaction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reactions.v1.MsgRemoveRegisteredReaction")]
pub struct MsgRemoveRegisteredReaction {
    /// Id of the subspace from which to remove the registered reaction
    #[prost(uint64, tag = "1")]
    pub subspace_id: u64,
    /// Id of the registered reaction to be removed
    #[prost(uint32, tag = "2")]
    pub registered_reaction_id: u32,
    /// User removing the registered reaction
    #[prost(string, tag = "3")]
    pub user: ::prost::alloc::string::String,
}
/// MsgRemoveRegisteredReactionResponse represents the
/// Msg/RemoveRegisteredReaction response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reactions.v1.MsgRemoveRegisteredReactionResponse")]
pub struct MsgRemoveRegisteredReactionResponse {}
/// MsgSetReactionsParams represents the message to be used when setting
/// a subspace reactions params
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reactions.v1.MsgSetReactionsParams")]
pub struct MsgSetReactionsParams {
    /// Id of the subspace for which to set the params
    #[prost(uint64, tag = "1")]
    pub subspace_id: u64,
    /// Params related to RegisteredReactionValue reactions
    #[prost(message, optional, tag = "2")]
    pub registered_reaction: ::core::option::Option<RegisteredReactionValueParams>,
    /// Params related to FreeTextValue reactions
    #[prost(message, optional, tag = "3")]
    pub free_text: ::core::option::Option<FreeTextValueParams>,
    /// User setting the params
    #[prost(string, tag = "4")]
    pub user: ::prost::alloc::string::String,
}
/// MsgSetReactionsParamsResponse represents the Msg/SetReactionsParams response
/// type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.reactions.v1.MsgSetReactionsParamsResponse")]
pub struct MsgSetReactionsParamsResponse {}
pub struct ReactionsQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> ReactionsQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn reactions(
        &self,
        subspace_id: u64,
        post_id: u64,
        user: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> std::result::Result<QueryReactionsResponse, cosmwasm_std::StdError> {
        QueryReactionsRequest {
            subspace_id,
            post_id,
            user,
            pagination,
        }
        .query(self.querier)
    }
    pub fn reaction(
        &self,
        subspace_id: u64,
        post_id: u64,
        reaction_id: u32,
    ) -> std::result::Result<QueryReactionResponse, cosmwasm_std::StdError> {
        QueryReactionRequest {
            subspace_id,
            post_id,
            reaction_id,
        }
        .query(self.querier)
    }
    pub fn registered_reactions(
        &self,
        subspace_id: u64,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> std::result::Result<QueryRegisteredReactionsResponse, cosmwasm_std::StdError> {
        QueryRegisteredReactionsRequest {
            subspace_id,
            pagination,
        }
        .query(self.querier)
    }
    pub fn registered_reaction(
        &self,
        subspace_id: u64,
        reaction_id: u32,
    ) -> std::result::Result<QueryRegisteredReactionResponse, cosmwasm_std::StdError> {
        QueryRegisteredReactionRequest {
            subspace_id,
            reaction_id,
        }
        .query(self.querier)
    }
    pub fn reactions_params(
        &self,
        subspace_id: u64,
    ) -> std::result::Result<QueryReactionsParamsResponse, cosmwasm_std::StdError> {
        QueryReactionsParamsRequest { subspace_id }.query(self.querier)
    }
}
impl serde::Serialize for FreeTextValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.text.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reactions.v1.FreeTextValue", len)?;
        if !self.text.is_empty() {
            struct_ser.serialize_field("text", &self.text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FreeTextValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["text"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Text,
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
                            "text" => Ok(GeneratedField::Text),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FreeTextValue;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reactions.v1.FreeTextValue")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<FreeTextValue, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut text__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Text => {
                            if text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            text__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FreeTextValue {
                    text: text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reactions.v1.FreeTextValue",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for FreeTextValueParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.enabled {
            len += 1;
        }
        if self.max_length != 0 {
            len += 1;
        }
        if !self.reg_ex.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reactions.v1.FreeTextValueParams", len)?;
        if self.enabled {
            struct_ser.serialize_field("enabled", &self.enabled)?;
        }
        if self.max_length != 0 {
            struct_ser.serialize_field("maxLength", &self.max_length)?;
        }
        if !self.reg_ex.is_empty() {
            struct_ser.serialize_field("regEx", &self.reg_ex)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FreeTextValueParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["enabled", "max_length", "maxLength", "reg_ex", "regEx"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Enabled,
            MaxLength,
            RegEx,
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
                            "enabled" => Ok(GeneratedField::Enabled),
                            "maxLength" | "max_length" => Ok(GeneratedField::MaxLength),
                            "regEx" | "reg_ex" => Ok(GeneratedField::RegEx),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FreeTextValueParams;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reactions.v1.FreeTextValueParams")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<FreeTextValueParams, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut enabled__ = None;
                let mut max_length__ = None;
                let mut reg_ex__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxLength => {
                            if max_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxLength"));
                            }
                            max_length__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::RegEx => {
                            if reg_ex__.is_some() {
                                return Err(serde::de::Error::duplicate_field("regEx"));
                            }
                            reg_ex__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FreeTextValueParams {
                    enabled: enabled__.unwrap_or_default(),
                    max_length: max_length__.unwrap_or_default(),
                    reg_ex: reg_ex__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reactions.v1.FreeTextValueParams",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgAddReaction {
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
        if self.post_id != 0 {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        if !self.user.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reactions.v1.MsgAddReaction", len)?;
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        if self.post_id != 0 {
            struct_ser.serialize_field("postId", ToString::to_string(&self.post_id).as_str())?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddReaction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subspace_id",
            "subspaceId",
            "post_id",
            "postId",
            "value",
            "user",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspaceId,
            PostId,
            Value,
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
                            "subspaceId" | "subspace_id" => Ok(GeneratedField::SubspaceId),
                            "postId" | "post_id" => Ok(GeneratedField::PostId),
                            "value" => Ok(GeneratedField::Value),
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
            type Value = MsgAddReaction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reactions.v1.MsgAddReaction")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgAddReaction, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
                let mut post_id__ = None;
                let mut value__ = None;
                let mut user__ = None;
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
                        GeneratedField::PostId => {
                            if post_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("postId"));
                            }
                            post_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map.next_value()?;
                        }
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgAddReaction {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    post_id: post_id__.unwrap_or_default(),
                    value: value__,
                    user: user__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reactions.v1.MsgAddReaction",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgAddReactionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reaction_id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reactions.v1.MsgAddReactionResponse", len)?;
        if self.reaction_id != 0 {
            struct_ser.serialize_field("reactionId", &self.reaction_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddReactionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["reaction_id", "reactionId"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReactionId,
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
                            "reactionId" | "reaction_id" => Ok(GeneratedField::ReactionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgAddReactionResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reactions.v1.MsgAddReactionResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgAddReactionResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut reaction_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ReactionId => {
                            if reaction_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reactionId"));
                            }
                            reaction_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgAddReactionResponse {
                    reaction_id: reaction_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reactions.v1.MsgAddReactionResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgAddRegisteredReaction {
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
        if !self.shorthand_code.is_empty() {
            len += 1;
        }
        if !self.display_value.is_empty() {
            len += 1;
        }
        if !self.user.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reactions.v1.MsgAddRegisteredReaction", len)?;
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        if !self.shorthand_code.is_empty() {
            struct_ser.serialize_field("shorthandCode", &self.shorthand_code)?;
        }
        if !self.display_value.is_empty() {
            struct_ser.serialize_field("displayValue", &self.display_value)?;
        }
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddRegisteredReaction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subspace_id",
            "subspaceId",
            "shorthand_code",
            "shorthandCode",
            "display_value",
            "displayValue",
            "user",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspaceId,
            ShorthandCode,
            DisplayValue,
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
                            "subspaceId" | "subspace_id" => Ok(GeneratedField::SubspaceId),
                            "shorthandCode" | "shorthand_code" => Ok(GeneratedField::ShorthandCode),
                            "displayValue" | "display_value" => Ok(GeneratedField::DisplayValue),
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
            type Value = MsgAddRegisteredReaction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reactions.v1.MsgAddRegisteredReaction")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgAddRegisteredReaction, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
                let mut shorthand_code__ = None;
                let mut display_value__ = None;
                let mut user__ = None;
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
                        GeneratedField::ShorthandCode => {
                            if shorthand_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shorthandCode"));
                            }
                            shorthand_code__ = Some(map.next_value()?);
                        }
                        GeneratedField::DisplayValue => {
                            if display_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayValue"));
                            }
                            display_value__ = Some(map.next_value()?);
                        }
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgAddRegisteredReaction {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    shorthand_code: shorthand_code__.unwrap_or_default(),
                    display_value: display_value__.unwrap_or_default(),
                    user: user__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reactions.v1.MsgAddRegisteredReaction",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgAddRegisteredReactionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.registered_reaction_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("desmos.reactions.v1.MsgAddRegisteredReactionResponse", len)?;
        if self.registered_reaction_id != 0 {
            struct_ser.serialize_field("registeredReactionId", &self.registered_reaction_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddRegisteredReactionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["registered_reaction_id", "registeredReactionId"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RegisteredReactionId,
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
                            "registeredReactionId" | "registered_reaction_id" => {
                                Ok(GeneratedField::RegisteredReactionId)
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
            type Value = MsgAddRegisteredReactionResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reactions.v1.MsgAddRegisteredReactionResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgAddRegisteredReactionResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut registered_reaction_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RegisteredReactionId => {
                            if registered_reaction_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "registeredReactionId",
                                ));
                            }
                            registered_reaction_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgAddRegisteredReactionResponse {
                    registered_reaction_id: registered_reaction_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reactions.v1.MsgAddRegisteredReactionResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgEditRegisteredReaction {
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
        if self.registered_reaction_id != 0 {
            len += 1;
        }
        if !self.shorthand_code.is_empty() {
            len += 1;
        }
        if !self.display_value.is_empty() {
            len += 1;
        }
        if !self.user.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reactions.v1.MsgEditRegisteredReaction", len)?;
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        if self.registered_reaction_id != 0 {
            struct_ser.serialize_field("registeredReactionId", &self.registered_reaction_id)?;
        }
        if !self.shorthand_code.is_empty() {
            struct_ser.serialize_field("shorthandCode", &self.shorthand_code)?;
        }
        if !self.display_value.is_empty() {
            struct_ser.serialize_field("displayValue", &self.display_value)?;
        }
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgEditRegisteredReaction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subspace_id",
            "subspaceId",
            "registered_reaction_id",
            "registeredReactionId",
            "shorthand_code",
            "shorthandCode",
            "display_value",
            "displayValue",
            "user",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspaceId,
            RegisteredReactionId,
            ShorthandCode,
            DisplayValue,
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
                            "subspaceId" | "subspace_id" => Ok(GeneratedField::SubspaceId),
                            "registeredReactionId" | "registered_reaction_id" => {
                                Ok(GeneratedField::RegisteredReactionId)
                            }
                            "shorthandCode" | "shorthand_code" => Ok(GeneratedField::ShorthandCode),
                            "displayValue" | "display_value" => Ok(GeneratedField::DisplayValue),
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
            type Value = MsgEditRegisteredReaction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reactions.v1.MsgEditRegisteredReaction")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgEditRegisteredReaction, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
                let mut registered_reaction_id__ = None;
                let mut shorthand_code__ = None;
                let mut display_value__ = None;
                let mut user__ = None;
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
                        GeneratedField::RegisteredReactionId => {
                            if registered_reaction_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "registeredReactionId",
                                ));
                            }
                            registered_reaction_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ShorthandCode => {
                            if shorthand_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shorthandCode"));
                            }
                            shorthand_code__ = Some(map.next_value()?);
                        }
                        GeneratedField::DisplayValue => {
                            if display_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayValue"));
                            }
                            display_value__ = Some(map.next_value()?);
                        }
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgEditRegisteredReaction {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    registered_reaction_id: registered_reaction_id__.unwrap_or_default(),
                    shorthand_code: shorthand_code__.unwrap_or_default(),
                    display_value: display_value__.unwrap_or_default(),
                    user: user__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reactions.v1.MsgEditRegisteredReaction",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgEditRegisteredReactionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("desmos.reactions.v1.MsgEditRegisteredReactionResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgEditRegisteredReactionResponse {
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
            type Value = MsgEditRegisteredReactionResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reactions.v1.MsgEditRegisteredReactionResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgEditRegisteredReactionResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgEditRegisteredReactionResponse {})
            }
        }
        deserializer.deserialize_struct(
            "desmos.reactions.v1.MsgEditRegisteredReactionResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgRemoveReaction {
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
        if self.post_id != 0 {
            len += 1;
        }
        if self.reaction_id != 0 {
            len += 1;
        }
        if !self.user.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reactions.v1.MsgRemoveReaction", len)?;
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        if self.post_id != 0 {
            struct_ser.serialize_field("postId", ToString::to_string(&self.post_id).as_str())?;
        }
        if self.reaction_id != 0 {
            struct_ser.serialize_field("reactionId", &self.reaction_id)?;
        }
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRemoveReaction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subspace_id",
            "subspaceId",
            "post_id",
            "postId",
            "reaction_id",
            "reactionId",
            "user",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspaceId,
            PostId,
            ReactionId,
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
                            "subspaceId" | "subspace_id" => Ok(GeneratedField::SubspaceId),
                            "postId" | "post_id" => Ok(GeneratedField::PostId),
                            "reactionId" | "reaction_id" => Ok(GeneratedField::ReactionId),
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
            type Value = MsgRemoveReaction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reactions.v1.MsgRemoveReaction")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgRemoveReaction, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
                let mut post_id__ = None;
                let mut reaction_id__ = None;
                let mut user__ = None;
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
                        GeneratedField::PostId => {
                            if post_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("postId"));
                            }
                            post_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ReactionId => {
                            if reaction_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reactionId"));
                            }
                            reaction_id__ = Some(
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
                    }
                }
                Ok(MsgRemoveReaction {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    post_id: post_id__.unwrap_or_default(),
                    reaction_id: reaction_id__.unwrap_or_default(),
                    user: user__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reactions.v1.MsgRemoveReaction",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgRemoveReactionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("desmos.reactions.v1.MsgRemoveReactionResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRemoveReactionResponse {
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
            type Value = MsgRemoveReactionResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reactions.v1.MsgRemoveReactionResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgRemoveReactionResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRemoveReactionResponse {})
            }
        }
        deserializer.deserialize_struct(
            "desmos.reactions.v1.MsgRemoveReactionResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgRemoveRegisteredReaction {
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
        if self.registered_reaction_id != 0 {
            len += 1;
        }
        if !self.user.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reactions.v1.MsgRemoveRegisteredReaction", len)?;
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        if self.registered_reaction_id != 0 {
            struct_ser.serialize_field("registeredReactionId", &self.registered_reaction_id)?;
        }
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRemoveRegisteredReaction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subspace_id",
            "subspaceId",
            "registered_reaction_id",
            "registeredReactionId",
            "user",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspaceId,
            RegisteredReactionId,
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
                            "subspaceId" | "subspace_id" => Ok(GeneratedField::SubspaceId),
                            "registeredReactionId" | "registered_reaction_id" => {
                                Ok(GeneratedField::RegisteredReactionId)
                            }
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
            type Value = MsgRemoveRegisteredReaction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reactions.v1.MsgRemoveRegisteredReaction")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgRemoveRegisteredReaction, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
                let mut registered_reaction_id__ = None;
                let mut user__ = None;
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
                        GeneratedField::RegisteredReactionId => {
                            if registered_reaction_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "registeredReactionId",
                                ));
                            }
                            registered_reaction_id__ = Some(
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
                    }
                }
                Ok(MsgRemoveRegisteredReaction {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    registered_reaction_id: registered_reaction_id__.unwrap_or_default(),
                    user: user__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reactions.v1.MsgRemoveRegisteredReaction",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgRemoveRegisteredReactionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct(
            "desmos.reactions.v1.MsgRemoveRegisteredReactionResponse",
            len,
        )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRemoveRegisteredReactionResponse {
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
            type Value = MsgRemoveRegisteredReactionResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct desmos.reactions.v1.MsgRemoveRegisteredReactionResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgRemoveRegisteredReactionResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRemoveRegisteredReactionResponse {})
            }
        }
        deserializer.deserialize_struct(
            "desmos.reactions.v1.MsgRemoveRegisteredReactionResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSetReactionsParams {
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
        if self.registered_reaction.is_some() {
            len += 1;
        }
        if self.free_text.is_some() {
            len += 1;
        }
        if !self.user.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reactions.v1.MsgSetReactionsParams", len)?;
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        if let Some(v) = self.registered_reaction.as_ref() {
            struct_ser.serialize_field("registeredReaction", v)?;
        }
        if let Some(v) = self.free_text.as_ref() {
            struct_ser.serialize_field("freeText", v)?;
        }
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetReactionsParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subspace_id",
            "subspaceId",
            "registered_reaction",
            "registeredReaction",
            "free_text",
            "freeText",
            "user",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspaceId,
            RegisteredReaction,
            FreeText,
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
                            "subspaceId" | "subspace_id" => Ok(GeneratedField::SubspaceId),
                            "registeredReaction" | "registered_reaction" => {
                                Ok(GeneratedField::RegisteredReaction)
                            }
                            "freeText" | "free_text" => Ok(GeneratedField::FreeText),
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
            type Value = MsgSetReactionsParams;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reactions.v1.MsgSetReactionsParams")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgSetReactionsParams, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
                let mut registered_reaction__ = None;
                let mut free_text__ = None;
                let mut user__ = None;
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
                        GeneratedField::RegisteredReaction => {
                            if registered_reaction__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "registeredReaction",
                                ));
                            }
                            registered_reaction__ = map.next_value()?;
                        }
                        GeneratedField::FreeText => {
                            if free_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("freeText"));
                            }
                            free_text__ = map.next_value()?;
                        }
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgSetReactionsParams {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    registered_reaction: registered_reaction__,
                    free_text: free_text__,
                    user: user__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reactions.v1.MsgSetReactionsParams",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSetReactionsParamsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("desmos.reactions.v1.MsgSetReactionsParamsResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetReactionsParamsResponse {
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
            type Value = MsgSetReactionsParamsResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reactions.v1.MsgSetReactionsParamsResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgSetReactionsParamsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSetReactionsParamsResponse {})
            }
        }
        deserializer.deserialize_struct(
            "desmos.reactions.v1.MsgSetReactionsParamsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryReactionRequest {
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
        if self.post_id != 0 {
            len += 1;
        }
        if self.reaction_id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reactions.v1.QueryReactionRequest", len)?;
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        if self.post_id != 0 {
            struct_ser.serialize_field("postId", ToString::to_string(&self.post_id).as_str())?;
        }
        if self.reaction_id != 0 {
            struct_ser.serialize_field("reactionId", &self.reaction_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryReactionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subspace_id",
            "subspaceId",
            "post_id",
            "postId",
            "reaction_id",
            "reactionId",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspaceId,
            PostId,
            ReactionId,
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
                            "postId" | "post_id" => Ok(GeneratedField::PostId),
                            "reactionId" | "reaction_id" => Ok(GeneratedField::ReactionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryReactionRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reactions.v1.QueryReactionRequest")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryReactionRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
                let mut post_id__ = None;
                let mut reaction_id__ = None;
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
                        GeneratedField::PostId => {
                            if post_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("postId"));
                            }
                            post_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ReactionId => {
                            if reaction_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reactionId"));
                            }
                            reaction_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryReactionRequest {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    post_id: post_id__.unwrap_or_default(),
                    reaction_id: reaction_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reactions.v1.QueryReactionRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryReactionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reaction.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reactions.v1.QueryReactionResponse", len)?;
        if let Some(v) = self.reaction.as_ref() {
            struct_ser.serialize_field("reaction", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryReactionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["reaction"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reaction,
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
                            "reaction" => Ok(GeneratedField::Reaction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryReactionResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reactions.v1.QueryReactionResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryReactionResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut reaction__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Reaction => {
                            if reaction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reaction"));
                            }
                            reaction__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryReactionResponse {
                    reaction: reaction__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reactions.v1.QueryReactionResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryReactionsParamsRequest {
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
        let mut struct_ser =
            serializer.serialize_struct("desmos.reactions.v1.QueryReactionsParamsRequest", len)?;
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryReactionsParamsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["subspace_id", "subspaceId"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = QueryReactionsParamsRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reactions.v1.QueryReactionsParamsRequest")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryReactionsParamsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
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
                    }
                }
                Ok(QueryReactionsParamsRequest {
                    subspace_id: subspace_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reactions.v1.QueryReactionsParamsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryReactionsParamsResponse {
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
            serializer.serialize_struct("desmos.reactions.v1.QueryReactionsParamsResponse", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryReactionsParamsResponse {
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
            type Value = QueryReactionsParamsResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reactions.v1.QueryReactionsParamsResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryReactionsParamsResponse, V::Error>
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
                Ok(QueryReactionsParamsResponse { params: params__ })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reactions.v1.QueryReactionsParamsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryReactionsRequest {
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
        if self.post_id != 0 {
            len += 1;
        }
        if !self.user.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reactions.v1.QueryReactionsRequest", len)?;
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        if self.post_id != 0 {
            struct_ser.serialize_field("postId", ToString::to_string(&self.post_id).as_str())?;
        }
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryReactionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subspace_id",
            "subspaceId",
            "post_id",
            "postId",
            "user",
            "pagination",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspaceId,
            PostId,
            User,
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
                            "postId" | "post_id" => Ok(GeneratedField::PostId),
                            "user" => Ok(GeneratedField::User),
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
            type Value = QueryReactionsRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reactions.v1.QueryReactionsRequest")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryReactionsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
                let mut post_id__ = None;
                let mut user__ = None;
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
                        GeneratedField::PostId => {
                            if post_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("postId"));
                            }
                            post_id__ = Some(
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
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryReactionsRequest {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    post_id: post_id__.unwrap_or_default(),
                    user: user__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reactions.v1.QueryReactionsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryReactionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.reactions.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reactions.v1.QueryReactionsResponse", len)?;
        if !self.reactions.is_empty() {
            struct_ser.serialize_field("reactions", &self.reactions)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryReactionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["reactions", "pagination"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reactions,
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
                            "reactions" => Ok(GeneratedField::Reactions),
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
            type Value = QueryReactionsResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reactions.v1.QueryReactionsResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryReactionsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut reactions__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Reactions => {
                            if reactions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reactions"));
                            }
                            reactions__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryReactionsResponse {
                    reactions: reactions__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reactions.v1.QueryReactionsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryRegisteredReactionRequest {
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
        if self.reaction_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("desmos.reactions.v1.QueryRegisteredReactionRequest", len)?;
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        if self.reaction_id != 0 {
            struct_ser.serialize_field("reactionId", &self.reaction_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRegisteredReactionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["subspace_id", "subspaceId", "reaction_id", "reactionId"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspaceId,
            ReactionId,
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
                            "reactionId" | "reaction_id" => Ok(GeneratedField::ReactionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryRegisteredReactionRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reactions.v1.QueryRegisteredReactionRequest")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryRegisteredReactionRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
                let mut reaction_id__ = None;
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
                        GeneratedField::ReactionId => {
                            if reaction_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reactionId"));
                            }
                            reaction_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryRegisteredReactionRequest {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    reaction_id: reaction_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reactions.v1.QueryRegisteredReactionRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryRegisteredReactionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.registered_reaction.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("desmos.reactions.v1.QueryRegisteredReactionResponse", len)?;
        if let Some(v) = self.registered_reaction.as_ref() {
            struct_ser.serialize_field("registeredReaction", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRegisteredReactionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["registered_reaction", "registeredReaction"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RegisteredReaction,
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
                            "registeredReaction" | "registered_reaction" => {
                                Ok(GeneratedField::RegisteredReaction)
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
            type Value = QueryRegisteredReactionResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reactions.v1.QueryRegisteredReactionResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryRegisteredReactionResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut registered_reaction__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RegisteredReaction => {
                            if registered_reaction__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "registeredReaction",
                                ));
                            }
                            registered_reaction__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryRegisteredReactionResponse {
                    registered_reaction: registered_reaction__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reactions.v1.QueryRegisteredReactionResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryRegisteredReactionsRequest {
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
        let mut struct_ser = serializer
            .serialize_struct("desmos.reactions.v1.QueryRegisteredReactionsRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for QueryRegisteredReactionsRequest {
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
            type Value = QueryRegisteredReactionsRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reactions.v1.QueryRegisteredReactionsRequest")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryRegisteredReactionsRequest, V::Error>
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
                Ok(QueryRegisteredReactionsRequest {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reactions.v1.QueryRegisteredReactionsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryRegisteredReactionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.registered_reactions.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("desmos.reactions.v1.QueryRegisteredReactionsResponse", len)?;
        if !self.registered_reactions.is_empty() {
            struct_ser.serialize_field("registeredReactions", &self.registered_reactions)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRegisteredReactionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["registered_reactions", "registeredReactions", "pagination"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RegisteredReactions,
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
                            "registeredReactions" | "registered_reactions" => {
                                Ok(GeneratedField::RegisteredReactions)
                            }
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
            type Value = QueryRegisteredReactionsResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reactions.v1.QueryRegisteredReactionsResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryRegisteredReactionsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut registered_reactions__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RegisteredReactions => {
                            if registered_reactions__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "registeredReactions",
                                ));
                            }
                            registered_reactions__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryRegisteredReactionsResponse {
                    registered_reactions: registered_reactions__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reactions.v1.QueryRegisteredReactionsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Reaction {
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
        if self.post_id != 0 {
            len += 1;
        }
        if self.id != 0 {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        if !self.author.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.reactions.v1.Reaction", len)?;
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        if self.post_id != 0 {
            struct_ser.serialize_field("postId", ToString::to_string(&self.post_id).as_str())?;
        }
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        if !self.author.is_empty() {
            struct_ser.serialize_field("author", &self.author)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Reaction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subspace_id",
            "subspaceId",
            "post_id",
            "postId",
            "id",
            "value",
            "author",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspaceId,
            PostId,
            Id,
            Value,
            Author,
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
                            "postId" | "post_id" => Ok(GeneratedField::PostId),
                            "id" => Ok(GeneratedField::Id),
                            "value" => Ok(GeneratedField::Value),
                            "author" => Ok(GeneratedField::Author),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Reaction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reactions.v1.Reaction")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Reaction, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
                let mut post_id__ = None;
                let mut id__ = None;
                let mut value__ = None;
                let mut author__ = None;
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
                        GeneratedField::PostId => {
                            if post_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("postId"));
                            }
                            post_id__ = Some(
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
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map.next_value()?;
                        }
                        GeneratedField::Author => {
                            if author__.is_some() {
                                return Err(serde::de::Error::duplicate_field("author"));
                            }
                            author__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Reaction {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    post_id: post_id__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    value: value__,
                    author: author__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("desmos.reactions.v1.Reaction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RegisteredReaction {
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
        if !self.shorthand_code.is_empty() {
            len += 1;
        }
        if !self.display_value.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reactions.v1.RegisteredReaction", len)?;
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.shorthand_code.is_empty() {
            struct_ser.serialize_field("shorthandCode", &self.shorthand_code)?;
        }
        if !self.display_value.is_empty() {
            struct_ser.serialize_field("displayValue", &self.display_value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegisteredReaction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subspace_id",
            "subspaceId",
            "id",
            "shorthand_code",
            "shorthandCode",
            "display_value",
            "displayValue",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspaceId,
            Id,
            ShorthandCode,
            DisplayValue,
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
                            "shorthandCode" | "shorthand_code" => Ok(GeneratedField::ShorthandCode),
                            "displayValue" | "display_value" => Ok(GeneratedField::DisplayValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegisteredReaction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reactions.v1.RegisteredReaction")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<RegisteredReaction, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
                let mut id__ = None;
                let mut shorthand_code__ = None;
                let mut display_value__ = None;
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
                        GeneratedField::ShorthandCode => {
                            if shorthand_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shorthandCode"));
                            }
                            shorthand_code__ = Some(map.next_value()?);
                        }
                        GeneratedField::DisplayValue => {
                            if display_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayValue"));
                            }
                            display_value__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RegisteredReaction {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    shorthand_code: shorthand_code__.unwrap_or_default(),
                    display_value: display_value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reactions.v1.RegisteredReaction",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for RegisteredReactionValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.registered_reaction_id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reactions.v1.RegisteredReactionValue", len)?;
        if self.registered_reaction_id != 0 {
            struct_ser.serialize_field("registeredReactionId", &self.registered_reaction_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegisteredReactionValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["registered_reaction_id", "registeredReactionId"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RegisteredReactionId,
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
                            "registeredReactionId" | "registered_reaction_id" => {
                                Ok(GeneratedField::RegisteredReactionId)
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
            type Value = RegisteredReactionValue;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reactions.v1.RegisteredReactionValue")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<RegisteredReactionValue, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut registered_reaction_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RegisteredReactionId => {
                            if registered_reaction_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "registeredReactionId",
                                ));
                            }
                            registered_reaction_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(RegisteredReactionValue {
                    registered_reaction_id: registered_reaction_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reactions.v1.RegisteredReactionValue",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for RegisteredReactionValueParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.enabled {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("desmos.reactions.v1.RegisteredReactionValueParams", len)?;
        if self.enabled {
            struct_ser.serialize_field("enabled", &self.enabled)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegisteredReactionValueParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["enabled"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Enabled,
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
                            "enabled" => Ok(GeneratedField::Enabled),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegisteredReactionValueParams;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reactions.v1.RegisteredReactionValueParams")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<RegisteredReactionValueParams, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut enabled__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RegisteredReactionValueParams {
                    enabled: enabled__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reactions.v1.RegisteredReactionValueParams",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for SubspaceReactionsParams {
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
        if self.registered_reaction.is_some() {
            len += 1;
        }
        if self.free_text.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.reactions.v1.SubspaceReactionsParams", len)?;
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        if let Some(v) = self.registered_reaction.as_ref() {
            struct_ser.serialize_field("registeredReaction", v)?;
        }
        if let Some(v) = self.free_text.as_ref() {
            struct_ser.serialize_field("freeText", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubspaceReactionsParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subspace_id",
            "subspaceId",
            "registered_reaction",
            "registeredReaction",
            "free_text",
            "freeText",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspaceId,
            RegisteredReaction,
            FreeText,
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
                            "registeredReaction" | "registered_reaction" => {
                                Ok(GeneratedField::RegisteredReaction)
                            }
                            "freeText" | "free_text" => Ok(GeneratedField::FreeText),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubspaceReactionsParams;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.reactions.v1.SubspaceReactionsParams")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<SubspaceReactionsParams, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
                let mut registered_reaction__ = None;
                let mut free_text__ = None;
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
                        GeneratedField::RegisteredReaction => {
                            if registered_reaction__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "registeredReaction",
                                ));
                            }
                            registered_reaction__ = map.next_value()?;
                        }
                        GeneratedField::FreeText => {
                            if free_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("freeText"));
                            }
                            free_text__ = map.next_value()?;
                        }
                    }
                }
                Ok(SubspaceReactionsParams {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    registered_reaction: registered_reaction__,
                    free_text: free_text__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.reactions.v1.SubspaceReactionsParams",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
