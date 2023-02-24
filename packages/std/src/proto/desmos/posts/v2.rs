/// Post contains all the information about a single post
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.posts.v2.Post")]
pub struct Post {
    /// Id of the subspace inside which the post has been created
    #[prost(uint64, tag = "1")]
    pub subspace_id: u64,
    /// Id of the section inside which the post has been created
    #[prost(uint32, tag = "2")]
    pub section_id: u32,
    /// Unique id of the post
    #[prost(uint64, tag = "3")]
    pub id: u64,
    /// (optional) External id for this post
    #[prost(string, tag = "4")]
    pub external_id: ::prost::alloc::string::String,
    /// (optional) Text of the post
    #[prost(string, tag = "5")]
    pub text: ::prost::alloc::string::String,
    /// (optional) Entities connected to this post
    #[prost(message, optional, tag = "6")]
    pub entities: ::core::option::Option<Entities>,
    /// Tags related to this post, useful for categorization
    #[prost(string, repeated, tag = "7")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Author of the post
    #[prost(string, tag = "8")]
    pub author: ::prost::alloc::string::String,
    /// (optional) Id of the original post of the conversation
    #[prost(uint64, tag = "9")]
    pub conversation_id: u64,
    /// A list this posts references (either as a reply, repost or quote)
    #[prost(message, repeated, tag = "10")]
    pub referenced_posts: ::prost::alloc::vec::Vec<PostReference>,
    /// Reply settings of this post
    #[prost(enumeration = "ReplySetting", tag = "11")]
    pub reply_settings: i32,
    /// Creation date of the post
    #[prost(message, optional, tag = "12")]
    pub creation_date: ::core::option::Option<crate::shim::Timestamp>,
    /// (optional) Last edited time of the post
    #[prost(message, optional, tag = "13")]
    pub last_edited_date: ::core::option::Option<crate::shim::Timestamp>,
}
/// PostReference contains the details of a post reference
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.posts.v2.PostReference")]
pub struct PostReference {
    /// Type of reference
    #[prost(enumeration = "PostReferenceType", tag = "1")]
    pub r#type: i32,
    /// Id of the referenced post
    #[prost(uint64, tag = "2")]
    pub post_id: u64,
    /// Position of the reference inside the post's text. This should be used only
    /// with the type set to TYPE_QUOTE
    #[prost(uint64, tag = "3")]
    pub position: u64,
}
/// Contains the details of entities parsed out of the post text
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.posts.v2.Entities")]
pub struct Entities {
    /// Hashtags represent inside the post text
    #[prost(message, repeated, tag = "1")]
    pub hashtags: ::prost::alloc::vec::Vec<TextTag>,
    /// Mentions present inside the post text
    #[prost(message, repeated, tag = "2")]
    pub mentions: ::prost::alloc::vec::Vec<TextTag>,
    /// Links present inside the post text
    #[prost(message, repeated, tag = "3")]
    pub urls: ::prost::alloc::vec::Vec<Url>,
}
/// TextTag represents a tag within the post text
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.posts.v2.TextTag")]
pub struct TextTag {
    /// Index of the character inside the text at which the tag starts
    #[prost(uint64, tag = "1")]
    pub start: u64,
    /// Index of the character inside the text at which the tag ends
    #[prost(uint64, tag = "2")]
    pub end: u64,
    /// Tag reference (user address, hashtag value, etc)
    #[prost(string, tag = "3")]
    pub tag: ::prost::alloc::string::String,
}
/// Url contains the details of a generic URL
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.posts.v2.Url")]
pub struct Url {
    /// Index of the character inside the text at which the URL starts
    #[prost(uint64, tag = "1")]
    pub start: u64,
    /// Index of the character inside the text at which the URL ends
    #[prost(uint64, tag = "2")]
    pub end: u64,
    /// Value of the URL where the user should be redirected to
    #[prost(string, tag = "3")]
    pub url: ::prost::alloc::string::String,
    /// (optional) Display value of the URL
    #[prost(string, tag = "4")]
    pub display_url: ::prost::alloc::string::String,
}
/// Attachment contains the data of a single post attachment
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.posts.v2.Attachment")]
pub struct Attachment {
    /// Id of the subspace inside which the post to which this attachment should be
    /// connected is
    #[prost(uint64, tag = "1")]
    pub subspace_id: u64,
    /// Id of the post to which this attachment should be connected
    #[prost(uint64, tag = "2")]
    pub post_id: u64,
    /// If of this attachment
    #[prost(uint32, tag = "3")]
    pub id: u32,
    /// Content of the attachment
    #[prost(message, optional, tag = "4")]
    pub content: ::core::option::Option<crate::shim::Any>,
}
/// Media represents a media attachment
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.posts.v2.Media")]
pub struct Media {
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub mime_type: ::prost::alloc::string::String,
}
/// Poll represents a poll attachment
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.posts.v2.Poll")]
pub struct Poll {
    /// Question of the poll
    #[prost(string, tag = "1")]
    pub question: ::prost::alloc::string::String,
    /// Answers the users can choose from
    #[prost(message, repeated, tag = "2")]
    pub provided_answers: ::prost::alloc::vec::Vec<poll::ProvidedAnswer>,
    /// Date at which the poll will close
    #[prost(message, optional, tag = "3")]
    pub end_date: ::core::option::Option<crate::shim::Timestamp>,
    /// Whether the poll allows multiple choices from the same user or not
    #[prost(bool, tag = "4")]
    pub allows_multiple_answers: bool,
    /// Whether the poll allows to edit an answer or not
    #[prost(bool, tag = "5")]
    pub allows_answer_edits: bool,
    /// Final poll results
    #[prost(message, optional, tag = "6")]
    pub final_tally_results: ::core::option::Option<PollTallyResults>,
}
/// Nested message and enum types in `Poll`.
pub mod poll {
    /// Provided answer contains the details of a possible poll answer
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
    #[proto_message(type_url = "/desmos.posts.v2.Poll.ProvidedAnswer")]
    pub struct ProvidedAnswer {
        /// (optional) Text of the answer
        #[prost(string, tag = "1")]
        pub text: ::prost::alloc::string::String,
        /// Attachments of the answer
        #[prost(message, repeated, tag = "2")]
        pub attachments: ::prost::alloc::vec::Vec<super::Attachment>,
    }
}
/// UserAnswer represents a user answer to a poll
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.posts.v2.UserAnswer")]
pub struct UserAnswer {
    /// Subspace id inside which the post related to this attachment is located
    #[prost(uint64, tag = "1")]
    pub subspace_id: u64,
    /// Id of the post associated to this attachment
    #[prost(uint64, tag = "2")]
    pub post_id: u64,
    /// Id of the poll to which this answer is associated
    #[prost(uint32, tag = "3")]
    pub poll_id: u32,
    /// Indexes of the answers inside the ProvidedAnswers array
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub answers_indexes: ::prost::alloc::vec::Vec<u32>,
    /// Address of the user answering the poll
    #[prost(string, tag = "5")]
    pub user: ::prost::alloc::string::String,
}
/// PollTallyResults contains the tally results for a poll
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.posts.v2.PollTallyResults")]
pub struct PollTallyResults {
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<poll_tally_results::AnswerResult>,
}
/// Nested message and enum types in `PollTallyResults`.
pub mod poll_tally_results {
    /// AnswerResult contains the result of a single poll provided answer
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
    #[proto_message(type_url = "/desmos.posts.v2.PollTallyResults.AnswerResult")]
    pub struct AnswerResult {
        /// Index of the answer inside the poll's ProvidedAnswers slice
        #[prost(uint32, tag = "1")]
        pub answer_index: u32,
        /// Number of votes the answer has received
        #[prost(uint64, tag = "2")]
        pub votes: u64,
    }
}
/// Params contains the parameters for the posts module
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.posts.v2.Params")]
pub struct Params {
    /// Maximum length of the post text
    #[prost(uint32, tag = "1")]
    pub max_text_length: u32,
}
/// PostReferenceType represents the different types of references
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(schemars::JsonSchema)]
pub enum PostReferenceType {
    /// No reference specified
    Unspecified = 0,
    /// This reference represents a reply to the specified post
    Reply = 1,
    /// This reference represents a quote of the specified post
    Quote = 2,
    /// This reference represents a repost of the specified post
    Repost = 3,
}
impl PostReferenceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PostReferenceType::Unspecified => "POST_REFERENCE_TYPE_UNSPECIFIED",
            PostReferenceType::Reply => "POST_REFERENCE_TYPE_REPLY",
            PostReferenceType::Quote => "POST_REFERENCE_TYPE_QUOTE",
            PostReferenceType::Repost => "POST_REFERENCE_TYPE_REPOST",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "POST_REFERENCE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "POST_REFERENCE_TYPE_REPLY" => Some(Self::Reply),
            "POST_REFERENCE_TYPE_QUOTE" => Some(Self::Quote),
            "POST_REFERENCE_TYPE_REPOST" => Some(Self::Repost),
            _ => None,
        }
    }
}
/// ReplySetting contains the possible reply settings that a post can have
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(schemars::JsonSchema)]
pub enum ReplySetting {
    /// No reply setting specified
    Unspecified = 0,
    /// Everyone will be able to reply to this post
    Everyone = 1,
    /// Only followers of the author will be able to reply to this post
    Followers = 2,
    /// Only the author mutual followers will be able to reply to this post
    Mutual = 3,
    /// Only people mentioned inside this post will be able to reply
    Mentions = 4,
}
impl ReplySetting {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReplySetting::Unspecified => "REPLY_SETTING_UNSPECIFIED",
            ReplySetting::Everyone => "REPLY_SETTING_EVERYONE",
            ReplySetting::Followers => "REPLY_SETTING_FOLLOWERS",
            ReplySetting::Mutual => "REPLY_SETTING_MUTUAL",
            ReplySetting::Mentions => "REPLY_SETTING_MENTIONS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "REPLY_SETTING_UNSPECIFIED" => Some(Self::Unspecified),
            "REPLY_SETTING_EVERYONE" => Some(Self::Everyone),
            "REPLY_SETTING_FOLLOWERS" => Some(Self::Followers),
            "REPLY_SETTING_MUTUAL" => Some(Self::Mutual),
            "REPLY_SETTING_MENTIONS" => Some(Self::Mentions),
            _ => None,
        }
    }
}
impl serde::Serialize for Attachment {
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
        if self.content.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.posts.v2.Attachment", len)?;
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
        if let Some(v) = self.content.as_ref() {
            struct_ser.serialize_field("content", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Attachment {
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
            "content",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspaceId,
            PostId,
            Id,
            Content,
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
                            "content" => Ok(GeneratedField::Content),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Attachment;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.posts.v2.Attachment")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Attachment, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
                let mut post_id__ = None;
                let mut id__ = None;
                let mut content__ = None;
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
                        GeneratedField::Content => {
                            if content__.is_some() {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            content__ = map.next_value()?;
                        }
                    }
                }
                Ok(Attachment {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    post_id: post_id__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    content: content__,
                })
            }
        }
        deserializer.deserialize_struct("desmos.posts.v2.Attachment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Entities {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.hashtags.is_empty() {
            len += 1;
        }
        if !self.mentions.is_empty() {
            len += 1;
        }
        if !self.urls.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.posts.v2.Entities", len)?;
        if !self.hashtags.is_empty() {
            struct_ser.serialize_field("hashtags", &self.hashtags)?;
        }
        if !self.mentions.is_empty() {
            struct_ser.serialize_field("mentions", &self.mentions)?;
        }
        if !self.urls.is_empty() {
            struct_ser.serialize_field("urls", &self.urls)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Entities {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["hashtags", "mentions", "urls"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hashtags,
            Mentions,
            Urls,
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
                            "hashtags" => Ok(GeneratedField::Hashtags),
                            "mentions" => Ok(GeneratedField::Mentions),
                            "urls" => Ok(GeneratedField::Urls),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Entities;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.posts.v2.Entities")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Entities, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut hashtags__ = None;
                let mut mentions__ = None;
                let mut urls__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Hashtags => {
                            if hashtags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hashtags"));
                            }
                            hashtags__ = Some(map.next_value()?);
                        }
                        GeneratedField::Mentions => {
                            if mentions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mentions"));
                            }
                            mentions__ = Some(map.next_value()?);
                        }
                        GeneratedField::Urls => {
                            if urls__.is_some() {
                                return Err(serde::de::Error::duplicate_field("urls"));
                            }
                            urls__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Entities {
                    hashtags: hashtags__.unwrap_or_default(),
                    mentions: mentions__.unwrap_or_default(),
                    urls: urls__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("desmos.posts.v2.Entities", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Media {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.uri.is_empty() {
            len += 1;
        }
        if !self.mime_type.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.posts.v2.Media", len)?;
        if !self.uri.is_empty() {
            struct_ser.serialize_field("uri", &self.uri)?;
        }
        if !self.mime_type.is_empty() {
            struct_ser.serialize_field("mimeType", &self.mime_type)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Media {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["uri", "mime_type", "mimeType"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uri,
            MimeType,
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
                            "uri" => Ok(GeneratedField::Uri),
                            "mimeType" | "mime_type" => Ok(GeneratedField::MimeType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Media;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.posts.v2.Media")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Media, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut uri__ = None;
                let mut mime_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Uri => {
                            if uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uri"));
                            }
                            uri__ = Some(map.next_value()?);
                        }
                        GeneratedField::MimeType => {
                            if mime_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mimeType"));
                            }
                            mime_type__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Media {
                    uri: uri__.unwrap_or_default(),
                    mime_type: mime_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("desmos.posts.v2.Media", FIELDS, GeneratedVisitor)
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
        if self.max_text_length != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.posts.v2.Params", len)?;
        if self.max_text_length != 0 {
            struct_ser.serialize_field("maxTextLength", &self.max_text_length)?;
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
        const FIELDS: &[&str] = &["max_text_length", "maxTextLength"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxTextLength,
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
                            "maxTextLength" | "max_text_length" => {
                                Ok(GeneratedField::MaxTextLength)
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
                formatter.write_str("struct desmos.posts.v2.Params")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Params, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut max_text_length__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MaxTextLength => {
                            if max_text_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxTextLength"));
                            }
                            max_text_length__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(Params {
                    max_text_length: max_text_length__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("desmos.posts.v2.Params", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Poll {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.question.is_empty() {
            len += 1;
        }
        if !self.provided_answers.is_empty() {
            len += 1;
        }
        if self.end_date.is_some() {
            len += 1;
        }
        if self.allows_multiple_answers {
            len += 1;
        }
        if self.allows_answer_edits {
            len += 1;
        }
        if self.final_tally_results.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.posts.v2.Poll", len)?;
        if !self.question.is_empty() {
            struct_ser.serialize_field("question", &self.question)?;
        }
        if !self.provided_answers.is_empty() {
            struct_ser.serialize_field("providedAnswers", &self.provided_answers)?;
        }
        if let Some(v) = self.end_date.as_ref() {
            struct_ser.serialize_field("endDate", v)?;
        }
        if self.allows_multiple_answers {
            struct_ser.serialize_field("allowsMultipleAnswers", &self.allows_multiple_answers)?;
        }
        if self.allows_answer_edits {
            struct_ser.serialize_field("allowsAnswerEdits", &self.allows_answer_edits)?;
        }
        if let Some(v) = self.final_tally_results.as_ref() {
            struct_ser.serialize_field("finalTallyResults", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Poll {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "question",
            "provided_answers",
            "providedAnswers",
            "end_date",
            "endDate",
            "allows_multiple_answers",
            "allowsMultipleAnswers",
            "allows_answer_edits",
            "allowsAnswerEdits",
            "final_tally_results",
            "finalTallyResults",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Question,
            ProvidedAnswers,
            EndDate,
            AllowsMultipleAnswers,
            AllowsAnswerEdits,
            FinalTallyResults,
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
                            "question" => Ok(GeneratedField::Question),
                            "providedAnswers" | "provided_answers" => {
                                Ok(GeneratedField::ProvidedAnswers)
                            }
                            "endDate" | "end_date" => Ok(GeneratedField::EndDate),
                            "allowsMultipleAnswers" | "allows_multiple_answers" => {
                                Ok(GeneratedField::AllowsMultipleAnswers)
                            }
                            "allowsAnswerEdits" | "allows_answer_edits" => {
                                Ok(GeneratedField::AllowsAnswerEdits)
                            }
                            "finalTallyResults" | "final_tally_results" => {
                                Ok(GeneratedField::FinalTallyResults)
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
            type Value = Poll;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.posts.v2.Poll")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Poll, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut question__ = None;
                let mut provided_answers__ = None;
                let mut end_date__ = None;
                let mut allows_multiple_answers__ = None;
                let mut allows_answer_edits__ = None;
                let mut final_tally_results__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Question => {
                            if question__.is_some() {
                                return Err(serde::de::Error::duplicate_field("question"));
                            }
                            question__ = Some(map.next_value()?);
                        }
                        GeneratedField::ProvidedAnswers => {
                            if provided_answers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providedAnswers"));
                            }
                            provided_answers__ = Some(map.next_value()?);
                        }
                        GeneratedField::EndDate => {
                            if end_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endDate"));
                            }
                            end_date__ = map.next_value()?;
                        }
                        GeneratedField::AllowsMultipleAnswers => {
                            if allows_multiple_answers__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "allowsMultipleAnswers",
                                ));
                            }
                            allows_multiple_answers__ = Some(map.next_value()?);
                        }
                        GeneratedField::AllowsAnswerEdits => {
                            if allows_answer_edits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowsAnswerEdits"));
                            }
                            allows_answer_edits__ = Some(map.next_value()?);
                        }
                        GeneratedField::FinalTallyResults => {
                            if final_tally_results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("finalTallyResults"));
                            }
                            final_tally_results__ = map.next_value()?;
                        }
                    }
                }
                Ok(Poll {
                    question: question__.unwrap_or_default(),
                    provided_answers: provided_answers__.unwrap_or_default(),
                    end_date: end_date__,
                    allows_multiple_answers: allows_multiple_answers__.unwrap_or_default(),
                    allows_answer_edits: allows_answer_edits__.unwrap_or_default(),
                    final_tally_results: final_tally_results__,
                })
            }
        }
        deserializer.deserialize_struct("desmos.posts.v2.Poll", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for poll::ProvidedAnswer {
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
        if !self.attachments.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.posts.v2.Poll.ProvidedAnswer", len)?;
        if !self.text.is_empty() {
            struct_ser.serialize_field("text", &self.text)?;
        }
        if !self.attachments.is_empty() {
            struct_ser.serialize_field("attachments", &self.attachments)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for poll::ProvidedAnswer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["text", "attachments"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Text,
            Attachments,
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
                            "attachments" => Ok(GeneratedField::Attachments),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = poll::ProvidedAnswer;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.posts.v2.Poll.ProvidedAnswer")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<poll::ProvidedAnswer, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut text__ = None;
                let mut attachments__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Text => {
                            if text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            text__ = Some(map.next_value()?);
                        }
                        GeneratedField::Attachments => {
                            if attachments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attachments"));
                            }
                            attachments__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(poll::ProvidedAnswer {
                    text: text__.unwrap_or_default(),
                    attachments: attachments__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.posts.v2.Poll.ProvidedAnswer",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for PollTallyResults {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.results.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.posts.v2.PollTallyResults", len)?;
        if !self.results.is_empty() {
            struct_ser.serialize_field("results", &self.results)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PollTallyResults {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["results"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Results,
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
                            "results" => Ok(GeneratedField::Results),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PollTallyResults;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.posts.v2.PollTallyResults")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<PollTallyResults, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut results__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Results => {
                            if results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("results"));
                            }
                            results__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(PollTallyResults {
                    results: results__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.posts.v2.PollTallyResults",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for poll_tally_results::AnswerResult {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.answer_index != 0 {
            len += 1;
        }
        if self.votes != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.posts.v2.PollTallyResults.AnswerResult", len)?;
        if self.answer_index != 0 {
            struct_ser.serialize_field("answerIndex", &self.answer_index)?;
        }
        if self.votes != 0 {
            struct_ser.serialize_field("votes", ToString::to_string(&self.votes).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for poll_tally_results::AnswerResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["answer_index", "answerIndex", "votes"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AnswerIndex,
            Votes,
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
                            "answerIndex" | "answer_index" => Ok(GeneratedField::AnswerIndex),
                            "votes" => Ok(GeneratedField::Votes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = poll_tally_results::AnswerResult;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.posts.v2.PollTallyResults.AnswerResult")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<poll_tally_results::AnswerResult, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut answer_index__ = None;
                let mut votes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AnswerIndex => {
                            if answer_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("answerIndex"));
                            }
                            answer_index__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Votes => {
                            if votes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("votes"));
                            }
                            votes__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(poll_tally_results::AnswerResult {
                    answer_index: answer_index__.unwrap_or_default(),
                    votes: votes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.posts.v2.PollTallyResults.AnswerResult",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Post {
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
        if self.section_id != 0 {
            len += 1;
        }
        if self.id != 0 {
            len += 1;
        }
        if !self.external_id.is_empty() {
            len += 1;
        }
        if !self.text.is_empty() {
            len += 1;
        }
        if self.entities.is_some() {
            len += 1;
        }
        if !self.tags.is_empty() {
            len += 1;
        }
        if !self.author.is_empty() {
            len += 1;
        }
        if self.conversation_id != 0 {
            len += 1;
        }
        if !self.referenced_posts.is_empty() {
            len += 1;
        }
        if self.reply_settings != 0 {
            len += 1;
        }
        if self.creation_date.is_some() {
            len += 1;
        }
        if self.last_edited_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.posts.v2.Post", len)?;
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        if self.section_id != 0 {
            struct_ser.serialize_field("sectionId", &self.section_id)?;
        }
        if self.id != 0 {
            struct_ser.serialize_field("id", ToString::to_string(&self.id).as_str())?;
        }
        if !self.external_id.is_empty() {
            struct_ser.serialize_field("externalId", &self.external_id)?;
        }
        if !self.text.is_empty() {
            struct_ser.serialize_field("text", &self.text)?;
        }
        if let Some(v) = self.entities.as_ref() {
            struct_ser.serialize_field("entities", v)?;
        }
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
        }
        if !self.author.is_empty() {
            struct_ser.serialize_field("author", &self.author)?;
        }
        if self.conversation_id != 0 {
            struct_ser.serialize_field(
                "conversationId",
                ToString::to_string(&self.conversation_id).as_str(),
            )?;
        }
        if !self.referenced_posts.is_empty() {
            struct_ser.serialize_field("referencedPosts", &self.referenced_posts)?;
        }
        if self.reply_settings != 0 {
            let v = ReplySetting::from_i32(self.reply_settings).ok_or_else(|| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.reply_settings))
            })?;
            struct_ser.serialize_field("replySettings", &v)?;
        }
        if let Some(v) = self.creation_date.as_ref() {
            struct_ser.serialize_field("creationDate", v)?;
        }
        if let Some(v) = self.last_edited_date.as_ref() {
            struct_ser.serialize_field("lastEditedDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Post {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subspace_id",
            "subspaceId",
            "section_id",
            "sectionId",
            "id",
            "external_id",
            "externalId",
            "text",
            "entities",
            "tags",
            "author",
            "conversation_id",
            "conversationId",
            "referenced_posts",
            "referencedPosts",
            "reply_settings",
            "replySettings",
            "creation_date",
            "creationDate",
            "last_edited_date",
            "lastEditedDate",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspaceId,
            SectionId,
            Id,
            ExternalId,
            Text,
            Entities,
            Tags,
            Author,
            ConversationId,
            ReferencedPosts,
            ReplySettings,
            CreationDate,
            LastEditedDate,
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
                            "sectionId" | "section_id" => Ok(GeneratedField::SectionId),
                            "id" => Ok(GeneratedField::Id),
                            "externalId" | "external_id" => Ok(GeneratedField::ExternalId),
                            "text" => Ok(GeneratedField::Text),
                            "entities" => Ok(GeneratedField::Entities),
                            "tags" => Ok(GeneratedField::Tags),
                            "author" => Ok(GeneratedField::Author),
                            "conversationId" | "conversation_id" => {
                                Ok(GeneratedField::ConversationId)
                            }
                            "referencedPosts" | "referenced_posts" => {
                                Ok(GeneratedField::ReferencedPosts)
                            }
                            "replySettings" | "reply_settings" => Ok(GeneratedField::ReplySettings),
                            "creationDate" | "creation_date" => Ok(GeneratedField::CreationDate),
                            "lastEditedDate" | "last_edited_date" => {
                                Ok(GeneratedField::LastEditedDate)
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
            type Value = Post;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.posts.v2.Post")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Post, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
                let mut section_id__ = None;
                let mut id__ = None;
                let mut external_id__ = None;
                let mut text__ = None;
                let mut entities__ = None;
                let mut tags__ = None;
                let mut author__ = None;
                let mut conversation_id__ = None;
                let mut referenced_posts__ = None;
                let mut reply_settings__ = None;
                let mut creation_date__ = None;
                let mut last_edited_date__ = None;
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
                        GeneratedField::SectionId => {
                            if section_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sectionId"));
                            }
                            section_id__ = Some(
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
                        GeneratedField::ExternalId => {
                            if external_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("externalId"));
                            }
                            external_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Text => {
                            if text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            text__ = Some(map.next_value()?);
                        }
                        GeneratedField::Entities => {
                            if entities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entities"));
                            }
                            entities__ = map.next_value()?;
                        }
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = Some(map.next_value()?);
                        }
                        GeneratedField::Author => {
                            if author__.is_some() {
                                return Err(serde::de::Error::duplicate_field("author"));
                            }
                            author__ = Some(map.next_value()?);
                        }
                        GeneratedField::ConversationId => {
                            if conversation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conversationId"));
                            }
                            conversation_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ReferencedPosts => {
                            if referenced_posts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referencedPosts"));
                            }
                            referenced_posts__ = Some(map.next_value()?);
                        }
                        GeneratedField::ReplySettings => {
                            if reply_settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("replySettings"));
                            }
                            reply_settings__ = Some(map.next_value::<ReplySetting>()? as i32);
                        }
                        GeneratedField::CreationDate => {
                            if creation_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationDate"));
                            }
                            creation_date__ = map.next_value()?;
                        }
                        GeneratedField::LastEditedDate => {
                            if last_edited_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastEditedDate"));
                            }
                            last_edited_date__ = map.next_value()?;
                        }
                    }
                }
                Ok(Post {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    section_id: section_id__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    external_id: external_id__.unwrap_or_default(),
                    text: text__.unwrap_or_default(),
                    entities: entities__,
                    tags: tags__.unwrap_or_default(),
                    author: author__.unwrap_or_default(),
                    conversation_id: conversation_id__.unwrap_or_default(),
                    referenced_posts: referenced_posts__.unwrap_or_default(),
                    reply_settings: reply_settings__.unwrap_or_default(),
                    creation_date: creation_date__,
                    last_edited_date: last_edited_date__,
                })
            }
        }
        deserializer.deserialize_struct("desmos.posts.v2.Post", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PostReference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type != 0 {
            len += 1;
        }
        if self.post_id != 0 {
            len += 1;
        }
        if self.position != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.posts.v2.PostReference", len)?;
        if self.r#type != 0 {
            let v = PostReferenceType::from_i32(self.r#type).ok_or_else(|| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.r#type))
            })?;
            struct_ser.serialize_field("type", &v)?;
        }
        if self.post_id != 0 {
            struct_ser.serialize_field("postId", ToString::to_string(&self.post_id).as_str())?;
        }
        if self.position != 0 {
            struct_ser.serialize_field("position", ToString::to_string(&self.position).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PostReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["type", "post_id", "postId", "position"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            PostId,
            Position,
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
                            "type" => Ok(GeneratedField::Type),
                            "postId" | "post_id" => Ok(GeneratedField::PostId),
                            "position" => Ok(GeneratedField::Position),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PostReference;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.posts.v2.PostReference")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<PostReference, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut post_id__ = None;
                let mut position__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map.next_value::<PostReferenceType>()? as i32);
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
                        GeneratedField::Position => {
                            if position__.is_some() {
                                return Err(serde::de::Error::duplicate_field("position"));
                            }
                            position__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(PostReference {
                    r#type: r#type__.unwrap_or_default(),
                    post_id: post_id__.unwrap_or_default(),
                    position: position__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("desmos.posts.v2.PostReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PostReferenceType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "POST_REFERENCE_TYPE_UNSPECIFIED",
            Self::Reply => "POST_REFERENCE_TYPE_REPLY",
            Self::Quote => "POST_REFERENCE_TYPE_QUOTE",
            Self::Repost => "POST_REFERENCE_TYPE_REPOST",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PostReferenceType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "POST_REFERENCE_TYPE_UNSPECIFIED",
            "POST_REFERENCE_TYPE_REPLY",
            "POST_REFERENCE_TYPE_QUOTE",
            "POST_REFERENCE_TYPE_REPOST",
        ];
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PostReferenceType;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }
            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(PostReferenceType::from_i32)
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
                    .and_then(PostReferenceType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }
            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "POST_REFERENCE_TYPE_UNSPECIFIED" => Ok(PostReferenceType::Unspecified),
                    "POST_REFERENCE_TYPE_REPLY" => Ok(PostReferenceType::Reply),
                    "POST_REFERENCE_TYPE_QUOTE" => Ok(PostReferenceType::Quote),
                    "POST_REFERENCE_TYPE_REPOST" => Ok(PostReferenceType::Repost),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ReplySetting {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "REPLY_SETTING_UNSPECIFIED",
            Self::Everyone => "REPLY_SETTING_EVERYONE",
            Self::Followers => "REPLY_SETTING_FOLLOWERS",
            Self::Mutual => "REPLY_SETTING_MUTUAL",
            Self::Mentions => "REPLY_SETTING_MENTIONS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ReplySetting {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "REPLY_SETTING_UNSPECIFIED",
            "REPLY_SETTING_EVERYONE",
            "REPLY_SETTING_FOLLOWERS",
            "REPLY_SETTING_MUTUAL",
            "REPLY_SETTING_MENTIONS",
        ];
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReplySetting;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }
            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(ReplySetting::from_i32)
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
                    .and_then(ReplySetting::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }
            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "REPLY_SETTING_UNSPECIFIED" => Ok(ReplySetting::Unspecified),
                    "REPLY_SETTING_EVERYONE" => Ok(ReplySetting::Everyone),
                    "REPLY_SETTING_FOLLOWERS" => Ok(ReplySetting::Followers),
                    "REPLY_SETTING_MUTUAL" => Ok(ReplySetting::Mutual),
                    "REPLY_SETTING_MENTIONS" => Ok(ReplySetting::Mentions),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TextTag {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start != 0 {
            len += 1;
        }
        if self.end != 0 {
            len += 1;
        }
        if !self.tag.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.posts.v2.TextTag", len)?;
        if self.start != 0 {
            struct_ser.serialize_field("start", ToString::to_string(&self.start).as_str())?;
        }
        if self.end != 0 {
            struct_ser.serialize_field("end", ToString::to_string(&self.end).as_str())?;
        }
        if !self.tag.is_empty() {
            struct_ser.serialize_field("tag", &self.tag)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TextTag {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["start", "end", "tag"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Start,
            End,
            Tag,
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
                            "start" => Ok(GeneratedField::Start),
                            "end" => Ok(GeneratedField::End),
                            "tag" => Ok(GeneratedField::Tag),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TextTag;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.posts.v2.TextTag")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<TextTag, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut start__ = None;
                let mut end__ = None;
                let mut tag__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Start => {
                            if start__.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            start__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::End => {
                            if end__.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            end__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Tag => {
                            if tag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            tag__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(TextTag {
                    start: start__.unwrap_or_default(),
                    end: end__.unwrap_or_default(),
                    tag: tag__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("desmos.posts.v2.TextTag", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Url {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start != 0 {
            len += 1;
        }
        if self.end != 0 {
            len += 1;
        }
        if !self.url.is_empty() {
            len += 1;
        }
        if !self.display_url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.posts.v2.Url", len)?;
        if self.start != 0 {
            struct_ser.serialize_field("start", ToString::to_string(&self.start).as_str())?;
        }
        if self.end != 0 {
            struct_ser.serialize_field("end", ToString::to_string(&self.end).as_str())?;
        }
        if !self.url.is_empty() {
            struct_ser.serialize_field("url", &self.url)?;
        }
        if !self.display_url.is_empty() {
            struct_ser.serialize_field("displayUrl", &self.display_url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Url {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["start", "end", "url", "display_url", "displayUrl"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Start,
            End,
            Url,
            DisplayUrl,
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
                            "start" => Ok(GeneratedField::Start),
                            "end" => Ok(GeneratedField::End),
                            "url" => Ok(GeneratedField::Url),
                            "displayUrl" | "display_url" => Ok(GeneratedField::DisplayUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Url;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.posts.v2.Url")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Url, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut start__ = None;
                let mut end__ = None;
                let mut url__ = None;
                let mut display_url__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Start => {
                            if start__.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            start__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::End => {
                            if end__.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            end__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = Some(map.next_value()?);
                        }
                        GeneratedField::DisplayUrl => {
                            if display_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayUrl"));
                            }
                            display_url__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Url {
                    start: start__.unwrap_or_default(),
                    end: end__.unwrap_or_default(),
                    url: url__.unwrap_or_default(),
                    display_url: display_url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("desmos.posts.v2.Url", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserAnswer {
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
        if self.poll_id != 0 {
            len += 1;
        }
        if !self.answers_indexes.is_empty() {
            len += 1;
        }
        if !self.user.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.posts.v2.UserAnswer", len)?;
        if self.subspace_id != 0 {
            struct_ser.serialize_field(
                "subspaceId",
                ToString::to_string(&self.subspace_id).as_str(),
            )?;
        }
        if self.post_id != 0 {
            struct_ser.serialize_field("postId", ToString::to_string(&self.post_id).as_str())?;
        }
        if self.poll_id != 0 {
            struct_ser.serialize_field("pollId", &self.poll_id)?;
        }
        if !self.answers_indexes.is_empty() {
            struct_ser.serialize_field("answersIndexes", &self.answers_indexes)?;
        }
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserAnswer {
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
            "poll_id",
            "pollId",
            "answers_indexes",
            "answersIndexes",
            "user",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubspaceId,
            PostId,
            PollId,
            AnswersIndexes,
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
                            "pollId" | "poll_id" => Ok(GeneratedField::PollId),
                            "answersIndexes" | "answers_indexes" => {
                                Ok(GeneratedField::AnswersIndexes)
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
            type Value = UserAnswer;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.posts.v2.UserAnswer")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<UserAnswer, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut subspace_id__ = None;
                let mut post_id__ = None;
                let mut poll_id__ = None;
                let mut answers_indexes__ = None;
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
                        GeneratedField::PollId => {
                            if poll_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pollId"));
                            }
                            poll_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::AnswersIndexes => {
                            if answers_indexes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("answersIndexes"));
                            }
                            answers_indexes__ = Some(
                                map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter()
                                    .map(|x| x.0)
                                    .collect(),
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
                Ok(UserAnswer {
                    subspace_id: subspace_id__.unwrap_or_default(),
                    post_id: post_id__.unwrap_or_default(),
                    poll_id: poll_id__.unwrap_or_default(),
                    answers_indexes: answers_indexes__.unwrap_or_default(),
                    user: user__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("desmos.posts.v2.UserAnswer", FIELDS, GeneratedVisitor)
    }
}
