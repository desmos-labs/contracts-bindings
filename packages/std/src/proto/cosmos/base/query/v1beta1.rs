/// PageRequest is to be embedded in gRPC request messages for efficient
/// pagination. Ex:
///
///   message SomeRequest {
///           Foo some_parameter = 1;
///           PageRequest pagination = 2;
///   }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/cosmos.base.query.v1beta1.PageRequest")]
pub struct PageRequest {
    /// key is a value returned in PageResponse.next_key to begin
    /// querying the next page most efficiently. Only one of offset or key
    /// should be set.
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// offset is a numeric offset that can be used when key is unavailable.
    /// It is less efficient than using key. Only one of offset or key should
    /// be set.
    #[prost(uint64, tag = "2")]
    pub offset: u64,
    /// limit is the total number of results to be returned in the result page.
    /// If left empty it will default to a value to be set by each app.
    #[prost(uint64, tag = "3")]
    pub limit: u64,
    /// count_total is set to true  to indicate that the result set should include
    /// a count of the total number of items available for pagination in UIs.
    /// count_total is only respected when offset is used. It is ignored when key
    /// is set.
    #[prost(bool, tag = "4")]
    pub count_total: bool,
    /// reverse is set to true if results are to be returned in the descending order.
    ///
    /// Since: cosmos-sdk 0.43
    #[prost(bool, tag = "5")]
    pub reverse: bool,
}
/// PageResponse is to be embedded in gRPC response messages where the
/// corresponding request message has used PageRequest.
///
///   message SomeResponse {
///           repeated Bar results = 1;
///           PageResponse page = 2;
///   }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/cosmos.base.query.v1beta1.PageResponse")]
pub struct PageResponse {
    /// next_key is the key to be passed to PageRequest.key to
    /// query the next page most efficiently
    #[prost(bytes = "vec", tag = "1")]
    pub next_key: ::prost::alloc::vec::Vec<u8>,
    /// total is total number of results available if PageRequest.count_total
    /// was set, its value is undefined otherwise
    #[prost(uint64, tag = "2")]
    pub total: u64,
}
impl serde::Serialize for PageRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if self.offset != 0 {
            len += 1;
        }
        if self.limit != 0 {
            len += 1;
        }
        if self.count_total {
            len += 1;
        }
        if self.reverse {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmos.base.query.v1beta1.PageRequest", len)?;
        if !self.key.is_empty() {
            struct_ser
                .serialize_field("key", pbjson::private::base64::encode(&self.key).as_str())?;
        }
        if self.offset != 0 {
            struct_ser.serialize_field("offset", ToString::to_string(&self.offset).as_str())?;
        }
        if self.limit != 0 {
            struct_ser.serialize_field("limit", ToString::to_string(&self.limit).as_str())?;
        }
        if self.count_total {
            struct_ser.serialize_field("countTotal", &self.count_total)?;
        }
        if self.reverse {
            struct_ser.serialize_field("reverse", &self.reverse)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PageRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "offset",
            "limit",
            "count_total",
            "countTotal",
            "reverse",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Offset,
            Limit,
            CountTotal,
            Reverse,
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
                            "key" => Ok(GeneratedField::Key),
                            "offset" => Ok(GeneratedField::Offset),
                            "limit" => Ok(GeneratedField::Limit),
                            "countTotal" | "count_total" => Ok(GeneratedField::CountTotal),
                            "reverse" => Ok(GeneratedField::Reverse),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PageRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmos.base.query.v1beta1.PageRequest")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<PageRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut offset__ = None;
                let mut limit__ = None;
                let mut count_total__ = None;
                let mut reverse__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(
                                map.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Offset => {
                            if offset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offset"));
                            }
                            offset__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::CountTotal => {
                            if count_total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("countTotal"));
                            }
                            count_total__ = Some(map.next_value()?);
                        }
                        GeneratedField::Reverse => {
                            if reverse__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reverse"));
                            }
                            reverse__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(PageRequest {
                    key: key__.unwrap_or_default(),
                    offset: offset__.unwrap_or_default(),
                    limit: limit__.unwrap_or_default(),
                    count_total: count_total__.unwrap_or_default(),
                    reverse: reverse__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.base.query.v1beta1.PageRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for PageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.next_key.is_empty() {
            len += 1;
        }
        if self.total != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("cosmos.base.query.v1beta1.PageResponse", len)?;
        if !self.next_key.is_empty() {
            struct_ser.serialize_field(
                "nextKey",
                pbjson::private::base64::encode(&self.next_key).as_str(),
            )?;
        }
        if self.total != 0 {
            struct_ser.serialize_field("total", ToString::to_string(&self.total).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PageResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["next_key", "nextKey", "total"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NextKey,
            Total,
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
                            "nextKey" | "next_key" => Ok(GeneratedField::NextKey),
                            "total" => Ok(GeneratedField::Total),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PageResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cosmos.base.query.v1beta1.PageResponse")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<PageResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut next_key__ = None;
                let mut total__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::NextKey => {
                            if next_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextKey"));
                            }
                            next_key__ = Some(
                                map.next_value::<::pbjson::private::BytesDeserialize<_>>()
                                    // [HACKED] deserialize null or other invalid types as empty
                                    .unwrap_or(pbjson::private::BytesDeserialize(vec![]))
                                    .0,
                            );
                        }
                        GeneratedField::Total => {
                            if total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            total__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(PageResponse {
                    next_key: next_key__.unwrap_or_default(),
                    total: total__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "cosmos.base.query.v1beta1.PageResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}