//! Implementations of [cosmwasm_std::CustomQuery] for [DesmosQuery].

#[cfg(feature = "posts")]
use crate::posts::query::PostsQuery;
#[cfg(feature = "profiles")]
use crate::profiles::query::ProfilesQuery;
#[cfg(feature = "reactions")]
use crate::reactions::query::ReactionsQuery;
#[cfg(feature = "relationships")]
use crate::relationships::query::RelationshipsQuery;
#[cfg(feature = "reports")]
use crate::reports::query::ReportsQuery;
#[cfg(feature = "subspaces")]
use crate::subspaces::query::SubspacesQuery;
use cosmwasm_std::{CustomQuery, QueryRequest};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Use the serde `rename_all` tag in order to produce the following json file structure
// ## Example
// {
//      "profiles": {
//          "method": {}
//      }
// }

/// Enum that defines how the desmos query messages are serialized.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DesmosQuery {
    /// Queries relative to the x/profiles module.
    #[cfg(feature = "profiles")]
    Profiles(ProfilesQuery),

    /// Queries relative to the x/subspaces module.
    #[cfg(feature = "subspaces")]
    Subspaces(SubspacesQuery),

    /// Queries relative to the x/relationships module.
    #[cfg(feature = "relationships")]
    Relationships(RelationshipsQuery),

    /// Queries relative to the x/posts module.
    #[cfg(feature = "posts")]
    Posts(PostsQuery),

    /// Queries relative to the x/reactions module.
    #[cfg(feature = "reactions")]
    Reactions(ReactionsQuery),

    /// Queries relative to the x/reports module.
    #[cfg(feature = "reports")]
    Reports(ReportsQuery),
}

impl CustomQuery for DesmosQuery {}

#[cfg(feature = "profiles")]
impl From<ProfilesQuery> for DesmosQuery {
    fn from(query: ProfilesQuery) -> Self {
        Self::Profiles(query)
    }
}

#[cfg(feature = "profiles")]
impl Into<QueryRequest<DesmosQuery>> for ProfilesQuery {
    fn into(self) -> QueryRequest<DesmosQuery> {
        QueryRequest::Custom(self.into())
    }
}

#[cfg(feature = "subspaces")]
impl From<SubspacesQuery> for DesmosQuery {
    fn from(query: SubspacesQuery) -> Self {
        Self::Subspaces(query)
    }
}

#[cfg(feature = "subspaces")]
impl Into<QueryRequest<DesmosQuery>> for SubspacesQuery {
    fn into(self) -> QueryRequest<DesmosQuery> {
        QueryRequest::Custom(self.into())
    }
}

#[cfg(feature = "relationships")]
impl From<RelationshipsQuery> for DesmosQuery {
    fn from(query: RelationshipsQuery) -> Self {
        Self::Relationships(query)
    }
}

#[cfg(feature = "relationships")]
impl Into<QueryRequest<DesmosQuery>> for RelationshipsQuery {
    fn into(self) -> QueryRequest<DesmosQuery> {
        QueryRequest::Custom(self.into())
    }
}

#[cfg(feature = "posts")]
impl From<PostsQuery> for DesmosQuery {
    fn from(query: PostsQuery) -> Self {
        Self::Posts(query)
    }
}

#[cfg(feature = "posts")]
impl Into<QueryRequest<DesmosQuery>> for PostsQuery {
    fn into(self) -> QueryRequest<DesmosQuery> {
        QueryRequest::Custom(self.into())
    }
}

#[cfg(feature = "reactions")]
impl From<ReactionsQuery> for DesmosQuery {
    fn from(query: ReactionsQuery) -> Self {
        Self::Reactions(query)
    }
}

#[cfg(feature = "reactions")]
impl Into<QueryRequest<DesmosQuery>> for ReactionsQuery {
    fn into(self) -> QueryRequest<DesmosQuery> {
        QueryRequest::Custom(self.into())
    }
}

#[cfg(feature = "reports")]
impl From<ReportsQuery> for DesmosQuery {
    fn from(query: ReportsQuery) -> Self {
        Self::Reports(query)
    }
}

#[cfg(feature = "reports")]
impl Into<QueryRequest<DesmosQuery>> for ReportsQuery {
    fn into(self) -> QueryRequest<DesmosQuery> {
        QueryRequest::Custom(self.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::posts::query::PostsQuery;
    use crate::reports::query::ReportsQuery;
    use crate::{
        profiles::query::ProfilesQuery, query::DesmosQuery, reactions::query::ReactionsQuery,
        relationships::query::RelationshipsQuery, subspaces::query::SubspacesQuery,
    };
    use cosmwasm_std::{Addr, Uint64};

    #[test]
    fn test_from_profiles_query() {
        let query = ProfilesQuery::Profile {
            user: Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2").to_string(),
        };
        let expected = DesmosQuery::Profiles(query.clone());
        assert_eq!(expected, DesmosQuery::from(query));
    }

    #[test]
    fn test_from_subspaces_query() {
        let query = SubspacesQuery::Subspaces {
            pagination: Default::default(),
        };
        let expected = DesmosQuery::Subspaces(query.clone());
        assert_eq!(expected, DesmosQuery::from(query));
    }

    #[test]
    fn test_from_relationships_query() {
        let query = RelationshipsQuery::Relationships {
            user: Some(Addr::unchecked(
                "cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2",
            )),
            counterparty: Some(Addr::unchecked(
                "desmos1rfv0f7mx7w9d3jv3h803u38vqym9ygg344asm3",
            )),
            subspace_id: Uint64::new(1),
            pagination: None,
        };
        let expected = DesmosQuery::Relationships(query.clone());
        assert_eq!(expected, DesmosQuery::from(query))
    }

    #[test]
    fn test_from_posts_query() {
        let query = PostsQuery::Post {
            subspace_id: Uint64::new(0),
            post_id: Uint64::new(0),
        };
        let expected = DesmosQuery::Posts(query.clone());
        assert_eq!(expected, DesmosQuery::from(query));
    }

    #[test]
    fn test_from_reactions_query() {
        let query = ReactionsQuery::Reactions {
            subspace_id: Uint64::new(1),
            post_id: Uint64::new(1),
            user: None,
            pagination: None,
        };
        let expected = DesmosQuery::Reactions(query.clone());
        assert_eq!(expected, DesmosQuery::from(query))
    }

    #[test]
    fn test_from_reports_query() {
        let query = ReportsQuery::Report {
            subspace_id: Uint64::new(1),
            report_id: Uint64::new(2),
        };
        let expected = DesmosQuery::Reports(query.clone());
        assert_eq!(expected, DesmosQuery::from(query))
    }
}
