/// Post contains all the information about a single post
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
#[proto_message(type_url = "/desmos.posts.v2.Post")]
#[serde(rename_all = "snake_case")]
pub struct Post {
    /// Id of the subspace inside which the post has been created
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the section inside which the post has been created
    #[prost(uint32, tag = "2")]
    pub section_id: u32,
    /// Unique id of the post
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
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
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub conversation_id: u64,
    /// A list this posts references (either as a reply, repost or quote)
    #[prost(message, repeated, tag = "10")]
    pub referenced_posts: ::prost::alloc::vec::Vec<PostReference>,
    /// Reply settings of this post
    #[prost(enumeration = "ReplySetting", tag = "11")]
    #[serde(deserialize_with = "ReplySetting::deserialize")]
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
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.PostReference")]
#[serde(rename_all = "snake_case")]
pub struct PostReference {
    /// Type of reference
    #[prost(enumeration = "PostReferenceType", tag = "1")]
    #[serde(deserialize_with = "PostReferenceType::deserialize")]
    pub r#type: i32,
    /// Id of the referenced post
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub post_id: u64,
    /// Position of the reference inside the post's text. This should be used only
    /// with the type set to TYPE_QUOTE
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub position: u64,
}
/// Contains the details of entities parsed out of the post text
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
#[proto_message(type_url = "/desmos.posts.v2.Entities")]
#[serde(rename_all = "snake_case")]
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
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.TextTag")]
#[serde(rename_all = "snake_case")]
pub struct TextTag {
    /// Index of the character inside the text at which the tag starts
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub start: u64,
    /// Index of the character inside the text at which the tag ends
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub end: u64,
    /// Tag reference (user address, hashtag value, etc)
    #[prost(string, tag = "3")]
    pub tag: ::prost::alloc::string::String,
}
/// Url contains the details of a generic URL
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
#[proto_message(type_url = "/desmos.posts.v2.Url")]
#[serde(rename_all = "snake_case")]
pub struct Url {
    /// Index of the character inside the text at which the URL starts
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub start: u64,
    /// Index of the character inside the text at which the URL ends
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
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
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.Attachment")]
#[serde(rename_all = "snake_case")]
pub struct Attachment {
    /// Id of the subspace inside which the post to which this attachment should be
    /// connected is
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the post to which this attachment should be connected
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
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
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.Media")]
#[serde(rename_all = "snake_case")]
pub struct Media {
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub mime_type: ::prost::alloc::string::String,
}
/// Poll represents a poll attachment
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
#[proto_message(type_url = "/desmos.posts.v2.Poll")]
#[serde(rename_all = "snake_case")]
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
    #[derive(
        Clone,
        PartialEq,
        ::prost::Message,
        schemars::JsonSchema,
        serde::Serialize,
        serde::Deserialize,
        std_derive::CosmwasmExt,
    )]
    #[proto_message(type_url = "/desmos.posts.v2.Poll.ProvidedAnswer")]
    #[serde(rename_all = "snake_case")]
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
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.UserAnswer")]
#[serde(rename_all = "snake_case")]
pub struct UserAnswer {
    /// Subspace id inside which the post related to this attachment is located
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the post associated to this attachment
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
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
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.PollTallyResults")]
#[serde(rename_all = "snake_case")]
pub struct PollTallyResults {
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<poll_tally_results::AnswerResult>,
}
/// Nested message and enum types in `PollTallyResults`.
pub mod poll_tally_results {
    /// AnswerResult contains the result of a single poll provided answer
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
    #[proto_message(type_url = "/desmos.posts.v2.PollTallyResults.AnswerResult")]
    #[serde(rename_all = "snake_case")]
    pub struct AnswerResult {
        /// Index of the answer inside the poll's ProvidedAnswers slice
        #[prost(uint32, tag = "1")]
        pub answer_index: u32,
        /// Number of votes the answer has received
        #[prost(uint64, tag = "2")]
        #[serde(
            serialize_with = "crate::serde::as_str::serialize",
            deserialize_with = "crate::serde::as_str::deserialize"
        )]
        pub votes: u64,
    }
}
/// Params contains the parameters for the posts module
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
#[proto_message(type_url = "/desmos.posts.v2.Params")]
#[serde(rename_all = "snake_case")]
pub struct Params {
    /// Maximum length of the post text
    #[prost(uint32, tag = "1")]
    pub max_text_length: u32,
}
/// PostReferenceType represents the different types of references
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
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
    pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<i32, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str_name(s).unwrap() as i32)
    }
}
/// ReplySetting contains the possible reply settings that a post can have
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
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
    pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<i32, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str_name(s).unwrap() as i32)
    }
}
