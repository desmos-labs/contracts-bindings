/// GenericSubspaceAuthorization defines an authorization to perform any
/// operation only inside a specific subspace.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.subspaces.v2.authz.GenericSubspaceAuthorization")]
pub struct GenericSubspaceAuthorization {
    /// Ids of the subspaces inside which to grant the permission
    #[prost(uint64, repeated, packed = "false", tag = "1")]
    pub subspaces_ids: ::prost::alloc::vec::Vec<u64>,
    /// Msg, identified by it's type URL, to grant unrestricted permissions to
    /// execute within the subspace
    #[prost(string, tag = "2")]
    pub msg: ::prost::alloc::string::String,
}
impl serde::Serialize for GenericSubspaceAuthorization {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.subspaces_ids.is_empty() {
            len += 1;
        }
        if !self.msg.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "desmos.subspaces.v2.authz.GenericSubspaceAuthorization",
            len,
        )?;
        if !self.subspaces_ids.is_empty() {
            struct_ser.serialize_field(
                "subspacesIds",
                &self
                    .subspaces_ids
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>(),
            )?;
        }
        if !self.msg.is_empty() {
            struct_ser.serialize_field("msg", &self.msg)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenericSubspaceAuthorization {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["subspaces_ids", "subspacesIds", "msg"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspacesIds,
            Msg,
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
                            "subspacesIds" | "subspaces_ids" => Ok(GeneratedField::SubspacesIds),
                            "msg" => Ok(GeneratedField::Msg),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenericSubspaceAuthorization;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.subspaces.v2.authz.GenericSubspaceAuthorization")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<GenericSubspaceAuthorization, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspaces_ids__ = None;
                let mut msg__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SubspacesIds => {
                            if subspaces_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subspacesIds"));
                            }
                            subspaces_ids__ = Some(
                                map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter()
                                    .map(|x| x.0)
                                    .collect(),
                            );
                        }
                        GeneratedField::Msg => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msg"));
                            }
                            msg__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GenericSubspaceAuthorization {
                    subspaces_ids: subspaces_ids__.unwrap_or_default(),
                    msg: msg__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.subspaces.v2.authz.GenericSubspaceAuthorization",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
