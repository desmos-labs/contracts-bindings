//! Implementations of [cosmwasm_std::CustomQuery] for [DesmosQuery].

use cosmwasm_std::CustomQuery;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg(feature = "profiles")]
use crate::profiles::query::ProfilesQuery;
#[cfg(feature = "relationships")]
use crate::relationships::query::RelationshipsQuery;
#[cfg(feature = "subspaces")]
use crate::subspaces::query::SubspacesQuery;

// Use the serde `rename_all` tag in order to produce the following json file structure
// ## Example
// {
//      "route": "profiles",
//      "query_data": {
//          "method": {}
//      }
// }
// Reference: https://serde.rs/enum-representations.html#adjacently-tagged

/// Enum that defines how the desmos query messages are serialized.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "route", content = "query_data")]
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
}

impl CustomQuery for DesmosQuery {}

#[cfg(feature = "profiles")]
impl From<ProfilesQuery> for DesmosQuery {
    fn from(query: ProfilesQuery) -> Self {
        Self::Profiles(query)
    }
}

#[cfg(feature = "subspaces")]
impl From<SubspacesQuery> for DesmosQuery {
    fn from(query: SubspacesQuery) -> Self {
        Self::Subspaces(query)
    }
}

#[cfg(feature = "relationships")]
impl From<RelationshipsQuery> for DesmosQuery {
    fn from(query: RelationshipsQuery) -> Self {
        Self::Relationships(query)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        profiles::query::ProfilesQuery, query::DesmosQuery,
        relationships::query::RelationshipsQuery, subspaces::query::SubspacesQuery,
    };
    use cosmwasm_std::{Addr, Uint64};

    #[test]
    fn test_from_profiles_query() {
        let query = ProfilesQuery::Profile {
            user: Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
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
}
