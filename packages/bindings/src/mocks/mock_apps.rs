//! Contains integration test utils for desmos custom modules.

#![cfg(not(tarpaulin_include))]
use crate::msg::DesmosMsg;
use crate::query::DesmosQuery;
use anyhow::Result as AnyResult;
use cosmwasm_std::testing::{MockApi, MockStorage};
use cosmwasm_std::{Addr, Api, Binary, BlockInfo, Empty, Event, Querier, Storage};
use cw_multi_test::{
    App, AppResponse, BankKeeper, BasicApp, BasicAppBuilder, CosmosRouter, FailingDistribution,
    FailingStaking, Module, Router, WasmKeeper,
};
use std::convert::TryFrom;

#[cfg(feature = "posts")]
use crate::posts::{mocks::mock_posts_query_response, msg::PostsMsg};
#[cfg(feature = "profiles")]
use crate::profiles::{mocks::mock_profiles_query_response, msg::ProfilesMsg};
#[cfg(feature = "reactions")]
use crate::reactions::{mocks::mock_reactions_query_response, msg::ReactionsMsg};
#[cfg(feature = "relationships")]
use crate::relationships::{mocks::mock_relationships_query_response, msg::RelationshipsMsg};
#[cfg(feature = "reports")]
use crate::reports::{mocks::mock_reports_query_response, models::ReportTarget, msg::ReportsMsg};
#[cfg(feature = "subspaces")]
use crate::subspaces::{mocks::mock_subspaces_query_response, msg::SubspacesMsg};

/// DesmosApp wraps the desmos custom module into a mock app for integration tests.
/// It always returns successful response with proper events.
pub type DesmosApp =
    App<BankKeeper, MockApi, MockStorage, DesmosKeeper, WasmKeeper<DesmosMsg, DesmosQuery>>;

/// Represents the implementation of [`Module`](cw_multi_test::Module) for handling the desmos execution and query messages.
#[derive(Default)]
pub struct DesmosKeeper {}

impl DesmosKeeper {
    /// Returns a new [DesmosKeeper].
    pub fn new() -> Self {
        DesmosKeeper {}
    }

    /// Handles [`ProfilesMsg`](crate::profiles::ProfilesMsg) then returns the response with proper [events](https://github.com/desmos-labs/desmos/blob/master/x/profiles/spec/05-events.md).
    #[cfg(feature = "profiles")]
    fn handle_profiles_msg(&self, block: &BlockInfo, msg: ProfilesMsg) -> AnyResult<AppResponse> {
        match msg {
            ProfilesMsg::SaveProfile { dtag, creator, .. } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("save_profile")
                        .add_attribute("profile_dtag", dtag)
                        .add_attribute("profile_creator", creator)
                        .add_attribute("profile_creation_time", block.time.to_string()),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            ProfilesMsg::DeleteProfile { creator, .. } => {
                let mut events = Vec::with_capacity(1);
                events.push(Event::new("delete_profile").add_attribute("profile_creator", creator));
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            ProfilesMsg::RequestDtagTransfer {
                sender: request_sender,
                receiver: request_receiver,
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("create_dtag_transfer_request")
                        .add_attribute("dtag_to_trade", "test")
                        .add_attribute("request_sender", request_sender)
                        .add_attribute("request_receiver", request_receiver),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            ProfilesMsg::AcceptDtagTransferRequest {
                new_dtag,
                sender: request_sender,
                receiver: request_receiver,
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("accept_dtag_transfer_request")
                        .add_attribute("dtag_to_trade", "test")
                        .add_attribute("new_dtag", new_dtag)
                        .add_attribute("request_sender", request_sender)
                        .add_attribute("request_receiver", request_receiver),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            ProfilesMsg::RefuseDtagTransferRequest {
                sender: request_sender,
                receiver: request_receiver,
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("refuse_dtag_transfer_request")
                        .add_attribute("request_sender", request_sender)
                        .add_attribute("request_receiver", request_receiver),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            ProfilesMsg::CancelDtagTransferRequest {
                sender: request_sender,
                receiver: request_receiver,
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("cancel_dtag_transfer_request")
                        .add_attribute("request_sender", request_sender)
                        .add_attribute("request_receiver", request_receiver),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            ProfilesMsg::LinkChainAccount {
                chain_address,
                chain_config,
                signer: owner,
                ..
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("link_chain_account")
                        .add_attribute("chain_link_account_target", chain_address.value)
                        .add_attribute("chain_link_source_chain_name", chain_config.name)
                        .add_attribute("chain_link_account_owner", owner)
                        .add_attribute("chain_link_creation_time", block.time.to_string()),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            ProfilesMsg::UnlinkChainAccount {
                owner,
                chain_name,
                target,
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("unlink_chain_account")
                        .add_attribute("chain_link_account_target", target)
                        .add_attribute("chain_link_source_chain_name", chain_name)
                        .add_attribute("chain_link_account_owner", owner),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            ProfilesMsg::SetDefaultExternalAddress {
                chain_name,
                target,
                signer,
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("set_default_external_address")
                        .add_attribute("chain_link_source_chain_name", chain_name)
                        .add_attribute("chain_link_account_target", target)
                        .add_attribute("chain_link_account_owner", signer),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            ProfilesMsg::LinkApplication {
                sender: user,
                link_data,
                ..
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("link_application")
                        .add_attribute("user", user)
                        .add_attribute("application_name", link_data.application)
                        .add_attribute("application_username", link_data.username)
                        .add_attribute("application_link_creation_time", block.time.to_string()),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            ProfilesMsg::UnlinkApplication {
                application,
                username,
                signer: user,
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("unlink_application")
                        .add_attribute("user", user)
                        .add_attribute("application_name", application)
                        .add_attribute("application_username", username),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
        }
    }

    /// Handles [`SubspacesMsg`](crate::subspaces::SubspacesMsg) then returns the response with proper [events](https://github.com/desmos-labs/desmos/blob/master/x/subspaces/spec/05-events.md).
    #[cfg(feature = "subspaces")]
    fn handle_subspaces_msg(&self, block: &BlockInfo, msg: SubspacesMsg) -> AnyResult<AppResponse> {
        match msg {
            SubspacesMsg::CreateSubspace { name, creator, .. } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("create_subspace")
                        .add_attribute("subspace_id", 1.to_string())
                        .add_attribute("subspace_name", name)
                        .add_attribute("subspace_creator", creator)
                        .add_attribute("creation_date", block.time.to_string()),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            SubspacesMsg::EditSubspace { subspace_id, .. } => {
                let mut events = Vec::with_capacity(1);

                events.push(Event::new("edit_subspace").add_attribute("subspace_id", subspace_id));
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            SubspacesMsg::DeleteSubspace { subspace_id, .. } => {
                let mut events = Vec::with_capacity(1);

                events
                    .push(Event::new("delete_subspace").add_attribute("subspace_id", subspace_id));
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            SubspacesMsg::CreateSection { subspace_id, .. } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("create_section")
                        .add_attribute("subspace_id", subspace_id)
                        .add_attribute("create_section", 1.to_string()),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            SubspacesMsg::EditSection {
                subspace_id,
                section_id,
                ..
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("edit_section")
                        .add_attribute("subspace_id", subspace_id)
                        .add_attribute("section_id", section_id.to_string()),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            SubspacesMsg::MoveSection {
                subspace_id,
                section_id,
                ..
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("move_section")
                        .add_attribute("subspace_id", subspace_id)
                        .add_attribute("section_id", section_id.to_string()),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            SubspacesMsg::DeleteSection {
                subspace_id,
                section_id,
                ..
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("delete_section")
                        .add_attribute("subspace_id", subspace_id)
                        .add_attribute("section_id", section_id.to_string()),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            SubspacesMsg::CreateUserGroup { subspace_id, .. } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("create_user_group")
                        .add_attribute("subspace_id", subspace_id)
                        .add_attribute("user_group_id", 1.to_string()),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            SubspacesMsg::EditUserGroup {
                subspace_id,
                group_id,
                ..
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("edit_user_group")
                        .add_attribute("subspace_id", subspace_id)
                        .add_attribute("user_group_id", group_id.to_string()),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            SubspacesMsg::MoveUserGroup {
                subspace_id,
                group_id,
                ..
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("move_user_group")
                        .add_attribute("subspace_id", subspace_id)
                        .add_attribute("user_group_id", group_id.to_string()),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            SubspacesMsg::SetUserGroupPermissions {
                subspace_id,
                group_id,
                ..
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("set_user_group_permissions")
                        .add_attribute("subspace_id", subspace_id)
                        .add_attribute("user_group_id", group_id.to_string()),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            SubspacesMsg::DeleteUserGroup {
                subspace_id,
                group_id,
                ..
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("delete_user_group")
                        .add_attribute("subspace_id", subspace_id)
                        .add_attribute("user_group_id", group_id.to_string()),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            SubspacesMsg::AddUserToUserGroup {
                subspace_id,
                group_id,
                user,
                ..
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("add_group_member")
                        .add_attribute("subspace_id", subspace_id)
                        .add_attribute("user_group_id", group_id.to_string())
                        .add_attribute("user", user),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            SubspacesMsg::RemoveUserFromUserGroup {
                subspace_id,
                group_id,
                user,
                ..
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("remove_group_member")
                        .add_attribute("subspace_id", subspace_id)
                        .add_attribute("user_group_id", group_id.to_string())
                        .add_attribute("user", user),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            SubspacesMsg::SetUserPermissions {
                subspace_id, user, ..
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("set_user_permissions")
                        .add_attribute("subspace_id", subspace_id)
                        .add_attribute("user", user),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
        }
    }

    /// Handles [`RelationshipsMsg`](crate::relationships::RelationshipsMsg) then returns the response with proper [events](https://github.com/desmos-labs/desmos/blob/master/x/relationships/spec/05-events.md).
    #[cfg(feature = "relationships")]
    fn handle_relationships_msg(&self, msg: RelationshipsMsg) -> AnyResult<AppResponse> {
        match msg {
            RelationshipsMsg::CreateRelationship {
                signer: creator,
                counterparty,
                subspace_id,
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("create_relationship")
                        .add_attribute("creator", creator)
                        .add_attribute("counterparty", counterparty)
                        .add_attribute("subspace", subspace_id),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            RelationshipsMsg::DeleteRelationship {
                signer: creator,
                counterparty,
                subspace_id,
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("delete_relationship")
                        .add_attribute("creator", creator)
                        .add_attribute("counterparty", counterparty)
                        .add_attribute("subspace", subspace_id),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            RelationshipsMsg::BlockUser {
                blocker,
                blocked,
                subspace_id,
                ..
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("block_user")
                        .add_attribute("blocker", blocker)
                        .add_attribute("blocked", blocked)
                        .add_attribute("subspace", subspace_id),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            RelationshipsMsg::UnblockUser {
                blocker,
                blocked,
                subspace_id,
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("create_dtag_transfer_request")
                        .add_attribute("blocker", blocker)
                        .add_attribute("blocked", blocked)
                        .add_attribute("subspace", subspace_id),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
        }
    }

    /// Handles [`PostsMsg`](crate::posts::PostsMsg) then returns the response with proper [events](https://github.com/desmos-labs/desmos/blob/master/x/posts/spec/05-events.md).
    #[cfg(feature = "posts")]
    fn handle_posts_msg(&self, block: &BlockInfo, msg: PostsMsg) -> AnyResult<AppResponse> {
        match msg {
            PostsMsg::CreatePost {
                subspace_id,
                section_id,
                author,
                ..
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("create_post")
                        .add_attribute("subspace_id", subspace_id)
                        .add_attribute("section_id", section_id.to_string())
                        .add_attribute("post_id", 1.to_string())
                        .add_attribute("author", author)
                        .add_attribute("creation_time", block.time.to_string()),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            PostsMsg::EditPost {
                subspace_id,
                post_id,
                ..
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("edit_post")
                        .add_attribute("subspace_id", subspace_id)
                        .add_attribute("post_id", post_id)
                        .add_attribute("last_edit_time", block.time.to_string()),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            PostsMsg::DeletePost {
                subspace_id,
                post_id,
                ..
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("delete_post")
                        .add_attribute("subspace_id", subspace_id)
                        .add_attribute("post_id", post_id),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            PostsMsg::AddPostAttachment {
                subspace_id,
                post_id,
                ..
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("add_post_attachment")
                        .add_attribute("subspace_id", subspace_id)
                        .add_attribute("post_id", post_id)
                        .add_attribute("attachment_id", 1.to_string())
                        .add_attribute("last_edit_time", block.time.to_string()),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            PostsMsg::RemovePostAttachment {
                subspace_id,
                post_id,
                attachment_id,
                ..
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("remove_post_attachment")
                        .add_attribute("subspace_id", subspace_id)
                        .add_attribute("post_id", post_id)
                        .add_attribute("attachment_id", attachment_id.to_string())
                        .add_attribute("last_edit_time", block.time.to_string()),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            PostsMsg::AnswerPoll {
                subspace_id,
                post_id,
                poll_id,
                ..
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("answer_poll")
                        .add_attribute("subspace_id", subspace_id)
                        .add_attribute("post_id", post_id)
                        .add_attribute("poll_id", poll_id.to_string()),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
        }
    }

    /// Handles [`ReportsMsg`](crate::reports::ReportsMsg) then returns the response with proper [events](https://github.com/desmos-labs/desmos/blob/master/x/reports/spec/05-events.md).
    #[cfg(feature = "reports")]
    fn handle_reports_msg(&self, block: &BlockInfo, msg: ReportsMsg) -> AnyResult<AppResponse> {
        match msg {
            ReportsMsg::CreateReport {
                subspace_id,
                reporter,
                target,
                ..
            } => {
                let mut events = Vec::with_capacity(2);
                events.push(
                    Event::new("create_report")
                        .add_attribute("subspace_id", subspace_id)
                        .add_attribute("report_id", 1.to_string())
                        .add_attribute("reporter", &reporter)
                        .add_attribute("creation_time", block.time.to_string()),
                );
                let report_target = ReportTarget::try_from(target)?;
                match report_target {
                    ReportTarget::Post { post_id } => {
                        events.push(
                            Event::new("report_post")
                                .add_attribute("subspace_id", subspace_id)
                                .add_attribute("post_id", post_id)
                                .add_attribute("reporter", reporter),
                        );
                    }
                    ReportTarget::User { user } => {
                        events.push(
                            Event::new("report_user")
                                .add_attribute("subspace_id", subspace_id)
                                .add_attribute("user", user)
                                .add_attribute("reporter", reporter),
                        );
                    }
                }
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            ReportsMsg::DeleteReport {
                subspace_id,
                report_id,
                ..
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("delete_report")
                        .add_attribute("subspace_id", subspace_id)
                        .add_attribute("report_id", report_id),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            ReportsMsg::SupportStandardReason {
                subspace_id,
                standard_reason_id,
                ..
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("support_standard_reason")
                        .add_attribute("subspace_id", subspace_id)
                        .add_attribute("standard_reason_id", standard_reason_id.to_string())
                        .add_attribute("reason_id", 1.to_string()),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            ReportsMsg::AddReason { subspace_id, .. } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("add_reason")
                        .add_attribute("subspace_id", subspace_id)
                        .add_attribute("reason_id", 1.to_string()),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            ReportsMsg::RemoveReason {
                subspace_id,
                reason_id,
                ..
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("create_post")
                        .add_attribute("subspace_id", subspace_id)
                        .add_attribute("reason_id", reason_id.to_string()),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
        }
    }

    /// Handles [`ReactionsMsg`](crate::reactions::ReactionsMsg) then returns the response with proper [events](https://github.com/desmos-labs/desmos/blob/master/x/reactions/spec/05-events.md).
    #[cfg(feature = "reactions")]
    fn handle_reactions_msg(&self, msg: ReactionsMsg) -> AnyResult<AppResponse> {
        match msg {
            ReactionsMsg::AddReaction {
                subspace_id,
                post_id,
                user,
                ..
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("add_reaction")
                        .add_attribute("subspace_id", subspace_id)
                        .add_attribute("post_id", post_id)
                        .add_attribute("reaction_id", 1.to_string())
                        .add_attribute("user", user),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            ReactionsMsg::RemoveReaction {
                subspace_id,
                post_id,
                reaction_id,
                ..
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("remove_reaction")
                        .add_attribute("subspace_id", subspace_id)
                        .add_attribute("post_id	", post_id)
                        .add_attribute("reaction_id", reaction_id.to_string()),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            ReactionsMsg::AddRegisteredReaction { subspace_id, .. } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("add_registered_reaction")
                        .add_attribute("subspace_id", subspace_id)
                        .add_attribute("registered_reaction_id", 1.to_string()),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            ReactionsMsg::EditRegisteredReaction {
                subspace_id,
                registered_reaction_id,
                ..
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("edit_registered_reaction")
                        .add_attribute("subspace_id", subspace_id)
                        .add_attribute(
                            "registered_reaction_id",
                            registered_reaction_id.to_string(),
                        ),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            ReactionsMsg::RemoveRegisteredReaction {
                subspace_id,
                registered_reaction_id,
                ..
            } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("remove_registered_reaction")
                        .add_attribute("subspace_id", subspace_id)
                        .add_attribute(
                            "registered_reaction_id",
                            registered_reaction_id.to_string(),
                        ),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
            ReactionsMsg::SetReactionsParams { subspace_id, .. } => {
                let mut events = Vec::with_capacity(1);
                events.push(
                    Event::new("set_reactions_params").add_attribute("subspace_id", subspace_id),
                );
                AnyResult::Ok(AppResponse {
                    events: events,
                    data: None,
                })
            }
        }
    }
}

impl Module for DesmosKeeper {
    type ExecT = DesmosMsg;
    type QueryT = DesmosQuery;
    type SudoT = Empty;

    fn execute<ExecC, QueryC>(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        block: &BlockInfo,
        _sender: Addr,
        msg: DesmosMsg,
    ) -> AnyResult<AppResponse> {
        match msg {
            #[cfg(feature = "profiles")]
            DesmosMsg::Profiles(msg) => self.handle_profiles_msg(block, msg),
            #[cfg(feature = "subspaces")]
            DesmosMsg::Subspaces(msg) => self.handle_subspaces_msg(block, msg),
            #[cfg(feature = "relationships")]
            DesmosMsg::Relationships(msg) => self.handle_relationships_msg(msg),
            #[cfg(feature = "posts")]
            DesmosMsg::Posts(msg) => self.handle_posts_msg(block, msg),
            #[cfg(feature = "reports")]
            DesmosMsg::Reports(msg) => self.handle_reports_msg(block, msg),
            #[cfg(feature = "reactions")]
            DesmosMsg::Reactions(msg) => self.handle_reactions_msg(msg),
        }
    }

    fn query(
        &self,
        _api: &dyn Api,
        _storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        request: DesmosQuery,
    ) -> AnyResult<Binary> {
        match request {
            #[cfg(feature = "profiles")]
            DesmosQuery::Profiles(query) => {
                AnyResult::Ok(mock_profiles_query_response(&query).unwrap())
            }
            #[cfg(feature = "subspaces")]
            DesmosQuery::Subspaces(query) => {
                AnyResult::Ok(mock_subspaces_query_response(&query).unwrap())
            }
            #[cfg(feature = "relationships")]
            DesmosQuery::Relationships(query) => {
                AnyResult::Ok(mock_relationships_query_response(&query).unwrap())
            }
            #[cfg(feature = "posts")]
            DesmosQuery::Posts(query) => AnyResult::Ok(mock_posts_query_response(&query).unwrap()),
            #[cfg(feature = "reactions")]
            DesmosQuery::Reactions(query) => {
                AnyResult::Ok(mock_reactions_query_response(&query).unwrap())
            }
            #[cfg(feature = "reports")]
            DesmosQuery::Reports(query) => {
                AnyResult::Ok(mock_reports_query_response(&query).unwrap())
            }
        }
    }

    fn sudo<ExecC, QueryC>(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &BlockInfo,
        _msg: Empty,
    ) -> AnyResult<AppResponse> {
        unimplemented!()
    }
}

/// Creates new default `DesmosApp` with customized initialization function.
pub fn custom_desmos_app<F>(init_fn: F) -> DesmosApp
where
    F: FnOnce(
        &mut Router<
            BankKeeper,
            DesmosKeeper,
            WasmKeeper<DesmosMsg, DesmosQuery>,
            FailingStaking,
            FailingDistribution,
        >,
        &dyn Api,
        &mut dyn Storage,
    ),
{
    BasicAppBuilder::<DesmosMsg, DesmosQuery>::new_custom()
        .with_custom(DesmosKeeper::new())
        .build(init_fn)
}

/// Represents failing desmos app always returning error.
pub type FailingDesmosApp = BasicApp<DesmosMsg, DesmosQuery>;

/// Returns a mock failing app.
pub fn mock_failing_desmos_app() -> FailingDesmosApp {
    BasicAppBuilder::<DesmosMsg, DesmosQuery>::new_custom().build(|_, _, _| {})
}

/// Returns a mock desmos app.
pub fn mock_desmos_app() -> DesmosApp {
    BasicAppBuilder::<DesmosMsg, DesmosQuery>::new_custom()
        .with_custom(DesmosKeeper::new())
        .build(|_, _, _| {})
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        posts::{
            mocks::MockPostsQueries, models_query::QueryPostResponse, msg::PostsMsg,
            querier::PostsQuerier,
        },
        profiles::{
            mocks::MockProfilesQueries, models_query::QueryProfileResponse, msg::ProfilesMsg,
            querier::ProfilesQuerier,
        },
        reactions::{
            mocks::MockReactionsQueries, models_query::QueryReactionsResponse, msg::ReactionsMsg,
            querier::ReactionsQuerier,
        },
        relationships::{
            mocks::MockRelationshipsQueries, models_query::QueryRelationshipsResponse,
            msg::RelationshipsMsg, querier::RelationshipsQuerier,
        },
        reports::{
            mocks::MockReportsQueries, models_query::QueryReportResponse, msg::ReportsMsg,
            querier::ReportsQuerier,
        },
        subspaces::{
            mocks::MockSubspacesQueries, models_query::QuerySubspaceResponse, msg::SubspacesMsg,
            querier::SubspacesQuerier,
        },
    };
    use cw_multi_test::Executor;
    use std::ops::Deref;
    const SENDER: &str = "sender";

    #[test]
    fn execute_profiles_msg_properly() {
        let mut app = mock_desmos_app();
        let result = app.execute(
            Addr::unchecked(SENDER),
            DesmosMsg::Profiles(ProfilesMsg::delete_profile(Addr::unchecked(SENDER))).into(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn execute_relationships_msg_properly() {
        let mut app = mock_desmos_app();
        let result = app.execute(
            Addr::unchecked(SENDER),
            DesmosMsg::Relationships(RelationshipsMsg::unblock_user(
                Addr::unchecked(SENDER),
                Addr::unchecked(SENDER),
                1,
            ))
            .into(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn execute_subspaces_msg_properly() {
        let mut app = mock_desmos_app();
        let result = app.execute(
            Addr::unchecked(SENDER),
            DesmosMsg::Subspaces(SubspacesMsg::delete_subspace(1, Addr::unchecked(SENDER))).into(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn execute_posts_msg_properly() {
        let mut app = mock_desmos_app();
        let result = app.execute(
            Addr::unchecked(SENDER),
            DesmosMsg::Posts(PostsMsg::delete_post(1, 4, Addr::unchecked(SENDER))).into(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn execute_reports_msg_properly() {
        let mut app = mock_desmos_app();
        let result = app.execute(
            Addr::unchecked(SENDER),
            DesmosMsg::Reports(ReportsMsg::delete_report(1, 1, Addr::unchecked(SENDER))).into(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn execute_reactions_msg_properly() {
        let mut app = mock_desmos_app();
        let result = app.execute(
            Addr::unchecked(SENDER),
            DesmosMsg::Reactions(ReactionsMsg::remove_reaction(
                1,
                1,
                1,
                Addr::unchecked(SENDER),
            ))
            .into(),
        );
        assert!(result.is_ok());
    }
    #[test]
    fn test_profiles_query_properly() {
        let app = mock_desmos_app();
        let app_querier = app.wrap();
        let querier = ProfilesQuerier::new(app_querier.deref());
        let expected = QueryProfileResponse {
            profile: MockProfilesQueries::get_mock_profile(),
        };
        let response = querier.query_profile(Addr::unchecked("")).unwrap();
        assert_eq!(expected, response)
    }

    #[test]
    fn test_subspaces_query_properly() {
        let app = mock_desmos_app();
        let app_querier = app.wrap();
        let querier = SubspacesQuerier::new(app_querier.deref());
        let response = querier.query_subspace(1).unwrap();
        let expected = QuerySubspaceResponse {
            subspace: MockSubspacesQueries::get_mock_subspace(),
        };
        assert_eq!(expected, response);
    }

    #[test]
    fn test_relationships_query_properly() {
        let app = mock_desmos_app();
        let app_querier = app.wrap();
        let querier = RelationshipsQuerier::new(app_querier.deref());
        let response = querier
            .query_relationships(
                1,
                Some(Addr::unchecked("")),
                Some(Addr::unchecked("")),
                None,
            )
            .unwrap();
        let expected = QueryRelationshipsResponse {
            relationships: vec![MockRelationshipsQueries::get_mock_relationship()],
            pagination: Default::default(),
        };
        assert_eq!(expected, response)
    }

    #[test]
    fn test_posts_query_properly() {
        let app = mock_desmos_app();
        let app_querier = app.wrap();
        let querier = PostsQuerier::new(app_querier.deref());
        let response = querier.query_post(1, 1).unwrap();
        let expected = QueryPostResponse {
            post: MockPostsQueries::get_mocked_post(1u64.into(), 1u64.into()),
        };
        assert_eq!(expected, response)
    }

    #[test]
    fn test_reactions_query_properly() {
        let app = mock_desmos_app();
        let app_querier = app.wrap();
        let querier = ReactionsQuerier::new(app_querier.deref());
        let response = querier.query_reactions(1, 1, None, None).unwrap();
        let expected = QueryReactionsResponse {
            reactions: vec![MockReactionsQueries::get_mock_reaction()],
            pagination: Default::default(),
        };
        assert_eq!(expected, response)
    }

    #[test]
    fn test_reports_query_properly() {
        let app = mock_desmos_app();
        let app_querier = app.wrap();
        let querier = ReportsQuerier::new(app_querier.deref());
        let response = querier.query_report(1, 1).unwrap();
        let expected = QueryReportResponse {
            report: MockReportsQueries::get_mocked_report(&1u64.into()),
        };
        assert_eq!(expected, response)
    }

    #[test]
    fn failing_app_excute_error() {
        let mut app = mock_failing_desmos_app();
        let result = app.execute(
            Addr::unchecked(SENDER),
            DesmosMsg::Profiles(ProfilesMsg::delete_profile(Addr::unchecked(SENDER))).into(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn failing_app_query_error() {
        let app = mock_failing_desmos_app();
        let app_querier = app.wrap();
        let querier = ProfilesQuerier::new(app_querier.deref());
        let result = querier.query_profile(Addr::unchecked(""));
        assert!(result.is_err())
    }
}
