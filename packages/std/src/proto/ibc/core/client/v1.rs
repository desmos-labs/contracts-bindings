/// IdentifiedClientState defines a client state with an additional client
/// identifier field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.client.v1.IdentifiedClientState")]
pub struct IdentifiedClientState {
    /// client identifier
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// client state
    #[prost(message, optional, tag = "2")]
    pub client_state: ::core::option::Option<crate::shim::Any>,
}
/// ConsensusStateWithHeight defines a consensus state with an additional height
/// field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.client.v1.ConsensusStateWithHeight")]
pub struct ConsensusStateWithHeight {
    /// consensus state height
    #[prost(message, optional, tag = "1")]
    pub height: ::core::option::Option<Height>,
    /// consensus state
    #[prost(message, optional, tag = "2")]
    pub consensus_state: ::core::option::Option<crate::shim::Any>,
}
/// ClientConsensusStates defines all the stored consensus states for a given
/// client.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.client.v1.ClientConsensusStates")]
pub struct ClientConsensusStates {
    /// client identifier
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// consensus states and their heights associated with the client
    #[prost(message, repeated, tag = "2")]
    pub consensus_states: ::prost::alloc::vec::Vec<ConsensusStateWithHeight>,
}
/// ClientUpdateProposal is a governance proposal. If it passes, the substitute
/// client's latest consensus state is copied over to the subject client. The proposal
/// handler may fail if the subject and the substitute do not match in client and
/// chain parameters (with exception to latest height, frozen height, and chain-id).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.client.v1.ClientUpdateProposal")]
pub struct ClientUpdateProposal {
    /// the title of the update proposal
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// the description of the proposal
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// the client identifier for the client to be updated if the proposal passes
    #[prost(string, tag = "3")]
    pub subject_client_id: ::prost::alloc::string::String,
    /// the substitute client identifier for the client standing in for the subject
    /// client
    #[prost(string, tag = "4")]
    pub substitute_client_id: ::prost::alloc::string::String,
}
/// UpgradeProposal is a gov Content type for initiating an IBC breaking
/// upgrade.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.client.v1.UpgradeProposal")]
pub struct UpgradeProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub plan: ::core::option::Option<super::super::super::super::cosmos::upgrade::v1beta1::Plan>,
    /// An UpgradedClientState must be provided to perform an IBC breaking upgrade.
    /// This will make the chain commit to the correct upgraded (self) client state
    /// before the upgrade occurs, so that connecting chains can verify that the
    /// new upgraded client is valid by verifying a proof on the previous version
    /// of the chain. This will allow IBC connections to persist smoothly across
    /// planned chain upgrades
    #[prost(message, optional, tag = "4")]
    pub upgraded_client_state: ::core::option::Option<crate::shim::Any>,
}
/// Height is a monotonically increasing data type
/// that can be compared against another Height for the purposes of updating and
/// freezing clients
///
/// Normally the RevisionHeight is incremented at each height while keeping
/// RevisionNumber the same. However some consensus algorithms may choose to
/// reset the height in certain conditions e.g. hard forks, state-machine
/// breaking changes In these cases, the RevisionNumber is incremented so that
/// height continues to be monitonically increasing even as the RevisionHeight
/// gets reset
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.client.v1.Height")]
pub struct Height {
    /// the revision that the client is currently on
    #[prost(uint64, tag = "1")]
    pub revision_number: u64,
    /// the height within the given revision
    #[prost(uint64, tag = "2")]
    pub revision_height: u64,
}
/// Params defines the set of IBC light client parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.client.v1.Params")]
pub struct Params {
    /// allowed_clients defines the list of allowed client state types.
    #[prost(string, repeated, tag = "1")]
    pub allowed_clients: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl serde::Serialize for ClientConsensusStates {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.client_id.is_empty() {
            len += 1;
        }
        if !self.consensus_states.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("ibc.core.client.v1.ClientConsensusStates", len)?;
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if !self.consensus_states.is_empty() {
            struct_ser.serialize_field("consensusStates", &self.consensus_states)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClientConsensusStates {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_id",
            "clientId",
            "consensus_states",
            "consensusStates",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientId,
            ConsensusStates,
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
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            "consensusStates" | "consensus_states" => {
                                Ok(GeneratedField::ConsensusStates)
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
            type Value = ClientConsensusStates;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ibc.core.client.v1.ClientConsensusStates")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<ClientConsensusStates, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut client_id__ = None;
                let mut consensus_states__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::ConsensusStates => {
                            if consensus_states__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consensusStates"));
                            }
                            consensus_states__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ClientConsensusStates {
                    client_id: client_id__.unwrap_or_default(),
                    consensus_states: consensus_states__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "ibc.core.client.v1.ClientConsensusStates",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ClientUpdateProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.subject_client_id.is_empty() {
            len += 1;
        }
        if !self.substitute_client_id.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("ibc.core.client.v1.ClientUpdateProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.subject_client_id.is_empty() {
            struct_ser.serialize_field("subjectClientId", &self.subject_client_id)?;
        }
        if !self.substitute_client_id.is_empty() {
            struct_ser.serialize_field("substituteClientId", &self.substitute_client_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClientUpdateProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "subject_client_id",
            "subjectClientId",
            "substitute_client_id",
            "substituteClientId",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            SubjectClientId,
            SubstituteClientId,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "subjectClientId" | "subject_client_id" => {
                                Ok(GeneratedField::SubjectClientId)
                            }
                            "substituteClientId" | "substitute_client_id" => {
                                Ok(GeneratedField::SubstituteClientId)
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
            type Value = ClientUpdateProposal;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ibc.core.client.v1.ClientUpdateProposal")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClientUpdateProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut subject_client_id__ = None;
                let mut substitute_client_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
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
                        GeneratedField::SubjectClientId => {
                            if subject_client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectClientId"));
                            }
                            subject_client_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::SubstituteClientId => {
                            if substitute_client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "substituteClientId",
                                ));
                            }
                            substitute_client_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ClientUpdateProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    subject_client_id: subject_client_id__.unwrap_or_default(),
                    substitute_client_id: substitute_client_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "ibc.core.client.v1.ClientUpdateProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ConsensusStateWithHeight {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.height.is_some() {
            len += 1;
        }
        if self.consensus_state.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("ibc.core.client.v1.ConsensusStateWithHeight", len)?;
        if let Some(v) = self.height.as_ref() {
            struct_ser.serialize_field("height", v)?;
        }
        if let Some(v) = self.consensus_state.as_ref() {
            struct_ser.serialize_field("consensusState", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConsensusStateWithHeight {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["height", "consensus_state", "consensusState"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Height,
            ConsensusState,
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
                            "height" => Ok(GeneratedField::Height),
                            "consensusState" | "consensus_state" => {
                                Ok(GeneratedField::ConsensusState)
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
            type Value = ConsensusStateWithHeight;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ibc.core.client.v1.ConsensusStateWithHeight")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<ConsensusStateWithHeight, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut height__ = None;
                let mut consensus_state__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = map.next_value()?;
                        }
                        GeneratedField::ConsensusState => {
                            if consensus_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consensusState"));
                            }
                            consensus_state__ = map.next_value()?;
                        }
                    }
                }
                Ok(ConsensusStateWithHeight {
                    height: height__,
                    consensus_state: consensus_state__,
                })
            }
        }
        deserializer.deserialize_struct(
            "ibc.core.client.v1.ConsensusStateWithHeight",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Height {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.revision_number != 0 {
            len += 1;
        }
        if self.revision_height != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ibc.core.client.v1.Height", len)?;
        if self.revision_number != 0 {
            struct_ser.serialize_field(
                "revisionNumber",
                ToString::to_string(&self.revision_number).as_str(),
            )?;
        }
        if self.revision_height != 0 {
            struct_ser.serialize_field(
                "revisionHeight",
                ToString::to_string(&self.revision_height).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Height {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "revision_number",
            "revisionNumber",
            "revision_height",
            "revisionHeight",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RevisionNumber,
            RevisionHeight,
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
                            "revisionNumber" | "revision_number" => {
                                Ok(GeneratedField::RevisionNumber)
                            }
                            "revisionHeight" | "revision_height" => {
                                Ok(GeneratedField::RevisionHeight)
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
            type Value = Height;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ibc.core.client.v1.Height")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Height, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut revision_number__ = None;
                let mut revision_height__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RevisionNumber => {
                            if revision_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("revisionNumber"));
                            }
                            revision_number__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::RevisionHeight => {
                            if revision_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("revisionHeight"));
                            }
                            revision_height__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(Height {
                    revision_number: revision_number__.unwrap_or_default(),
                    revision_height: revision_height__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.core.client.v1.Height", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IdentifiedClientState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.client_id.is_empty() {
            len += 1;
        }
        if self.client_state.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("ibc.core.client.v1.IdentifiedClientState", len)?;
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if let Some(v) = self.client_state.as_ref() {
            struct_ser.serialize_field("clientState", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IdentifiedClientState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["client_id", "clientId", "client_state", "clientState"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientId,
            ClientState,
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
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            "clientState" | "client_state" => Ok(GeneratedField::ClientState),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IdentifiedClientState;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ibc.core.client.v1.IdentifiedClientState")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<IdentifiedClientState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut client_id__ = None;
                let mut client_state__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::ClientState => {
                            if client_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientState"));
                            }
                            client_state__ = map.next_value()?;
                        }
                    }
                }
                Ok(IdentifiedClientState {
                    client_id: client_id__.unwrap_or_default(),
                    client_state: client_state__,
                })
            }
        }
        deserializer.deserialize_struct(
            "ibc.core.client.v1.IdentifiedClientState",
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
        if !self.allowed_clients.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ibc.core.client.v1.Params", len)?;
        if !self.allowed_clients.is_empty() {
            struct_ser.serialize_field("allowedClients", &self.allowed_clients)?;
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
        const FIELDS: &[&str] = &["allowed_clients", "allowedClients"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllowedClients,
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
                            "allowedClients" | "allowed_clients" => {
                                Ok(GeneratedField::AllowedClients)
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
                formatter.write_str("struct ibc.core.client.v1.Params")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Params, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut allowed_clients__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AllowedClients => {
                            if allowed_clients__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedClients"));
                            }
                            allowed_clients__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Params {
                    allowed_clients: allowed_clients__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ibc.core.client.v1.Params", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpgradeProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.plan.is_some() {
            len += 1;
        }
        if self.upgraded_client_state.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("ibc.core.client.v1.UpgradeProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.plan.as_ref() {
            struct_ser.serialize_field("plan", v)?;
        }
        if let Some(v) = self.upgraded_client_state.as_ref() {
            struct_ser.serialize_field("upgradedClientState", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpgradeProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "plan",
            "upgraded_client_state",
            "upgradedClientState",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            Plan,
            UpgradedClientState,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "plan" => Ok(GeneratedField::Plan),
                            "upgradedClientState" | "upgraded_client_state" => {
                                Ok(GeneratedField::UpgradedClientState)
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
            type Value = UpgradeProposal;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ibc.core.client.v1.UpgradeProposal")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpgradeProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut plan__ = None;
                let mut upgraded_client_state__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
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
                        GeneratedField::Plan => {
                            if plan__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plan"));
                            }
                            plan__ = map.next_value()?;
                        }
                        GeneratedField::UpgradedClientState => {
                            if upgraded_client_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "upgradedClientState",
                                ));
                            }
                            upgraded_client_state__ = map.next_value()?;
                        }
                    }
                }
                Ok(UpgradeProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    plan: plan__,
                    upgraded_client_state: upgraded_client_state__,
                })
            }
        }
        deserializer.deserialize_struct(
            "ibc.core.client.v1.UpgradeProposal",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
