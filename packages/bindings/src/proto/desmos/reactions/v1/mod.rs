pub mod client;
/// Reaction contains the data of a single post reaction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.Reaction")]
#[serde(rename_all = "snake_case")]
pub struct Reaction {
    /// Id of the subspace inside which the reaction has been put
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the post to which the reaction is associated
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
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
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.RegisteredReactionValue")]
#[serde(rename_all = "snake_case")]
pub struct RegisteredReactionValue {
    /// Id of the registered reaction
    #[prost(uint32, tag = "1")]
    pub registered_reaction_id: u32,
}
/// FreeTextValue contains the details of a reaction value that
/// is made of free text
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.FreeTextValue")]
#[serde(rename_all = "snake_case")]
pub struct FreeTextValue {
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
}
/// RegisteredReaction contains the details of a registered reaction within a
/// subspace
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.RegisteredReaction")]
#[serde(rename_all = "snake_case")]
pub struct RegisteredReaction {
    /// Id of the subspace for which this reaction has been registered
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
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
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.SubspaceReactionsParams")]
#[serde(rename_all = "snake_case")]
pub struct SubspaceReactionsParams {
    /// Id of the subspace for which these params are valid
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
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
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.FreeTextValueParams")]
#[serde(rename_all = "snake_case")]
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
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.RegisteredReactionValueParams")]
#[serde(rename_all = "snake_case")]
pub struct RegisteredReactionValueParams {
    /// Whether RegisteredReactionValue reactions should be enabled
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
/// GenesisState contains the data of the genesis state for the reactions module
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.GenesisState")]
#[serde(rename_all = "snake_case")]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub subspaces_data: ::prost::alloc::vec::Vec<SubspaceDataEntry>,
    #[prost(message, repeated, tag = "2")]
    pub registered_reactions: ::prost::alloc::vec::Vec<RegisteredReaction>,
    #[prost(message, repeated, tag = "3")]
    pub posts_data: ::prost::alloc::vec::Vec<PostDataEntry>,
    #[prost(message, repeated, tag = "4")]
    pub reactions: ::prost::alloc::vec::Vec<Reaction>,
    #[prost(message, repeated, tag = "5")]
    pub subspaces_params: ::prost::alloc::vec::Vec<SubspaceReactionsParams>,
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
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.SubspaceDataEntry")]
#[serde(rename_all = "snake_case")]
pub struct SubspaceDataEntry {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    #[prost(uint32, tag = "2")]
    pub registered_reaction_id: u32,
}
/// PostDataEntry contains the data related to a single post
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.PostDataEntry")]
#[serde(rename_all = "snake_case")]
pub struct PostDataEntry {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub post_id: u64,
    #[prost(uint32, tag = "3")]
    pub reaction_id: u32,
}
/// MsgAddReaction represents the message to be used to add a post reaction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.MsgAddReaction")]
#[serde(rename_all = "snake_case")]
pub struct MsgAddReaction {
    /// Id of the subspace inside which the post to react to is
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the post to react to
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
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
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.MsgAddReactionResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgAddReactionResponse {
    /// Id of the newly added reaction
    #[prost(uint32, tag = "1")]
    pub reaction_id: u32,
}
/// MsgRemoveReaction represents the message to be used to remove an
/// existing reaction from a post
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.MsgRemoveReaction")]
#[serde(rename_all = "snake_case")]
pub struct MsgRemoveReaction {
    /// Id of the subspace inside which the reaction to remove is
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the post from which to remove the reaction
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
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
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.MsgRemoveReactionResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgRemoveReactionResponse {}
/// MsgAddRegisteredReaction represents the message to be used to
/// register a new supported reaction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.MsgAddRegisteredReaction")]
#[serde(rename_all = "snake_case")]
pub struct MsgAddRegisteredReaction {
    /// Id of the subspace inside which this reaction should be registered
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
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
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.MsgAddRegisteredReactionResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgAddRegisteredReactionResponse {
    /// Id of the newly registered reaction
    #[prost(uint32, tag = "1")]
    pub registered_reaction_id: u32,
}
/// MsgEditRegisteredReaction represents the message to be used to edit a
/// registered reaction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.MsgEditRegisteredReaction")]
#[serde(rename_all = "snake_case")]
pub struct MsgEditRegisteredReaction {
    /// Id of the subspace inside which the reaction to edit is
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
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
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.MsgEditRegisteredReactionResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgEditRegisteredReactionResponse {}
/// MsgRemoveRegisteredReaction represents the message to be used to
/// remove an existing registered reaction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.MsgRemoveRegisteredReaction")]
#[serde(rename_all = "snake_case")]
pub struct MsgRemoveRegisteredReaction {
    /// Id of the subspace from which to remove the registered reaction
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
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
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.MsgRemoveRegisteredReactionResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgRemoveRegisteredReactionResponse {}
/// MsgSetReactionsParams represents the message to be used when setting
/// a subspace reactions params
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.MsgSetReactionsParams")]
#[serde(rename_all = "snake_case")]
pub struct MsgSetReactionsParams {
    /// Id of the subspace for which to set the params
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
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
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.MsgSetReactionsParamsResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgSetReactionsParamsResponse {}
/// QueryReactionsRequest is the request type for the Query/Reactions RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.QueryReactionsRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/desmos.reactions.v1.Query/Reactions",
    response_type = QueryReactionsResponse
)]
pub struct QueryReactionsRequest {
    /// Id of the subspace that contains the post to query the reactions for
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Post id to query the reactions for
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
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
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.QueryReactionsResponse")]
#[serde(rename_all = "snake_case")]
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
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.QueryReactionRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/desmos.reactions.v1.Query/Reaction",
    response_type = QueryReactionResponse
)]
pub struct QueryReactionRequest {
    /// Id of the subspace that contains the post to query the reactions for
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Post id to query the reactions for
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub post_id: u64,
    /// Id of the reaction to query
    #[prost(uint32, tag = "3")]
    pub reaction_id: u32,
}
/// QueryReactionResponse is the response type for the Query/Reaction RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.QueryReactionResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryReactionResponse {
    #[prost(message, optional, tag = "1")]
    pub reaction: ::core::option::Option<Reaction>,
}
/// QueryRegisteredReactionsRequest is the request type for the
/// Query/RegisteredReactions RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.QueryRegisteredReactionsRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/desmos.reactions.v1.Query/RegisteredReactions",
    response_type = QueryRegisteredReactionsResponse
)]
pub struct QueryRegisteredReactionsRequest {
    /// Id of the subspace to query the registered reactions for
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
/// QueryRegisteredReactionsResponse is the response type for the
/// Query/RegisteredReactions RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.QueryRegisteredReactionsResponse")]
#[serde(rename_all = "snake_case")]
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
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.QueryRegisteredReactionRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/desmos.reactions.v1.Query/RegisteredReaction",
    response_type = QueryRegisteredReactionResponse
)]
pub struct QueryRegisteredReactionRequest {
    /// Id of the subspace to query the registered reactions for
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the registered reaction to query for
    #[prost(uint32, tag = "2")]
    pub reaction_id: u32,
}
/// QueryRegisteredReactionResponse is the response type for the
/// Query/RegisteredReaction RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.QueryRegisteredReactionResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryRegisteredReactionResponse {
    #[prost(message, optional, tag = "1")]
    pub registered_reaction: ::core::option::Option<RegisteredReaction>,
}
/// QueryReactionsParamsRequest is the request type for the Query/ReactionsParams
/// RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.QueryReactionsParamsRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/desmos.reactions.v1.Query/ReactionsParams",
    response_type = QueryReactionsParamsResponse
)]
pub struct QueryReactionsParamsRequest {
    /// Id of the subspace for which to query the params
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
}
/// QueryReactionsParamsResponse is the response type for the
/// Query/ReactionsParam RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.reactions.v1.QueryReactionsParamsResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryReactionsParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<SubspaceReactionsParams>,
}
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
