//! Implementations of [cosmwasm_std::CustomMsg] for [DesmosMsg].

#[cfg(feature = "posts")]
use crate::posts::msg::PostsMsg;
#[cfg(feature = "profiles")]
use crate::profiles::msg::ProfilesMsg;
#[cfg(feature = "relationships")]
use crate::relationships::msg::RelationshipsMsg;
#[cfg(feature = "subspaces")]
use crate::subspaces::msg::SubspacesMsg;
use cosmwasm_std::{CosmosMsg, CustomMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Use the serde `rename_all` tag in order to produce the following json file structure
// ## Example
// {
//      "route": "profiles",
//      "msg_data": {
//          "method": {}
//      }
// }
// Reference: https://serde.rs/enum-representations.html#adjacently-tagged

/// Enum that defines how the messages are serialized.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "route", content = "msg_data")]
pub enum DesmosMsg {
    /// Messages relative to the x/profiles module.
    #[cfg(feature = "profiles")]
    Profiles(ProfilesMsg),

    /// Messages relative to the x/subspaces module.
    #[cfg(feature = "subspaces")]
    Subspaces(SubspacesMsg),

    /// Messages relative to the x/relationships module.
    #[cfg(feature = "relationships")]
    Relationships(RelationshipsMsg),

    /// Messages relative to the x/posts module.
    #[cfg(feature = "posts")]
    Posts(PostsMsg),
}

impl Into<CosmosMsg<DesmosMsg>> for DesmosMsg {
    fn into(self) -> CosmosMsg<DesmosMsg> {
        CosmosMsg::Custom(self)
    }
}

impl CustomMsg for DesmosMsg {}

#[cfg(feature = "profiles")]
impl From<ProfilesMsg> for DesmosMsg {
    fn from(msg: ProfilesMsg) -> Self {
        Self::Profiles(msg)
    }
}

#[cfg(feature = "profiles")]
impl Into<CosmosMsg<DesmosMsg>> for ProfilesMsg {
    fn into(self) -> CosmosMsg<DesmosMsg> {
        DesmosMsg::from(self).into()
    }
}

#[cfg(feature = "subspaces")]
impl From<SubspacesMsg> for DesmosMsg {
    fn from(msg: SubspacesMsg) -> Self {
        Self::Subspaces(msg)
    }
}

#[cfg(feature = "subspaces")]
impl Into<CosmosMsg<DesmosMsg>> for SubspacesMsg {
    fn into(self) -> CosmosMsg<DesmosMsg> {
        DesmosMsg::from(self).into()
    }
}

#[cfg(feature = "relationships")]
impl From<RelationshipsMsg> for DesmosMsg {
    fn from(msg: RelationshipsMsg) -> Self {
        Self::Relationships(msg)
    }
}

#[cfg(feature = "relationships")]
impl Into<CosmosMsg<DesmosMsg>> for RelationshipsMsg {
    fn into(self) -> CosmosMsg<DesmosMsg> {
        DesmosMsg::from(self).into()
    }
}

#[cfg(feature = "posts")]
impl From<PostsMsg> for DesmosMsg {
    fn from(msg: PostsMsg) -> Self {
        Self::Posts(msg)
    }
}

#[cfg(feature = "posts")]
impl Into<CosmosMsg<DesmosMsg>> for PostsMsg {
    fn into(self) -> CosmosMsg<DesmosMsg> {
        DesmosMsg::from(self).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::posts::msg::PostsMsg;
    use crate::{
        msg::DesmosMsg, profiles::msg::ProfilesMsg, relationships::msg::RelationshipsMsg,
        subspaces::msg::SubspacesMsg,
    };
    use cosmwasm_std::{Addr, Uint64};

    #[test]
    fn test_from_profile_msg() {
        let msg = ProfilesMsg::RequestDtagTransfer {
            receiver: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            sender: Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
        };
        let expected = DesmosMsg::Profiles(msg.clone());
        assert_eq!(expected, DesmosMsg::from(msg))
    }

    #[test]
    fn test_from_relationships_msg() {
        let msg = RelationshipsMsg::CreateRelationship {
            signer: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            counterparty: Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
            subspace_id: Uint64::new(1),
        };
        let expected = DesmosMsg::Relationships(msg.clone());
        assert_eq!(expected, DesmosMsg::from(msg))
    }

    #[test]
    fn test_from_subspaces_msg() {
        let msg = SubspacesMsg::CreateSubspace {
            name: "test".to_string(),
            description: "test".to_string(),
            treasury: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            owner: Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
            creator: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        };
        let expected = DesmosMsg::Subspaces(msg.clone());
        assert_eq!(expected, DesmosMsg::from(msg));
    }

    #[test]
    fn test_from_posts_msg() {
        let msg = PostsMsg::DeletePost {
            subspace_id: Uint64::new(0),
            post_id: Uint64::new(1),
            signer: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        };
        let expected = DesmosMsg::Posts(msg.clone());
        assert_eq!(expected, DesmosMsg::from(msg));
    }
}
