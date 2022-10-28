//! Contains the messages that can be sent to the Desmos blockchain to interact with the x/profiles module.

use crate::profiles::proto::*;
use desmos_std::proto::ibc::core::client::v1::Height;
use crate::profiles::types::AddressData;
use cosmwasm_std::Addr;

/// Represents the messages to interact with the profiles module.
pub struct ProfilesMsgBuilder {}

impl ProfilesMsgBuilder {
    /// Creates an instance of [`ProfilesMsg::SaveProfile`].
    /// * `dtag` - Unique profile tag.
    /// * `creator` - Address of which is creating the profile.
    /// * `nickname` - Human readable name of the profile.
    /// * `bio` - Biography of the profile.
    /// * `profile_picture` - URL to the profile picture.
    /// * `cover_picture` - URL to the cover cover picture.
    pub fn save_profile(
        dtag: &str,
        creator: Addr,
        nickname: &str,
        bio: &str,
        profile_picture: &str,
        cover_picture: &str,
    ) -> MsgSaveProfile {
        MsgSaveProfile {
            dtag: dtag.into(),
            creator: creator.into(),
            nickname: nickname.into(),
            bio: bio.into(),
            profile_picture: profile_picture.into(),
            cover_picture: cover_picture.into(),
       }
    }

    /// Creates an instance of [`ProfilesMsg::DeleteProfile`].
    ///
    /// * `creator` - Address of the profile to delete.
    pub fn delete_profile(creator: Addr) -> MsgDeleteProfile {
        MsgDeleteProfile { creator: creator.into() }
    }

    /// Creates an instance of [`ProfilesMsg::RequestDtagTransfer`].
    ///
    /// * `sender` - Address of who is going to send the DTag.
    /// * `receiver` - Address of who is going to receive the DTag
    pub fn request_dtag_transfer(sender: Addr, receiver: Addr) -> MsgRequestDTagTransfer {
        MsgRequestDTagTransfer { receiver: receiver.into(), sender: sender.into() }
    }

    /// Creates an instance of [`ProfilesMsg::AcceptDtagTransferRequest`].
    ///
    /// * `new_dtag` - The DTag to accept.
    /// * `sender` - Address of who has sent the DTag.
    /// * `receiver` - Address of who is receiving the DTag.
    pub fn accept_dtag_transfer_request(
        new_dtag: &str,
        sender: Addr,
        receiver: Addr,
    ) -> MsgAcceptDTagTransferRequest {
        MsgAcceptDTagTransferRequest {
            new_dtag: new_dtag.into(),
            sender: sender.into(),
            receiver: receiver.into(),
        }
    }

    /// Creates an instance of [`ProfilesMsg::RefuseDtagTransferRequest`].
    ///
    /// * `sender` - Address of who has started the DTag transfer.
    /// * `receiver` - Address of who was supposed to receive the DTag.
    pub fn refuse_dtag_transfer_request(sender: Addr, receiver: Addr) -> MsgRefuseDTagTransferRequest {
        MsgRefuseDTagTransferRequest { sender: sender.into(), receiver: receiver.into() }
    }

    /// Creates an instance of [`ProfilesMsg::CancelDtagTransferRequest`].
    ///
    /// * `receiver` - Address of who was supposed to receive the DTag.
    /// * `sender` - Address of who has started the DTag transfer.
    pub fn cancel_dtag_transfer_request(receiver: Addr, sender: Addr) -> MsgCancelDTagTransferRequest {
        MsgCancelDTagTransferRequest { receiver: receiver.into(), sender: sender.into()}
    }

    /// Creates an instance of [`ProfilesMsg::LinkChainAccount`].
    ///
    /// * `chain_address` - Data of the external chain address to be connected
    /// with the Desmos profile.
    /// * `proof` - The ownership proof of the external chain address.
    /// * `chain_config` - The configuration of the external chain.
    /// * `signer` - Address associated with the profile to which link the external account.
    pub fn link_chain_account(
        chain_address: AddressData,
        proof: Proof,
        chain_config: ChainConfig,
        signer: Addr,
    ) -> MsgLinkChainAccount {
        MsgLinkChainAccount {
            chain_address: Some(chain_address.into()),
            proof: Some(proof),
            chain_config: Some(chain_config),
            signer: signer.into(),
        }
    }

    /// Creates an instance of [`ProfilesMsg::UnlinkChainAccount`].
    ///
    /// * `owner` - The profile address from which to remove the link.
    /// * `chain_name` - The chain name associated with the link to be removed.
    /// * `target` - The external address to be removed.
    pub fn unlink_chain_account(owner: Addr, chain_name: &str, target: &str) -> MsgUnlinkChainAccount {
        MsgUnlinkChainAccount {
            owner: owner.into(),
            chain_name: chain_name.into(),
            target: target.to_owned(),
        }
    }

    /// Creates an instance of [`ProfilesMsg::SetDefaultExternalAddress`].
    ///
    /// * `chain_name` - The chain name associated with the link to be set as default one.
    /// * `target` - The external address to be set as default one.
    /// * `signer` - The profile address which to set a default external address.
    pub fn set_default_external_address(
        chain_name: &str,
        target: &str,
        signer: Addr,
    ) -> MsgSetDefaultExternalAddress {
        MsgSetDefaultExternalAddress {
            chain_name: chain_name.to_owned(),
            target: target.to_owned(),
            signer: signer.into(),
        }
    }

    /// Creates an instance of [`ProfilesMsg::LinkApplication`].
    ///
    /// * `sender` - Sender of the connection request.
    /// * `link_data` - The data related to the application to which connect.
    /// * `call_data` - Hex encoded call data that will be sent to the data source in order to
    /// verify the link.
    /// * `source_port` - The port on which the packet will be sent.
    /// * `source_channel` - The channel by which the packet will be sent.
    /// * `timeout_height` - Timeout height relative to the current block height.
    /// The timeout is disabled when set to 0.
    /// * `timeout_timestamp` - Timeout timestamp (in nanoseconds) relative to the current block timestamp.
    /// The timeout is disabled when set to 0.
    pub fn link_application(
        sender: Addr,
        link_data: Data,
        call_data: String,
        source_port: String,
        source_channel: String,
        timeout_height: Height,
        timeout_timestamp: u64,
    ) -> MsgLinkApplication {
        MsgLinkApplication {
            sender: sender.into(),
            link_data: Some(link_data),
            call_data,
            source_port,
            source_channel,
            timeout_height: Some(timeout_height),
            timeout_timestamp: timeout_timestamp,
        }
    }

    /// Creates an instance of [`ProfilesMsg::UnlinkApplication`].
    ///
    /// * `application` - The name of the application to unlink.
    /// * `username` - The username inside the application to unlink.
    /// * `signer` - The Desmos account from which the application should be unlinked.
    pub fn unlink_application(application: &str, username: &str, signer: Addr) -> MsgUnlinkApplication {
        MsgUnlinkApplication {
            application: application.into(),
            username: username.into(),
            signer: signer.into(),
        }
    }
}