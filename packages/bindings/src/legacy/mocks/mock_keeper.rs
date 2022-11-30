//! Contains integration test utils to mock the responses that a contract receive
//! when performing integration tests.

#![cfg(not(tarpaulin_include))]
use crate::legacy::mocks::mock_queriers::MockDesmosQuerier;
use crate::legacy::msg::DesmosMsg;
#[cfg(feature = "posts")]
use crate::legacy::posts::msg::PostsMsg;
#[cfg(feature = "profiles")]
use crate::legacy::profiles::msg::ProfilesMsg;
use crate::legacy::query::DesmosQuery;
#[cfg(feature = "reactions")]
use crate::legacy::reactions::msg::ReactionsMsg;
#[cfg(feature = "relationships")]
use crate::legacy::relationships::msg::RelationshipsMsg;
#[cfg(feature = "reports")]
use crate::legacy::reports::{models::ReportTarget, msg::ReportsMsg};
#[cfg(feature = "subspaces")]
use crate::legacy::subspaces::msg::SubspacesMsg;
use anyhow::Result as AnyResult;
use cosmwasm_std::{
    Addr, Api, Binary, BlockInfo, ContractResult, Empty, Event, Querier, QueryRequest, Storage,
};
use cw_multi_test::{AppResponse, CosmosRouter, Module};
use std::convert::TryFrom;

/// Represents the implementation of [`Module`](cw_multi_test::Module) for handling the desmos execution and query messages.
#[derive(Default)]
pub struct DesmosKeeper {
    /// Querier used to handle the query requests.
    pub querier: MockDesmosQuerier,
}

impl DesmosKeeper {
    /// Returns a new [DesmosKeeper].
    pub fn new() -> Self {
        DesmosKeeper {
            querier: MockDesmosQuerier::new(&[]),
        }
    }

    /// Returns a new [DesmosKeeper] with a custom instance of [MockDesmosQuerier].
    pub fn with_custom_querier(querier: MockDesmosQuerier) -> Self {
        DesmosKeeper { querier }
    }

    /// Handles [`ProfilesMsg`](crate::legacy::profiles::msg::ProfilesMsg) then returns the response with proper [events](https://github.com/desmos-labs/desmos/blob/master/x/profiles/spec/05-events.md).
    #[cfg(feature = "profiles")]
    pub fn handle_profiles_msg(block: &BlockInfo, msg: ProfilesMsg) -> AnyResult<AppResponse> {
        match msg {
            ProfilesMsg::SaveProfile { dtag, creator, .. } => {
                let events = vec![Event::new("save_profile")
                    .add_attribute("profile_dtag", dtag)
                    .add_attribute("profile_creator", creator)
                    .add_attribute("profile_creation_time", block.time.to_string())];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            ProfilesMsg::DeleteProfile { creator, .. } => {
                let events =
                    vec![Event::new("delete_profile").add_attribute("profile_creator", creator)];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            ProfilesMsg::RequestDtagTransfer {
                sender: request_sender,
                receiver: request_receiver,
            } => {
                let events = vec![Event::new("create_dtag_transfer_request")
                    .add_attribute("dtag_to_trade", "test")
                    .add_attribute("request_sender", request_sender)
                    .add_attribute("request_receiver", request_receiver)];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            ProfilesMsg::AcceptDtagTransferRequest {
                new_dtag,
                sender: request_sender,
                receiver: request_receiver,
            } => {
                let events = vec![Event::new("accept_dtag_transfer_request")
                    .add_attribute("dtag_to_trade", "test")
                    .add_attribute("new_dtag", new_dtag)
                    .add_attribute("request_sender", request_sender)
                    .add_attribute("request_receiver", request_receiver)];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            ProfilesMsg::RefuseDtagTransferRequest {
                sender: request_sender,
                receiver: request_receiver,
            } => {
                let events = vec![Event::new("refuse_dtag_transfer_request")
                    .add_attribute("request_sender", request_sender)
                    .add_attribute("request_receiver", request_receiver)];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            ProfilesMsg::CancelDtagTransferRequest {
                sender: request_sender,
                receiver: request_receiver,
            } => {
                let events = vec![Event::new("cancel_dtag_transfer_request")
                    .add_attribute("request_sender", request_sender)
                    .add_attribute("request_receiver", request_receiver)];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            ProfilesMsg::LinkChainAccount {
                chain_address,
                chain_config,
                signer: owner,
                ..
            } => {
                let events = vec![Event::new("link_chain_account")
                    .add_attribute("chain_link_account_target", chain_address.value)
                    .add_attribute("chain_link_source_chain_name", chain_config.name)
                    .add_attribute("chain_link_account_owner", owner)
                    .add_attribute("chain_link_creation_time", block.time.to_string())];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            ProfilesMsg::UnlinkChainAccount {
                owner,
                chain_name,
                target,
            } => {
                let events = vec![Event::new("unlink_chain_account")
                    .add_attribute("chain_link_account_target", target)
                    .add_attribute("chain_link_source_chain_name", chain_name)
                    .add_attribute("chain_link_account_owner", owner)];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            ProfilesMsg::SetDefaultExternalAddress {
                chain_name,
                target,
                signer,
            } => {
                let events = vec![Event::new("set_default_external_address")
                    .add_attribute("chain_link_source_chain_name", chain_name)
                    .add_attribute("chain_link_account_target", target)
                    .add_attribute("chain_link_account_owner", signer)];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            ProfilesMsg::LinkApplication {
                sender: user,
                link_data,
                ..
            } => {
                let events = vec![Event::new("link_application")
                    .add_attribute("user", user)
                    .add_attribute("application_name", link_data.application)
                    .add_attribute("application_username", link_data.username)
                    .add_attribute("application_link_creation_time", block.time.to_string())];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            ProfilesMsg::UnlinkApplication {
                application,
                username,
                signer: user,
            } => {
                let events = vec![Event::new("unlink_application")
                    .add_attribute("user", user)
                    .add_attribute("application_name", application)
                    .add_attribute("application_username", username)];
                AnyResult::Ok(AppResponse { events, data: None })
            }
        }
    }

    /// Handles [`SubspacesMsg`](crate::legacy::subspaces::msg::SubspacesMsg) then returns the response with proper [events](https://github.com/desmos-labs/desmos/blob/master/x/subspaces/spec/05-events.md).
    #[cfg(feature = "subspaces")]
    pub fn handle_subspaces_msg(block: &BlockInfo, msg: SubspacesMsg) -> AnyResult<AppResponse> {
        match msg {
            SubspacesMsg::CreateSubspace { name, creator, .. } => {
                let events = vec![Event::new("create_subspace")
                    .add_attribute("subspace_id", 1.to_string())
                    .add_attribute("subspace_name", name)
                    .add_attribute("subspace_creator", creator)
                    .add_attribute("creation_date", block.time.to_string())];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            SubspacesMsg::EditSubspace { subspace_id, .. } => {
                let events =
                    vec![Event::new("edit_subspace").add_attribute("subspace_id", subspace_id)];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            SubspacesMsg::DeleteSubspace { subspace_id, .. } => {
                let events =
                    vec![Event::new("delete_subspace").add_attribute("subspace_id", subspace_id)];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            SubspacesMsg::CreateSection { subspace_id, .. } => {
                let events = vec![Event::new("create_section")
                    .add_attribute("subspace_id", subspace_id)
                    .add_attribute("create_section", 1.to_string())];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            SubspacesMsg::EditSection {
                subspace_id,
                section_id,
                ..
            } => {
                let events = vec![Event::new("edit_section")
                    .add_attribute("subspace_id", subspace_id)
                    .add_attribute("section_id", section_id.to_string())];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            SubspacesMsg::MoveSection {
                subspace_id,
                section_id,
                ..
            } => {
                let events = vec![Event::new("move_section")
                    .add_attribute("subspace_id", subspace_id)
                    .add_attribute("section_id", section_id.to_string())];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            SubspacesMsg::DeleteSection {
                subspace_id,
                section_id,
                ..
            } => {
                let events = vec![Event::new("delete_section")
                    .add_attribute("subspace_id", subspace_id)
                    .add_attribute("section_id", section_id.to_string())];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            SubspacesMsg::CreateUserGroup { subspace_id, .. } => {
                let events = vec![Event::new("create_user_group")
                    .add_attribute("subspace_id", subspace_id)
                    .add_attribute("user_group_id", 1.to_string())];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            SubspacesMsg::EditUserGroup {
                subspace_id,
                group_id,
                ..
            } => {
                let events = vec![Event::new("edit_user_group")
                    .add_attribute("subspace_id", subspace_id)
                    .add_attribute("user_group_id", group_id.to_string())];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            SubspacesMsg::MoveUserGroup {
                subspace_id,
                group_id,
                ..
            } => {
                let events = vec![Event::new("move_user_group")
                    .add_attribute("subspace_id", subspace_id)
                    .add_attribute("user_group_id", group_id.to_string())];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            SubspacesMsg::SetUserGroupPermissions {
                subspace_id,
                group_id,
                ..
            } => {
                let events = vec![Event::new("set_user_group_permissions")
                    .add_attribute("subspace_id", subspace_id)
                    .add_attribute("user_group_id", group_id.to_string())];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            SubspacesMsg::DeleteUserGroup {
                subspace_id,
                group_id,
                ..
            } => {
                let events = vec![Event::new("delete_user_group")
                    .add_attribute("subspace_id", subspace_id)
                    .add_attribute("user_group_id", group_id.to_string())];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            SubspacesMsg::AddUserToUserGroup {
                subspace_id,
                group_id,
                user,
                ..
            } => {
                let events = vec![Event::new("add_group_member")
                    .add_attribute("subspace_id", subspace_id)
                    .add_attribute("user_group_id", group_id.to_string())
                    .add_attribute("user", user)];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            SubspacesMsg::RemoveUserFromUserGroup {
                subspace_id,
                group_id,
                user,
                ..
            } => {
                let events = vec![Event::new("remove_group_member")
                    .add_attribute("subspace_id", subspace_id)
                    .add_attribute("user_group_id", group_id.to_string())
                    .add_attribute("user", user)];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            SubspacesMsg::SetUserPermissions {
                subspace_id, user, ..
            } => {
                let events = vec![Event::new("set_user_permissions")
                    .add_attribute("subspace_id", subspace_id)
                    .add_attribute("user", user)];
                AnyResult::Ok(AppResponse { events, data: None })
            }
        }
    }

    /// Handles [`RelationshipsMsg`](crate::legacy::relationships::msg::RelationshipsMsg) then returns the response with proper [events](https://github.com/desmos-labs/desmos/blob/master/x/relationships/spec/05-events.md).
    #[cfg(feature = "relationships")]
    pub fn handle_relationships_msg(
        _block: &BlockInfo,
        msg: RelationshipsMsg,
    ) -> AnyResult<AppResponse> {
        match msg {
            RelationshipsMsg::CreateRelationship {
                signer: creator,
                counterparty,
                subspace_id,
            } => {
                let events = vec![Event::new("create_relationship")
                    .add_attribute("creator", creator)
                    .add_attribute("counterparty", counterparty)
                    .add_attribute("subspace", subspace_id)];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            RelationshipsMsg::DeleteRelationship {
                signer: creator,
                counterparty,
                subspace_id,
            } => {
                let events = vec![Event::new("delete_relationship")
                    .add_attribute("creator", creator)
                    .add_attribute("counterparty", counterparty)
                    .add_attribute("subspace", subspace_id)];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            RelationshipsMsg::BlockUser {
                blocker,
                blocked,
                subspace_id,
                ..
            } => {
                let events = vec![Event::new("block_user")
                    .add_attribute("blocker", blocker)
                    .add_attribute("blocked", blocked)
                    .add_attribute("subspace", subspace_id)];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            RelationshipsMsg::UnblockUser {
                blocker,
                blocked,
                subspace_id,
            } => {
                let events = vec![Event::new("create_dtag_transfer_request")
                    .add_attribute("blocker", blocker)
                    .add_attribute("blocked", blocked)
                    .add_attribute("subspace", subspace_id)];
                AnyResult::Ok(AppResponse { events, data: None })
            }
        }
    }

    /// Handles [`PostsMsg`](crate::legacy::posts::msg::PostsMsg) then returns the response with proper [events](https://github.com/desmos-labs/desmos/blob/master/x/posts/spec/05-events.md).
    #[cfg(feature = "posts")]
    pub fn handle_posts_msg(block: &BlockInfo, msg: PostsMsg) -> AnyResult<AppResponse> {
        match msg {
            PostsMsg::CreatePost {
                subspace_id,
                section_id,
                author,
                ..
            } => {
                let events = vec![Event::new("create_post")
                    .add_attribute("subspace_id", subspace_id)
                    .add_attribute("section_id", section_id.to_string())
                    .add_attribute("post_id", 1.to_string())
                    .add_attribute("author", author)
                    .add_attribute("creation_time", block.time.to_string())];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            PostsMsg::EditPost {
                subspace_id,
                post_id,
                ..
            } => {
                let events = vec![Event::new("edit_post")
                    .add_attribute("subspace_id", subspace_id)
                    .add_attribute("post_id", post_id)
                    .add_attribute("last_edit_time", block.time.to_string())];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            PostsMsg::DeletePost {
                subspace_id,
                post_id,
                ..
            } => {
                let events = vec![Event::new("delete_post")
                    .add_attribute("subspace_id", subspace_id)
                    .add_attribute("post_id", post_id)];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            PostsMsg::AddPostAttachment {
                subspace_id,
                post_id,
                ..
            } => {
                let events = vec![Event::new("add_post_attachment")
                    .add_attribute("subspace_id", subspace_id)
                    .add_attribute("post_id", post_id)
                    .add_attribute("attachment_id", 1.to_string())
                    .add_attribute("last_edit_time", block.time.to_string())];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            PostsMsg::RemovePostAttachment {
                subspace_id,
                post_id,
                attachment_id,
                ..
            } => {
                let events = vec![Event::new("remove_post_attachment")
                    .add_attribute("subspace_id", subspace_id)
                    .add_attribute("post_id", post_id)
                    .add_attribute("attachment_id", attachment_id.to_string())
                    .add_attribute("last_edit_time", block.time.to_string())];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            PostsMsg::AnswerPoll {
                subspace_id,
                post_id,
                poll_id,
                ..
            } => {
                let events = vec![Event::new("answer_poll")
                    .add_attribute("subspace_id", subspace_id)
                    .add_attribute("post_id", post_id)
                    .add_attribute("poll_id", poll_id.to_string())];
                AnyResult::Ok(AppResponse { events, data: None })
            }
        }
    }

    /// Handles [`ReportsMsg`](crate::legacy::reports::msg::ReportsMsg) then returns the response with proper [events](https://github.com/desmos-labs/desmos/blob/master/x/reports/spec/05-events.md).
    #[cfg(feature = "reports")]
    pub fn handle_reports_msg(block: &BlockInfo, msg: ReportsMsg) -> AnyResult<AppResponse> {
        match msg {
            ReportsMsg::CreateReport {
                subspace_id,
                reporter,
                target,
                ..
            } => {
                let mut events = vec![Event::new("create_report")
                    .add_attribute("subspace_id", subspace_id)
                    .add_attribute("report_id", 1.to_string())
                    .add_attribute("reporter", &reporter)
                    .add_attribute("creation_time", block.time.to_string())];
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
                AnyResult::Ok(AppResponse { events, data: None })
            }
            ReportsMsg::DeleteReport {
                subspace_id,
                report_id,
                ..
            } => {
                let events = vec![Event::new("delete_report")
                    .add_attribute("subspace_id", subspace_id)
                    .add_attribute("report_id", report_id)];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            ReportsMsg::SupportStandardReason {
                subspace_id,
                standard_reason_id,
                ..
            } => {
                let events = vec![Event::new("support_standard_reason")
                    .add_attribute("subspace_id", subspace_id)
                    .add_attribute("standard_reason_id", standard_reason_id.to_string())
                    .add_attribute("reason_id", 1.to_string())];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            ReportsMsg::AddReason { subspace_id, .. } => {
                let events = vec![Event::new("add_reason")
                    .add_attribute("subspace_id", subspace_id)
                    .add_attribute("reason_id", 1.to_string())];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            ReportsMsg::RemoveReason {
                subspace_id,
                reason_id,
                ..
            } => {
                let events = vec![Event::new("create_post")
                    .add_attribute("subspace_id", subspace_id)
                    .add_attribute("reason_id", reason_id.to_string())];
                AnyResult::Ok(AppResponse { events, data: None })
            }
        }
    }

    /// Handles [`ReactionsMsg`](crate::legacy::reactions::msg::ReactionsMsg) then returns the response with proper [events](https://github.com/desmos-labs/desmos/blob/master/x/reactions/spec/05-events.md).
    #[cfg(feature = "reactions")]
    pub fn handle_reactions_msg(_block: &BlockInfo, msg: ReactionsMsg) -> AnyResult<AppResponse> {
        match msg {
            ReactionsMsg::AddReaction {
                subspace_id,
                post_id,
                user,
                ..
            } => {
                let events = vec![Event::new("add_reaction")
                    .add_attribute("subspace_id", subspace_id)
                    .add_attribute("post_id", post_id)
                    .add_attribute("reaction_id", 1.to_string())
                    .add_attribute("user", user)];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            ReactionsMsg::RemoveReaction {
                subspace_id,
                post_id,
                reaction_id,
                ..
            } => {
                let events = vec![Event::new("remove_reaction")
                    .add_attribute("subspace_id", subspace_id)
                    .add_attribute("post_id	", post_id)
                    .add_attribute("reaction_id", reaction_id.to_string())];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            ReactionsMsg::AddRegisteredReaction { subspace_id, .. } => {
                let events = vec![Event::new("add_registered_reaction")
                    .add_attribute("subspace_id", subspace_id)
                    .add_attribute("registered_reaction_id", 1.to_string())];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            ReactionsMsg::EditRegisteredReaction {
                subspace_id,
                registered_reaction_id,
                ..
            } => {
                let events = vec![Event::new("edit_registered_reaction")
                    .add_attribute("subspace_id", subspace_id)
                    .add_attribute("registered_reaction_id", registered_reaction_id.to_string())];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            ReactionsMsg::RemoveRegisteredReaction {
                subspace_id,
                registered_reaction_id,
                ..
            } => {
                let events = vec![Event::new("remove_registered_reaction")
                    .add_attribute("subspace_id", subspace_id)
                    .add_attribute("registered_reaction_id", registered_reaction_id.to_string())];
                AnyResult::Ok(AppResponse { events, data: None })
            }
            ReactionsMsg::SetReactionsParams { subspace_id, .. } => {
                let events =
                    vec![Event::new("set_reactions_params")
                        .add_attribute("subspace_id", subspace_id)];
                AnyResult::Ok(AppResponse { events, data: None })
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
            DesmosMsg::Profiles(msg) => DesmosKeeper::handle_profiles_msg(block, msg),
            #[cfg(feature = "subspaces")]
            DesmosMsg::Subspaces(msg) => DesmosKeeper::handle_subspaces_msg(block, msg),
            #[cfg(feature = "relationships")]
            DesmosMsg::Relationships(msg) => DesmosKeeper::handle_relationships_msg(block, msg),
            #[cfg(feature = "posts")]
            DesmosMsg::Posts(msg) => DesmosKeeper::handle_posts_msg(block, msg),
            #[cfg(feature = "reports")]
            DesmosMsg::Reports(msg) => DesmosKeeper::handle_reports_msg(block, msg),
            #[cfg(feature = "reactions")]
            DesmosMsg::Reactions(msg) => DesmosKeeper::handle_reactions_msg(block, msg),
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

    fn query(
        &self,
        _api: &dyn Api,
        _storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        request: DesmosQuery,
    ) -> AnyResult<Binary> {
        let request = QueryRequest::Custom(request);
        let result = self.querier.handle_query(&request).into_result();

        if let Result::Err(error) = result {
            AnyResult::Err(error.into())
        } else {
            let contract_result = result.unwrap();
            match contract_result {
                ContractResult::Ok(binary) => AnyResult::Ok(binary),
                ContractResult::Err(err) => AnyResult::Err(anyhow::Error::msg(err)),
            }
        }
    }
}
