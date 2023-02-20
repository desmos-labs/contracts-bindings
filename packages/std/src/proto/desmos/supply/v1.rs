/// QueryTotalRequest is the request type for Query/Total RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.supply.v1.QueryTotalRequest")]
#[proto_query(
    path = "/desmos.supply.v1.Query/Total",
    response_type = QueryTotalResponse
)]
pub struct QueryTotalRequest {
    /// coin denom to query the circulating supply for
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// divider_exponent is a factor used to power the divider used to convert the
    /// supply to the desired representation
    #[prost(uint64, tag = "2")]
    pub divider_exponent: u64,
}
/// QueryTotalResponse is the response type for the Query/Total RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.supply.v1.QueryTotalResponse")]
pub struct QueryTotalResponse {
    #[prost(string, tag = "1")]
    pub total_supply: ::prost::alloc::string::String,
}
/// QueryCirculatingRequest is the request type for the Query/Circulating RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.supply.v1.QueryCirculatingRequest")]
#[proto_query(
    path = "/desmos.supply.v1.Query/Circulating",
    response_type = QueryCirculatingResponse
)]
pub struct QueryCirculatingRequest {
    /// coin denom to query the circulating supply for
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// divider_exponent is a factor used to power the divider used to convert the
    /// supply to the desired representation
    #[prost(uint64, tag = "2")]
    pub divider_exponent: u64,
}
/// QueryCirculatingResponse is the response type for the Query/Circulating RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.supply.v1.QueryCirculatingResponse")]
pub struct QueryCirculatingResponse {
    #[prost(string, tag = "1")]
    pub circulating_supply: ::prost::alloc::string::String,
}
pub struct SupplyQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> SupplyQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn total(
        &self,
        denom: ::prost::alloc::string::String,
        divider_exponent: u64,
    ) -> std::result::Result<QueryTotalResponse, cosmwasm_std::StdError> {
        QueryTotalRequest {
            denom,
            divider_exponent,
        }
        .query(self.querier)
    }
    pub fn circulating(
        &self,
        denom: ::prost::alloc::string::String,
        divider_exponent: u64,
    ) -> std::result::Result<QueryCirculatingResponse, cosmwasm_std::StdError> {
        QueryCirculatingRequest {
            denom,
            divider_exponent,
        }
        .query(self.querier)
    }
}
impl serde::Serialize for QueryCirculatingRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.denom.is_empty() {
            len += 1;
        }
        if self.divider_exponent != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.supply.v1.QueryCirculatingRequest", len)?;
        if !self.denom.is_empty() {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        if self.divider_exponent != 0 {
            struct_ser.serialize_field(
                "dividerExponent",
                ToString::to_string(&self.divider_exponent).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryCirculatingRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["denom", "divider_exponent", "dividerExponent"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Denom,
            DividerExponent,
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
                            "denom" => Ok(GeneratedField::Denom),
                            "dividerExponent" | "divider_exponent" => {
                                Ok(GeneratedField::DividerExponent)
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
            type Value = QueryCirculatingRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.supply.v1.QueryCirculatingRequest")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryCirculatingRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut denom__ = None;
                let mut divider_exponent__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Denom => {
                            if denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denom"));
                            }
                            denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::DividerExponent => {
                            if divider_exponent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dividerExponent"));
                            }
                            divider_exponent__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryCirculatingRequest {
                    denom: denom__.unwrap_or_default(),
                    divider_exponent: divider_exponent__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.supply.v1.QueryCirculatingRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryCirculatingResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.circulating_supply.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.supply.v1.QueryCirculatingResponse", len)?;
        if !self.circulating_supply.is_empty() {
            struct_ser.serialize_field("circulatingSupply", &self.circulating_supply)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryCirculatingResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["circulating_supply", "circulatingSupply"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CirculatingSupply,
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
                            "circulatingSupply" | "circulating_supply" => {
                                Ok(GeneratedField::CirculatingSupply)
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
            type Value = QueryCirculatingResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.supply.v1.QueryCirculatingResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryCirculatingResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut circulating_supply__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CirculatingSupply => {
                            if circulating_supply__.is_some() {
                                return Err(serde::de::Error::duplicate_field("circulatingSupply"));
                            }
                            circulating_supply__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QueryCirculatingResponse {
                    circulating_supply: circulating_supply__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.supply.v1.QueryCirculatingResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryTotalRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.denom.is_empty() {
            len += 1;
        }
        if self.divider_exponent != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.supply.v1.QueryTotalRequest", len)?;
        if !self.denom.is_empty() {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        if self.divider_exponent != 0 {
            struct_ser.serialize_field(
                "dividerExponent",
                ToString::to_string(&self.divider_exponent).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryTotalRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["denom", "divider_exponent", "dividerExponent"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Denom,
            DividerExponent,
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
                            "denom" => Ok(GeneratedField::Denom),
                            "dividerExponent" | "divider_exponent" => {
                                Ok(GeneratedField::DividerExponent)
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
            type Value = QueryTotalRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.supply.v1.QueryTotalRequest")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryTotalRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut denom__ = None;
                let mut divider_exponent__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Denom => {
                            if denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denom"));
                            }
                            denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::DividerExponent => {
                            if divider_exponent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dividerExponent"));
                            }
                            divider_exponent__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryTotalRequest {
                    denom: denom__.unwrap_or_default(),
                    divider_exponent: divider_exponent__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.supply.v1.QueryTotalRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryTotalResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.total_supply.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.supply.v1.QueryTotalResponse", len)?;
        if !self.total_supply.is_empty() {
            struct_ser.serialize_field("totalSupply", &self.total_supply)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryTotalResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["total_supply", "totalSupply"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TotalSupply,
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
                            "totalSupply" | "total_supply" => Ok(GeneratedField::TotalSupply),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryTotalResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.supply.v1.QueryTotalResponse")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryTotalResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut total_supply__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TotalSupply => {
                            if total_supply__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalSupply"));
                            }
                            total_supply__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QueryTotalResponse {
                    total_supply: total_supply__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.supply.v1.QueryTotalResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
