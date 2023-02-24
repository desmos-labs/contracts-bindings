/// SignatureDescriptors wraps multiple SignatureDescriptor's.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/cosmos.tx.signing.v1beta1.SignatureDescriptors")]
pub struct SignatureDescriptors {
    /// signatures are the signature descriptors
    #[prost(message, repeated, tag = "1")]
    pub signatures: ::prost::alloc::vec::Vec<SignatureDescriptor>,
}
/// SignatureDescriptor is a convenience type which represents the full data for
/// a signature including the public key of the signer, signing modes and the
/// signature itself. It is primarily used for coordinating signatures between
/// clients.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/cosmos.tx.signing.v1beta1.SignatureDescriptor")]
pub struct SignatureDescriptor {
    /// public_key is the public key of the signer
    #[prost(message, optional, tag = "1")]
    pub public_key: ::core::option::Option<crate::shim::Any>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<signature_descriptor::Data>,
    /// sequence is the sequence of the account, which describes the
    /// number of committed transactions signed by a given address. It is used to prevent
    /// replay attacks.
    #[prost(uint64, tag = "3")]
    pub sequence: u64,
}
/// Nested message and enum types in `SignatureDescriptor`.
pub mod signature_descriptor {
    /// Data represents signature data
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
    #[proto_message(type_url = "/cosmos.tx.signing.v1beta1.SignatureDescriptor.Data")]
    pub struct Data {
        /// sum is the oneof that specifies whether this represents single or multi-signature data
        #[prost(oneof = "data::Sum", tags = "1, 2")]
        pub sum: ::core::option::Option<data::Sum>,
    }
    /// Nested message and enum types in `Data`.
    pub mod data {
        /// Single is the signature data for a single signer
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(
            Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt,
        )]
        #[proto_message(type_url = "/cosmos.tx.signing.v1beta1.SignatureDescriptor.Data.Single")]
        pub struct Single {
            /// mode is the signing mode of the single signer
            #[prost(enumeration = "super::super::SignMode", tag = "1")]
            pub mode: i32,
            /// signature is the raw signature bytes
            #[prost(bytes = "vec", tag = "2")]
            pub signature: ::prost::alloc::vec::Vec<u8>,
        }
        /// Multi is the signature data for a multisig public key
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(
            Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt,
        )]
        #[proto_message(type_url = "/cosmos.tx.signing.v1beta1.SignatureDescriptor.Data.Multi")]
        pub struct Multi {
            /// bitarray specifies which keys within the multisig are signing
            #[prost(message, optional, tag = "1")]
            pub bitarray: ::core::option::Option<
                super::super::super::super::super::crypto::multisig::v1beta1::CompactBitArray,
            >,
            /// signatures is the signatures of the multi-signature
            #[prost(message, repeated, tag = "2")]
            pub signatures: ::prost::alloc::vec::Vec<super::Data>,
        }
        /// sum is the oneof that specifies whether this represents single or multi-signature data
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof, schemars::JsonSchema)]
        pub enum Sum {
            /// single represents a single signer
            #[prost(message, tag = "1")]
            Single(Single),
            /// multi represents a multisig signer
            #[prost(message, tag = "2")]
            Multi(Multi),
        }
    }
}
/// SignMode represents a signing mode with its own security guarantees.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(schemars::JsonSchema)]
pub enum SignMode {
    /// SIGN_MODE_UNSPECIFIED specifies an unknown signing mode and will be
    /// rejected
    Unspecified = 0,
    /// SIGN_MODE_DIRECT specifies a signing mode which uses SignDoc and is
    /// verified with raw bytes from Tx
    Direct = 1,
    /// SIGN_MODE_TEXTUAL is a future signing mode that will verify some
    /// human-readable textual representation on top of the binary representation
    /// from SIGN_MODE_DIRECT
    Textual = 2,
    /// SIGN_MODE_LEGACY_AMINO_JSON is a backwards compatibility mode which uses
    /// Amino JSON and will be removed in the future
    LegacyAminoJson = 127,
    /// SIGN_MODE_EIP_191 specifies the sign mode for EIP 191 signing on the Cosmos
    /// SDK. Ref: <https://eips.ethereum.org/EIPS/eip-191>
    ///
    /// Currently, SIGN_MODE_EIP_191 is registered as a SignMode enum variant,
    /// but is not implemented on the SDK by default. To enable EIP-191, you need
    /// to pass a custom `TxConfig` that has an implementation of
    /// `SignModeHandler` for EIP-191. The SDK may decide to fully support
    /// EIP-191 in the future.
    ///
    /// Since: cosmos-sdk 0.45.2
    Eip191 = 191,
}
impl SignMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SignMode::Unspecified => "SIGN_MODE_UNSPECIFIED",
            SignMode::Direct => "SIGN_MODE_DIRECT",
            SignMode::Textual => "SIGN_MODE_TEXTUAL",
            SignMode::LegacyAminoJson => "SIGN_MODE_LEGACY_AMINO_JSON",
            SignMode::Eip191 => "SIGN_MODE_EIP_191",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SIGN_MODE_UNSPECIFIED" => Some(Self::Unspecified),
            "SIGN_MODE_DIRECT" => Some(Self::Direct),
            "SIGN_MODE_TEXTUAL" => Some(Self::Textual),
            "SIGN_MODE_LEGACY_AMINO_JSON" => Some(Self::LegacyAminoJson),
            "SIGN_MODE_EIP_191" => Some(Self::Eip191),
            _ => None,
        }
    }
}
impl serde::Serialize for SignMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SIGN_MODE_UNSPECIFIED",
            Self::Direct => "SIGN_MODE_DIRECT",
            Self::Textual => "SIGN_MODE_TEXTUAL",
            Self::LegacyAminoJson => "SIGN_MODE_LEGACY_AMINO_JSON",
            Self::Eip191 => "SIGN_MODE_EIP_191",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SignMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SIGN_MODE_UNSPECIFIED",
            "SIGN_MODE_DIRECT",
            "SIGN_MODE_TEXTUAL",
            "SIGN_MODE_LEGACY_AMINO_JSON",
            "SIGN_MODE_EIP_191",
        ];
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SignMode;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }
            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(SignMode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }
            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(SignMode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }
            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SIGN_MODE_UNSPECIFIED" => Ok(SignMode::Unspecified),
                    "SIGN_MODE_DIRECT" => Ok(SignMode::Direct),
                    "SIGN_MODE_TEXTUAL" => Ok(SignMode::Textual),
                    "SIGN_MODE_LEGACY_AMINO_JSON" => Ok(SignMode::LegacyAminoJson),
                    "SIGN_MODE_EIP_191" => Ok(SignMode::Eip191),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SignatureDescriptor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.public_key.is_some() {
            len += 1;
        }
        if self.data.is_some() {
            len += 1;
        }
        if self.sequence != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmos.tx.signing.v1beta1.SignatureDescriptor", len)?;
        if let Some(v) = self.public_key.as_ref() {
            struct_ser.serialize_field("publicKey", v)?;
        }
        if let Some(v) = self.data.as_ref() {
            struct_ser.serialize_field("data", v)?;
        }
        if self.sequence != 0 {
            struct_ser.serialize_field("sequence", ToString::to_string(&self.sequence).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SignatureDescriptor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["public_key", "publicKey", "data", "sequence"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PublicKey,
            Data,
            Sequence,
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
                            "publicKey" | "public_key" => Ok(GeneratedField::PublicKey),
                            "data" => Ok(GeneratedField::Data),
                            "sequence" => Ok(GeneratedField::Sequence),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SignatureDescriptor;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmos.tx.signing.v1beta1.SignatureDescriptor")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<SignatureDescriptor, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut public_key__ = None;
                let mut data__ = None;
                let mut sequence__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PublicKey => {
                            if public_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKey"));
                            }
                            public_key__ = map.next_value()?;
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = map.next_value()?;
                        }
                        GeneratedField::Sequence => {
                            if sequence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequence"));
                            }
                            sequence__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(SignatureDescriptor {
                    public_key: public_key__,
                    data: data__,
                    sequence: sequence__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.tx.signing.v1beta1.SignatureDescriptor",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for signature_descriptor::Data {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.sum.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.tx.signing.v1beta1.SignatureDescriptor.Data", len)?;
        if let Some(v) = self.sum.as_ref() {
            match v {
                signature_descriptor::data::Sum::Single(v) => {
                    struct_ser.serialize_field("single", v)?;
                }
                signature_descriptor::data::Sum::Multi(v) => {
                    struct_ser.serialize_field("multi", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for signature_descriptor::Data {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["single", "multi"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Single,
            Multi,
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
                            "single" => Ok(GeneratedField::Single),
                            "multi" => Ok(GeneratedField::Multi),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = signature_descriptor::Data;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmos.tx.signing.v1beta1.SignatureDescriptor.Data")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<signature_descriptor::Data, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sum__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Single => {
                            if sum__.is_some() {
                                return Err(serde::de::Error::duplicate_field("single"));
                            }
                            sum__ = map
                                .next_value::<::std::option::Option<_>>()?
                                .map(signature_descriptor::data::Sum::Single);
                        }
                        GeneratedField::Multi => {
                            if sum__.is_some() {
                                return Err(serde::de::Error::duplicate_field("multi"));
                            }
                            sum__ = map
                                .next_value::<::std::option::Option<_>>()?
                                .map(signature_descriptor::data::Sum::Multi);
                        }
                    }
                }
                Ok(signature_descriptor::Data { sum: sum__ })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.tx.signing.v1beta1.SignatureDescriptor.Data",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for signature_descriptor::data::Multi {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.bitarray.is_some() {
            len += 1;
        }
        if !self.signatures.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "cosmos.tx.signing.v1beta1.SignatureDescriptor.Data.Multi",
            len,
        )?;
        if let Some(v) = self.bitarray.as_ref() {
            struct_ser.serialize_field("bitarray", v)?;
        }
        if !self.signatures.is_empty() {
            struct_ser.serialize_field("signatures", &self.signatures)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for signature_descriptor::data::Multi {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["bitarray", "signatures"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Bitarray,
            Signatures,
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
                            "bitarray" => Ok(GeneratedField::Bitarray),
                            "signatures" => Ok(GeneratedField::Signatures),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = signature_descriptor::data::Multi;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct cosmos.tx.signing.v1beta1.SignatureDescriptor.Data.Multi")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<signature_descriptor::data::Multi, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut bitarray__ = None;
                let mut signatures__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Bitarray => {
                            if bitarray__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitarray"));
                            }
                            bitarray__ = map.next_value()?;
                        }
                        GeneratedField::Signatures => {
                            if signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signatures"));
                            }
                            signatures__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(signature_descriptor::data::Multi {
                    bitarray: bitarray__,
                    signatures: signatures__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.tx.signing.v1beta1.SignatureDescriptor.Data.Multi",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for signature_descriptor::data::Single {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.mode != 0 {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "cosmos.tx.signing.v1beta1.SignatureDescriptor.Data.Single",
            len,
        )?;
        if self.mode != 0 {
            let v = SignMode::from_i32(self.mode).ok_or_else(|| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.mode))
            })?;
            struct_ser.serialize_field("mode", &v)?;
        }
        if !self.signature.is_empty() {
            struct_ser.serialize_field(
                "signature",
                pbjson::private::base64::encode(&self.signature).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for signature_descriptor::data::Single {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["mode", "signature"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Mode,
            Signature,
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
                            "mode" => Ok(GeneratedField::Mode),
                            "signature" => Ok(GeneratedField::Signature),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = signature_descriptor::data::Single;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct cosmos.tx.signing.v1beta1.SignatureDescriptor.Data.Single")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<signature_descriptor::data::Single, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut mode__ = None;
                let mut signature__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Mode => {
                            if mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            mode__ = Some(map.next_value::<SignMode>()? as i32);
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = Some(
                                map.next_value::<::pbjson::private::BytesDeserialize<_>>()
                                    .unwrap_or(pbjson::private::BytesDeserialize(vec![]))
                                    .0,
                            );
                        }
                    }
                }
                Ok(signature_descriptor::data::Single {
                    mode: mode__.unwrap_or_default(),
                    signature: signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.tx.signing.v1beta1.SignatureDescriptor.Data.Single",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for SignatureDescriptors {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.signatures.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmos.tx.signing.v1beta1.SignatureDescriptors", len)?;
        if !self.signatures.is_empty() {
            struct_ser.serialize_field("signatures", &self.signatures)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SignatureDescriptors {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signatures"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signatures,
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
                            "signatures" => Ok(GeneratedField::Signatures),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SignatureDescriptors;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmos.tx.signing.v1beta1.SignatureDescriptors")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<SignatureDescriptors, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut signatures__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Signatures => {
                            if signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signatures"));
                            }
                            signatures__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(SignatureDescriptors {
                    signatures: signatures__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.tx.signing.v1beta1.SignatureDescriptors",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
