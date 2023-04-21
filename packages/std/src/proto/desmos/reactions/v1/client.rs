/// SetReactionsParamsJSON contains the data that can be specified when setting a
/// subspace reactions params using the CLI command
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
#[proto_message(type_url = "/desmos.reactions.v1.client.SetReactionsParamsJSON")]
#[serde(rename_all = "snake_case")]
pub struct SetReactionsParamsJson {
    /// Params related to RegisteredReactionValue reactions
    #[prost(message, optional, tag = "1")]
    pub registered_reaction_params: ::core::option::Option<super::RegisteredReactionValueParams>,
    /// Params related to FreeTextValue reactions
    #[prost(message, optional, tag = "2")]
    pub free_text_params: ::core::option::Option<super::FreeTextValueParams>,
}
