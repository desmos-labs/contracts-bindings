//! Contains the messages that can be sent to the Desmos blockchain to interact with the x/profiles module.

use crate::stargate::profiles::proto::*;
use crate::stargate::profiles::types::AddressData;
use crate::stargate::types::Height;
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
        MsgDeleteProfile {
            creator: creator.into(),
        }
    }

    /// Creates an instance of [`ProfilesMsg::RequestDtagTransfer`].
    ///
    /// * `sender` - Address of who is going to send the DTag.
    /// * `receiver` - Address of who is going to receive the DTag
    pub fn request_dtag_transfer(sender: Addr, receiver: Addr) -> MsgRequestDTagTransfer {
        MsgRequestDTagTransfer {
            receiver: receiver.into(),
            sender: sender.into(),
        }
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
    pub fn refuse_dtag_transfer_request(
        sender: Addr,
        receiver: Addr,
    ) -> MsgRefuseDTagTransferRequest {
        MsgRefuseDTagTransferRequest {
            sender: sender.into(),
            receiver: receiver.into(),
        }
    }

    /// Creates an instance of [`ProfilesMsg::CancelDtagTransferRequest`].
    ///
    /// * `receiver` - Address of who was supposed to receive the DTag.
    /// * `sender` - Address of who has started the DTag transfer.
    pub fn cancel_dtag_transfer_request(
        receiver: Addr,
        sender: Addr,
    ) -> MsgCancelDTagTransferRequest {
        MsgCancelDTagTransferRequest {
            receiver: receiver.into(),
            sender: sender.into(),
        }
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
    pub fn unlink_chain_account(
        owner: Addr,
        chain_name: &str,
        target: &str,
    ) -> MsgUnlinkChainAccount {
        MsgUnlinkChainAccount {
            owner: owner.into(),
            chain_name: chain_name.into(),
            target: target.into(),
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
            chain_name: chain_name.into(),
            target: target.into(),
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
    pub fn unlink_application(
        application: &str,
        username: &str,
        signer: Addr,
    ) -> MsgUnlinkApplication {
        MsgUnlinkApplication {
            application: application.into(),
            username: username.into(),
            signer: signer.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::Binary;
    use desmos_std::public_keys::Secp256k1PublicKey;

    #[test]
    fn test_save_profile() {
        let msg = ProfilesMsgBuilder::save_profile("test", Addr::unchecked("user"), "", "", "", "");
        let expected = MsgSaveProfile {
            dtag: "test".into(),
            nickname: "".into(),
            bio: "".into(),
            profile_picture: "".into(),
            cover_picture: "".into(),
            creator: "user".into(),
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_delete_profile() {
        let msg = ProfilesMsgBuilder::delete_profile(Addr::unchecked("user"));
        let expected = MsgDeleteProfile {
            creator: "user".into(),
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_request_dtag_transfer() {
        let msg = ProfilesMsgBuilder::request_dtag_transfer(
            Addr::unchecked("user"),
            Addr::unchecked("reciever"),
        );
        let expected = MsgRequestDTagTransfer {
            sender: "user".into(),
            receiver: "reciever".into(),
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_accept_dtag_transfer_request() {
        let msg = ProfilesMsgBuilder::accept_dtag_transfer_request(
            "test",
            Addr::unchecked("reciever"),
            Addr::unchecked("user"),
        );
        let expected = MsgAcceptDTagTransferRequest {
            new_dtag: "test".into(),
            sender: "reciever".into(),
            receiver: "user".into(),
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_refuse_dtag_transfer_request() {
        let msg = ProfilesMsgBuilder::refuse_dtag_transfer_request(
            Addr::unchecked("reciever"),
            Addr::unchecked("user"),
        );
        let expected = MsgRefuseDTagTransferRequest {
            sender: "reciever".into(),
            receiver: "user".into(),
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_cancel_dtag_transfer_request() {
        let msg = ProfilesMsgBuilder::cancel_dtag_transfer_request(
            Addr::unchecked("reciever"),
            Addr::unchecked("user"),
        );
        let expected = MsgCancelDTagTransferRequest {
            receiver: "reciever".into(),
            sender: "user".into(),
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_link_chain_account() {
        let chain_addr = AddressData::Bech32Address(Bech32Address {
            value: "cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2".to_string(),
            prefix: "cosmos".to_string(),
        });

        let proof = Proof {
            pub_key: Some(Secp256k1PublicKey{
                key: Binary::from_base64("ArlRm0a5fFTHFfKha1LpDd+g3kZlyRBBF4R8PSM8Zo4Y").unwrap().to_vec(),
            }.into()),
            signature: Some(SingleSignature{
                value_type: SignatureValueType::Raw.into(),
                signature: Binary::from_base64("C7xppu4C4S3dgeC9TVqhyGN1hbMnMbnmWgXQI2WE8t0oHIHhDTqXyZgzhNNYiBO7ulno3G8EXO3Ep5KMFngyFg").unwrap().to_vec(),
            }.into()),
            plain_text: "636f736d6f733138786e6d6c7a71727172367a74353236706e637a786536357a6b33663478676d6e6470786e32".to_string(),
        };

        let chain_config = ChainConfig {
            name: "cosmos".to_string(),
        };

        let msg = ProfilesMsgBuilder::link_chain_account(
            chain_addr.clone(),
            proof.clone(),
            chain_config.clone(),
            Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
        );

        let expected = MsgLinkChainAccount {
            chain_address: Some(chain_addr.clone().into()),
            proof: Some(proof),
            chain_config: Some(chain_config),
            signer: "cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2".into(),
        };

        assert_eq!(expected, msg)
    }

    #[test]
    fn test_unlink_chain_account() {
        let msg =
            ProfilesMsgBuilder::unlink_chain_account(Addr::unchecked("owner"), "cosmos", "target");
        let expected = MsgUnlinkChainAccount {
            owner: "owner".into(),
            chain_name: "cosmos".into(),
            target: "target".into(),
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_set_default_external_address() {
        let msg = ProfilesMsgBuilder::set_default_external_address(
            "cosmos",
            "target",
            Addr::unchecked("owner"),
        );
        let expected = MsgSetDefaultExternalAddress {
            chain_name: "cosmos".into(),
            target: "target".into(),
            signer: "owner".into(),
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_link_application() {
        let data = Data {
            application: "twitter".into(),
            username: "goldrake".into(),
        };

        let oracle_req = OracleRequest {
            id: 537807,
            oracle_script_id: 32,
            call_data: Some(oracle_request::CallData {
                application: "twitter".into(),
                call_data: "7b22757365726e616d65223a224c756361675f5f2335323337227d".into(),
            }),
            client_id: "desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc-twitter-goldrake".into(),
        };

        let timeout_height = Height {
            revision_number: 0,
            revision_height: 0,
        };

        let msg = ProfilesMsgBuilder::link_application(
            Addr::unchecked("owner"),
            data.clone(),
            oracle_req.call_data.clone().unwrap().call_data,
            "123".into(),
            "123".into(),
            timeout_height.clone(),
            1,
        );

        let expected = MsgLinkApplication {
            sender: "owner".into(),
            link_data: Some(data),
            call_data: oracle_req.call_data.unwrap().call_data,
            source_port: "123".into(),
            source_channel: "123".into(),
            timeout_height: Some(timeout_height),
            timeout_timestamp: 1,
        };

        assert_eq!(expected, msg);
    }

    #[test]
    fn test_unlink_application() {
        let msg =
            ProfilesMsgBuilder::unlink_application("twitter", "gawrgura", Addr::unchecked("owner"));

        let expected = MsgUnlinkApplication {
            application: "twitter".into(),
            username: "gawrgura".into(),
            signer: "owner".into(),
        };

        assert_eq!(expected, msg)
    }
}
