/// Subspace contains all the data of a Desmos subspace
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.subspaces.v1.Subspace")]
pub struct Subspace {
    /// Unique id that identifies the subspace
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// Human-readable name of the subspace
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Optional description of this subspace
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Represents the account that is associated with the subspace and
    /// should be used to connect external applications to verify this subspace
    #[prost(string, tag = "4")]
    pub treasury: ::prost::alloc::string::String,
    /// Address of the user that owns the subspace
    #[prost(string, tag = "5")]
    pub owner: ::prost::alloc::string::String,
    /// Address of the subspace creator
    #[prost(string, tag = "6")]
    pub creator: ::prost::alloc::string::String,
    /// the creation time of the subspace
    #[prost(message, optional, tag = "7")]
    pub creation_time: ::core::option::Option<crate::shim::Timestamp>,
}
/// UserGroup represents a group of users
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.subspaces.v1.UserGroup")]
pub struct UserGroup {
    /// ID of the subspace inside which this group exists
    #[prost(uint64, tag = "1")]
    pub subspace_id: u64,
    /// Unique id that identifies the group
    #[prost(uint32, tag = "2")]
    pub id: u32,
    /// Human-readable name of the user group
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Optional description of this group
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// Permissions that will be granted to all the users part of this group
    #[prost(uint32, tag = "5")]
    pub permissions: u32,
}
/// PermissionDetail contains the details data of a permission
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.subspaces.v1.PermissionDetail")]
pub struct PermissionDetail {
    /// sum is the oneof that specifies whether this represents a user or
    /// group permission detail
    #[prost(oneof = "permission_detail::Sum", tags = "1, 2")]
    pub sum: ::core::option::Option<permission_detail::Sum>,
}
/// Nested message and enum types in `PermissionDetail`.
pub mod permission_detail {
    /// User is a permission that has been set to a specific user
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
    #[proto_message(type_url = "/desmos.subspaces.v1.PermissionDetail.User")]
    pub struct User {
        /// User for which the permission was set
        #[prost(string, tag = "1")]
        pub user: ::prost::alloc::string::String,
        /// Permission set to the user
        #[prost(uint32, tag = "2")]
        pub permission: u32,
    }
    /// Group is a permission that has been set to a user group
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
    #[proto_message(type_url = "/desmos.subspaces.v1.PermissionDetail.Group")]
    pub struct Group {
        /// Unique id of the group
        #[prost(uint32, tag = "1")]
        pub group_id: u32,
        /// Permission set to the group
        #[prost(uint32, tag = "2")]
        pub permission: u32,
    }
    /// sum is the oneof that specifies whether this represents a user or
    /// group permission detail
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof, schemars::JsonSchema)]
    pub enum Sum {
        /// User represents a user permission
        #[prost(message, tag = "1")]
        User(User),
        /// Group represents a group permission
        #[prost(message, tag = "2")]
        Group(Group),
    }
}
impl serde::Serialize for PermissionDetail {
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
        let mut struct_ser =
            serializer.serialize_struct("desmos.subspaces.v1.PermissionDetail", len)?;
        if let Some(v) = self.sum.as_ref() {
            match v {
                permission_detail::Sum::User(v) => {
                    struct_ser.serialize_field("user", v)?;
                }
                permission_detail::Sum::Group(v) => {
                    struct_ser.serialize_field("group", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PermissionDetail {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["user", "group"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
            Group,
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
                            "user" => Ok(GeneratedField::User),
                            "group" => Ok(GeneratedField::Group),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PermissionDetail;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.subspaces.v1.PermissionDetail")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<PermissionDetail, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sum__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if sum__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            sum__ = map
                                .next_value::<::std::option::Option<_>>()?
                                .map(permission_detail::Sum::User);
                        }
                        GeneratedField::Group => {
                            if sum__.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            sum__ = map
                                .next_value::<::std::option::Option<_>>()?
                                .map(permission_detail::Sum::Group);
                        }
                    }
                }
                Ok(PermissionDetail { sum: sum__ })
            }
        }
        deserializer.deserialize_struct(
            "desmos.subspaces.v1.PermissionDetail",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for permission_detail::Group {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.group_id != 0 {
            len += 1;
        }
        if self.permission != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.subspaces.v1.PermissionDetail.Group", len)?;
        if self.group_id != 0 {
            struct_ser.serialize_field("groupId", &self.group_id)?;
        }
        if self.permission != 0 {
            struct_ser.serialize_field("permission", &self.permission)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for permission_detail::Group {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["group_id", "groupId", "permission"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GroupId,
            Permission,
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
                            "groupId" | "group_id" => Ok(GeneratedField::GroupId),
                            "permission" => Ok(GeneratedField::Permission),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = permission_detail::Group;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.subspaces.v1.PermissionDetail.Group")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<permission_detail::Group, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut group_id__ = None;
                let mut permission__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::GroupId => {
                            if group_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("groupId"));
                            }
                            group_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Permission => {
                            if permission__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permission"));
                            }
                            permission__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(permission_detail::Group {
                    group_id: group_id__.unwrap_or_default(),
                    permission: permission__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.subspaces.v1.PermissionDetail.Group",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for permission_detail::User {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user.is_empty() {
            len += 1;
        }
        if self.permission != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.subspaces.v1.PermissionDetail.User", len)?;
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        if self.permission != 0 {
            struct_ser.serialize_field("permission", &self.permission)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for permission_detail::User {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["user", "permission"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
            Permission,
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
                            "user" => Ok(GeneratedField::User),
                            "permission" => Ok(GeneratedField::Permission),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = permission_detail::User;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.subspaces.v1.PermissionDetail.User")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<permission_detail::User, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                let mut permission__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = Some(map.next_value()?);
                        }
                        GeneratedField::Permission => {
                            if permission__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permission"));
                            }
                            permission__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(permission_detail::User {
                    user: user__.unwrap_or_default(),
                    permission: permission__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.subspaces.v1.PermissionDetail.User",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Subspace {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.treasury.is_empty() {
            len += 1;
        }
        if !self.owner.is_empty() {
            len += 1;
        }
        if !self.creator.is_empty() {
            len += 1;
        }
        if self.creation_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.subspaces.v1.Subspace", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", ToString::to_string(&self.id).as_str())?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.treasury.is_empty() {
            struct_ser.serialize_field("treasury", &self.treasury)?;
        }
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if !self.creator.is_empty() {
            struct_ser.serialize_field("creator", &self.creator)?;
        }
        if let Some(v) = self.creation_time.as_ref() {
            struct_ser.serialize_field("creationTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Subspace {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "description",
            "treasury",
            "owner",
            "creator",
            "creation_time",
            "creationTime",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Description,
            Treasury,
            Owner,
            Creator,
            CreationTime,
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
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "treasury" => Ok(GeneratedField::Treasury),
                            "owner" => Ok(GeneratedField::Owner),
                            "creator" => Ok(GeneratedField::Creator),
                            "creationTime" | "creation_time" => Ok(GeneratedField::CreationTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Subspace;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.subspaces.v1.Subspace")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Subspace, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut treasury__ = None;
                let mut owner__ = None;
                let mut creator__ = None;
                let mut creation_time__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map.next_value()?);
                        }
                        GeneratedField::Treasury => {
                            if treasury__.is_some() {
                                return Err(serde::de::Error::duplicate_field("treasury"));
                            }
                            treasury__ = Some(map.next_value()?);
                        }
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map.next_value()?);
                        }
                        GeneratedField::Creator => {
                            if creator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creator"));
                            }
                            creator__ = Some(map.next_value()?);
                        }
                        GeneratedField::CreationTime => {
                            if creation_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationTime"));
                            }
                            creation_time__ = map.next_value()?;
                        }
                    }
                }
                Ok(Subspace {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    treasury: treasury__.unwrap_or_default(),
                    owner: owner__.unwrap_or_default(),
                    creator: creator__.unwrap_or_default(),
                    creation_time: creation_time__,
                })
            }
        }
        deserializer.deserialize_struct("desmos.subspaces.v1.Subspace", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserGroup {
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
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.permissions != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.subspaces.v1.UserGroup", len)?;
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.permissions != 0 {
            struct_ser.serialize_field("permissions", &self.permissions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserGroup {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subspace_id",
            "subspaceId",
            "id",
            "name",
            "description",
            "permissions",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspaceId,
            Id,
            Name,
            Description,
            Permissions,
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
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "permissions" => Ok(GeneratedField::Permissions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserGroup;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.subspaces.v1.UserGroup")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<UserGroup, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
                let mut id__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut permissions__ = None;
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
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map.next_value()?);
                        }
                        GeneratedField::Permissions => {
                            if permissions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissions"));
                            }
                            permissions__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(UserGroup {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    permissions: permissions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("desmos.subspaces.v1.UserGroup", FIELDS, GeneratedVisitor)
    }
}
