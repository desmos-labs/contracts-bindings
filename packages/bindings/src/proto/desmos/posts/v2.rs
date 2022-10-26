/// Post contains all the information about a single post
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.Post")]
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
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
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
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub reply_settings: i32,
    /// Creation date of the post
    #[prost(message, optional, tag = "12")]
    pub creation_date: ::core::option::Option<crate::shim::Timestamp>,
    /// (optional) Last edited time of the post
    #[prost(message, optional, tag = "13")]
    pub last_edited_date: ::core::option::Option<crate::shim::Timestamp>,
}
/// PostReference contains the details of a post reference
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.PostReference")]
pub struct PostReference {
    /// Type of reference
    #[prost(enumeration = "PostReferenceType", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
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
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
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
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.TextTag")]
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
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.Url")]
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
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.Attachment")]
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
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u32,
    /// Content of the attachment
    #[prost(message, optional, tag = "4")]
    pub content: ::core::option::Option<crate::shim::Any>,
}
/// Media represents a media attachment
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.Media")]
pub struct Media {
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub mime_type: ::prost::alloc::string::String,
}
/// Poll represents a poll attachment
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
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
    #[derive(
        Clone,
        PartialEq,
        ::prost::Message,
        serde::Serialize,
        serde::Deserialize,
        schemars::JsonSchema,
        std_derive::CosmwasmExt,
    )]
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
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.UserAnswer")]
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
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub poll_id: u32,
    /// Indexes of the answers inside the ProvidedAnswers array
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub answers_indexes: ::prost::alloc::vec::Vec<u32>,
    /// Address of the user answering the poll
    #[prost(string, tag = "5")]
    pub user: ::prost::alloc::string::String,
}
/// PollTallyResults contains the tally results for a poll
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.PollTallyResults")]
pub struct PollTallyResults {
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<poll_tally_results::AnswerResult>,
}
/// Nested message and enum types in `PollTallyResults`.
pub mod poll_tally_results {
    /// AnswerResult contains the result of a single poll provided answer
    #[derive(
        Clone,
        PartialEq,
        ::prost::Message,
        serde::Serialize,
        serde::Deserialize,
        schemars::JsonSchema,
        std_derive::CosmwasmExt,
    )]
    #[proto_message(type_url = "/desmos.posts.v2.PollTallyResults.AnswerResult")]
    pub struct AnswerResult {
        /// Index of the answer inside the poll's ProvidedAnswers slice
        #[prost(uint32, tag = "1")]
        #[serde(
            serialize_with = "crate::serde::as_str::serialize",
            deserialize_with = "crate::serde::as_str::deserialize"
        )]
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
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.Params")]
pub struct Params {
    /// Maximum length of the post text
    #[prost(uint32, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_text_length: u32,
}
/// PostReferenceType represents the different types of references
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
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
/// ReplySetting contains the possible reply settings that a post can have
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
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
/// QuerySubspacePostsRequest is the request type for the Query/SubspacePosts RPC
/// method
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.QuerySubspacePostsRequest")]
#[proto_query(
    path = "/desmos.posts.v2.Query/SubspacePosts",
    response_type = QuerySubspacePostsResponse
)]
pub struct QuerySubspacePostsRequest {
    /// Id of the subspace to query the posts for
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QuerySubspacePostsResponse is the response type for the Query/SubspacePosts
/// RPC method
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.QuerySubspacePostsResponse")]
pub struct QuerySubspacePostsResponse {
    #[prost(message, repeated, tag = "1")]
    pub posts: ::prost::alloc::vec::Vec<Post>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QuerySectionPostsRequest is the request type for the Query/SectionPosts RPC
/// method
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.QuerySectionPostsRequest")]
#[proto_query(
    path = "/desmos.posts.v2.Query/SectionPosts",
    response_type = QuerySectionPostsResponse
)]
pub struct QuerySectionPostsRequest {
    /// Id of the subspace to query the posts for
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the section to query the posts for
    #[prost(uint32, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub section_id: u32,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QuerySectionPostsResponse is the response type for the Query/SectionPosts RPC
/// method
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.QuerySectionPostsResponse")]
pub struct QuerySectionPostsResponse {
    #[prost(message, repeated, tag = "1")]
    pub posts: ::prost::alloc::vec::Vec<Post>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryPostRequest is the request type for the Query/Post RPC method
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.QueryPostRequest")]
#[proto_query(path = "/desmos.posts.v2.Query/Post", response_type = QueryPostResponse)]
pub struct QueryPostRequest {
    /// Id of the subspace inside which the post lies
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the post to query for
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub post_id: u64,
}
/// QueryPostResponse is the response type for the Query/Post RPC method
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.QueryPostResponse")]
pub struct QueryPostResponse {
    #[prost(message, optional, tag = "1")]
    pub post: ::core::option::Option<Post>,
}
/// QueryPostsRequest is the request type for the Query/PostAttachments RPC
/// method
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.QueryPostAttachmentsRequest")]
#[proto_query(
    path = "/desmos.posts.v2.Query/PostAttachments",
    response_type = QueryPostAttachmentsResponse
)]
pub struct QueryPostAttachmentsRequest {
    /// Id of the subspace where the post is stored
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the post to query the attachments for
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub post_id: u64,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryPostAttachmentsResponse is the response type for the
/// Query/PostAttachments RPC method
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.QueryPostAttachmentsResponse")]
pub struct QueryPostAttachmentsResponse {
    #[prost(message, repeated, tag = "1")]
    pub attachments: ::prost::alloc::vec::Vec<Attachment>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryPollAnswersRequest is the request type for the Query/PollAnswers RPC
/// method
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.QueryPollAnswersRequest")]
#[proto_query(
    path = "/desmos.posts.v2.Query/PollAnswers",
    response_type = QueryPollAnswersResponse
)]
pub struct QueryPollAnswersRequest {
    /// Id of the subspace where the post is stored
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the post that holds the poll
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub post_id: u64,
    /// Id of the poll to query the answers for
    #[prost(uint32, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub poll_id: u32,
    /// (Optional) Address of the user to query the responses for
    #[prost(string, tag = "4")]
    pub user: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "5")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryPollAnswersResponse is the response type for the Query/PollAnswers RPC
/// method
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.QueryPollAnswersResponse")]
pub struct QueryPollAnswersResponse {
    #[prost(message, repeated, tag = "1")]
    pub answers: ::prost::alloc::vec::Vec<UserAnswer>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.QueryParamsRequest")]
#[proto_query(
    path = "/desmos.posts.v2.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.QueryParamsResponse")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// MsgCreatePost represents the message to be used to create a post.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.MsgCreatePost")]
pub struct MsgCreatePost {
    /// Id of the subspace inside which the post must be created
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the section inside which the post must be created
    #[prost(uint32, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub section_id: u32,
    /// (optional) External id for this post
    #[prost(string, tag = "3")]
    pub external_id: ::prost::alloc::string::String,
    /// (optional) Text of the post
    #[prost(string, tag = "4")]
    pub text: ::prost::alloc::string::String,
    /// (optional) Entities connected to this post
    #[prost(message, optional, tag = "5")]
    pub entities: ::core::option::Option<Entities>,
    /// Tags connected to this post
    #[prost(string, repeated, tag = "6")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Attachments of the post
    #[prost(message, repeated, tag = "7")]
    pub attachments: ::prost::alloc::vec::Vec<crate::shim::Any>,
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
    /// Reply settings of this post
    #[prost(enumeration = "ReplySetting", tag = "10")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub reply_settings: i32,
    /// A list this posts references (either as a reply, repost or quote)
    #[prost(message, repeated, tag = "11")]
    pub referenced_posts: ::prost::alloc::vec::Vec<PostReference>,
}
/// MsgCreatePostResponse defines the Msg/CreatePost response type.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.MsgCreatePostResponse")]
pub struct MsgCreatePostResponse {
    /// Id of the newly created post
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub post_id: u64,
    /// Creation date of the post
    #[prost(message, optional, tag = "2")]
    pub creation_date: ::core::option::Option<crate::shim::Timestamp>,
}
/// MsgEditPost represents the message to be used to edit a post.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.MsgEditPost")]
pub struct MsgEditPost {
    /// Id of the subspace inside which the post is
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the post to edit
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub post_id: u64,
    /// New text of the post. If set to \[do-not-modify\] it will change the current
    /// post's text.
    #[prost(string, tag = "3")]
    pub text: ::prost::alloc::string::String,
    /// New entities connected to this post. These will always replace the current
    /// post's entities
    #[prost(message, optional, tag = "4")]
    pub entities: ::core::option::Option<Entities>,
    /// New tags connected to this post. These will always replace the current
    /// post's tags
    #[prost(string, repeated, tag = "5")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Editor of the post
    #[prost(string, tag = "6")]
    pub editor: ::prost::alloc::string::String,
}
/// MsgCreatePostResponse defines the Msg/EditPost response type.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.MsgEditPostResponse")]
pub struct MsgEditPostResponse {
    /// Edit date of the post
    #[prost(message, optional, tag = "1")]
    pub edit_date: ::core::option::Option<crate::shim::Timestamp>,
}
/// MsgDeletePost represents the message used when deleting a post.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.MsgDeletePost")]
pub struct MsgDeletePost {
    /// Id of the subspace containing the post
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the post to be deleted
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub post_id: u64,
    /// User that is deleting the post
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgDeletePostResponse represents the Msg/DeletePost response type
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.MsgDeletePostResponse")]
pub struct MsgDeletePostResponse {}
/// MsgAddPostAttachment represents the message that should be
/// used when adding an attachment to post
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.MsgAddPostAttachment")]
pub struct MsgAddPostAttachment {
    /// Id of the subspace containing the post
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the post to which to add the attachment
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub post_id: u64,
    /// Content of the attachment
    #[prost(message, optional, tag = "3")]
    pub content: ::core::option::Option<crate::shim::Any>,
    /// Editor of the post
    #[prost(string, tag = "4")]
    pub editor: ::prost::alloc::string::String,
}
/// MsgAddPostAttachmentResponse defines the Msg/AddPostAttachment response type.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.MsgAddPostAttachmentResponse")]
pub struct MsgAddPostAttachmentResponse {
    /// New id of the uploaded attachment
    #[prost(uint32, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub attachment_id: u32,
    /// Edit date of the post
    #[prost(message, optional, tag = "2")]
    pub edit_date: ::core::option::Option<crate::shim::Timestamp>,
}
/// MsgRemovePostAttachment represents the message to be used when
/// removing an attachment from a post
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.MsgRemovePostAttachment")]
pub struct MsgRemovePostAttachment {
    /// Id of the subspace containing the post
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the post from which to remove the attachment
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub post_id: u64,
    /// Id of the attachment to be removed
    #[prost(uint32, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub attachment_id: u32,
    /// User that is removing the attachment
    #[prost(string, tag = "4")]
    pub editor: ::prost::alloc::string::String,
}
/// MsgRemovePostAttachmentResponse defines the
/// Msg/RemovePostAttachment response type.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.MsgRemovePostAttachmentResponse")]
pub struct MsgRemovePostAttachmentResponse {
    /// Edit date of the post
    #[prost(message, optional, tag = "1")]
    pub edit_date: ::core::option::Option<crate::shim::Timestamp>,
}
/// MsgAnswerPoll represents the message used to answer a poll
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.MsgAnswerPoll")]
pub struct MsgAnswerPoll {
    /// Id of the subspace containing the post
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the post that contains the poll to be answered
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub post_id: u64,
    /// Id of the poll to be answered
    #[prost(uint32, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub poll_id: u32,
    /// Indexes of the answer inside the ProvidedAnswers array
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub answers_indexes: ::prost::alloc::vec::Vec<u32>,
    /// Address of the user answering the poll
    #[prost(string, tag = "5")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgAnswerPollResponse represents the MSg/AnswerPoll response type
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.posts.v2.MsgAnswerPollResponse")]
pub struct MsgAnswerPollResponse {}
pub struct PostsQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> PostsQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn subspace_posts(
        &self,
        subspace_id: u64,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QuerySubspacePostsResponse, cosmwasm_std::StdError> {
        QuerySubspacePostsRequest {
            subspace_id,
            pagination,
        }
        .query(self.querier)
    }
    pub fn section_posts(
        &self,
        subspace_id: u64,
        section_id: u32,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QuerySectionPostsResponse, cosmwasm_std::StdError> {
        QuerySectionPostsRequest {
            subspace_id,
            section_id,
            pagination,
        }
        .query(self.querier)
    }
    pub fn post(
        &self,
        subspace_id: u64,
        post_id: u64,
    ) -> Result<QueryPostResponse, cosmwasm_std::StdError> {
        QueryPostRequest {
            subspace_id,
            post_id,
        }
        .query(self.querier)
    }
    pub fn post_attachments(
        &self,
        subspace_id: u64,
        post_id: u64,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryPostAttachmentsResponse, cosmwasm_std::StdError> {
        QueryPostAttachmentsRequest {
            subspace_id,
            post_id,
            pagination,
        }
        .query(self.querier)
    }
    pub fn poll_answers(
        &self,
        subspace_id: u64,
        post_id: u64,
        poll_id: u32,
        user: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryPollAnswersResponse, cosmwasm_std::StdError> {
        QueryPollAnswersRequest {
            subspace_id,
            post_id,
            poll_id,
            user,
            pagination,
        }
        .query(self.querier)
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
}
