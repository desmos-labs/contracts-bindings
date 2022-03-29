use crate::profiles::models_app_links::{Data, TimeoutHeight};
use crate::profiles::models_chain_links::{ChainConfig, ChainLinkAddr, Proof};
use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProfilesMsg {
    SaveProfile {
        dtag: String,
        nickname: String,
        bio: String,
        profile_picture: String,
        cover_picture: String,
        creator: Addr,
    },
    DeleteProfile {
        creator: Addr,
    },
    RequestDtagTransfer {
        receiver: Addr,
        sender: Addr,
    },
    AcceptDtagTransferRequest {
        new_dtag: String,
        sender: Addr,
        receiver: Addr,
    },
    RefuseDtagTransferRequest {
        sender: Addr,
        receiver: Addr,
    },
    CancelDtagTransferRequest {
        receiver: Addr,
        sender: Addr,
    },
    LinkChainAccount {
        chain_address: ChainLinkAddr,
        proof: Proof,
        chain_config: ChainConfig,
        signer: Addr,
    },
    LinkApplication {
        sender: Addr,
        link_data: Data,
        call_data: String,
        source_port: String,
        source_channel: String,
        timeout_height: TimeoutHeight,
        timeout_timestamp: Uint64,
    },
}

impl ProfilesMsg {
    pub fn save_profile(
        dtag: &str,
        creator: Addr,
        nickname: &str,
        bio: &str,
        profile_picture: &str,
        cover_picture: &str,
    ) -> ProfilesMsg {
        ProfilesMsg::SaveProfile {
            dtag: dtag.to_owned(),
            nickname: nickname.to_owned(),
            bio: bio.to_owned(),
            profile_picture: profile_picture.to_owned(),
            cover_picture: cover_picture.to_owned(),
            creator,
        }
    }

    pub fn delete_profile(creator: Addr) -> ProfilesMsg {
        ProfilesMsg::DeleteProfile { creator }
    }

    pub fn request_dtag_transfer(sender: Addr, receiver: Addr) -> ProfilesMsg {
        ProfilesMsg::RequestDtagTransfer { receiver, sender }
    }

    pub fn accept_dtag_transfer_request(
        new_dtag: &str,
        sender: Addr,
        receiver: Addr,
    ) -> ProfilesMsg {
        ProfilesMsg::AcceptDtagTransferRequest {
            new_dtag: new_dtag.to_owned(),
            sender,
            receiver,
        }
    }

    pub fn refuse_dtag_transfer_request(sender: Addr, receiver: Addr) -> ProfilesMsg {
        ProfilesMsg::RefuseDtagTransferRequest { sender, receiver }
    }

    pub fn cancel_dtag_transfer_request(receiver: Addr, sender: Addr) -> ProfilesMsg {
        ProfilesMsg::CancelDtagTransferRequest { receiver, sender }
    }

    pub fn link_chain_account(
        chain_address: ChainLinkAddr,
        proof: Proof,
        chain_config: ChainConfig,
        signer: Addr,
    ) -> ProfilesMsg {
        ProfilesMsg::LinkChainAccount {
            chain_address,
            proof,
            chain_config,
            signer,
        }
    }

    pub fn link_application(
        sender: Addr,
        link_data: Data,
        call_data: String,
        source_port: String,
        source_channel: String,
        timeout_height: TimeoutHeight,
        timeout_timestamp: u64,
    ) -> ProfilesMsg {
        ProfilesMsg::LinkApplication {
            sender,
            link_data,
            call_data,
            source_port,
            source_channel,
            timeout_height,
            timeout_timestamp: timeout_timestamp.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::profiles::{
        models_app_links::{CallData, Data, OracleRequest, TimeoutHeight},
        models_chain_links::{ChainConfig, ChainLinkAddr, Proof, Signature},
        models_common::PubKey,
        msg::ProfilesMsg,
    };
    use cosmwasm_std::{Addr, Uint64};

    #[test]
    fn test_save_profile() {
        let msg = ProfilesMsg::save_profile(
            "test",
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            "",
            "",
            "",
            "",
        );
        let expected = ProfilesMsg::SaveProfile {
            dtag: "test".to_string(),
            nickname: "".to_string(),
            bio: "".to_string(),
            profile_picture: "".to_string(),
            cover_picture: "".to_string(),
            creator: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_delete_profile() {
        let msg = ProfilesMsg::delete_profile(Addr::unchecked(
            "cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69",
        ));
        let expected = ProfilesMsg::DeleteProfile {
            creator: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_request_dtag_transfer() {
        let msg = ProfilesMsg::request_dtag_transfer(
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
        );
        let expected = ProfilesMsg::RequestDtagTransfer {
            receiver: Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
            sender: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_accept_dtag_transfer_request() {
        let msg = ProfilesMsg::accept_dtag_transfer_request(
            "test",
            Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        );
        let expected = ProfilesMsg::AcceptDtagTransferRequest {
            new_dtag: "test".to_string(),
            sender: Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
            receiver: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_refuse_dtag_transfer_request() {
        let msg = ProfilesMsg::refuse_dtag_transfer_request(
            Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        );
        let expected = ProfilesMsg::RefuseDtagTransferRequest {
            sender: Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
            receiver: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_cancel_dtag_transfer_request() {
        let msg = ProfilesMsg::cancel_dtag_transfer_request(
            Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        );
        let expected = ProfilesMsg::CancelDtagTransferRequest {
            receiver: Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
            sender: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_link_chain_account() {
        let chain_addr = ChainLinkAddr {
            proto_type: "/desmos.profiles.v1beta1.Bech32Address".to_string(),
            value: "cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2".to_string(),
            prefix: "cosmos".to_string(),
        };
        let proof = Proof {
            pub_key: PubKey {
                proto_type: "/cosmos.crypto.secp256k1.PubKey".to_string(),
                key: "AyRUhKXAY6zOCjjFkPN78Q29sBKHjUx4VSZQ4HXh66IM".to_string(),
            },
            signature: Signature {
                proto_type: "/desmos.profiles.v1beta1.SingleSignatureData".to_string(),
                mode: "SIGN_MODE_DIRECT".to_string(),
                signature: "C7xppu4C4S3dgeC9TVqhyGN1hbMnMbnmWgXQI2WE8t0oHIHhDTqXyZgzhNNYiBO7ulno3G8EXO3Ep5KMFngyFg".to_string(),
            },
            plain_text: "636f736d6f733138786e6d6c7a71727172367a74353236706e637a786536357a6b33663478676d6e6470786e32".to_string(),
        };
        let chain_config = ChainConfig {
            name: "cosmos".to_string(),
        };
        let msg = ProfilesMsg::link_chain_account(
            chain_addr.clone(),
            proof.clone(),
            chain_config.clone(),
            Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
        );
        let expected = ProfilesMsg::LinkChainAccount {
            chain_address: chain_addr.clone(),
            proof,
            chain_config,
            signer: Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_link_application() {
        let data = Data {
            application: "twitter".to_string(),
            username: "goldrake".to_string(),
        };
        let oracle_req = OracleRequest {
            id: Uint64::new(537807),
            oracle_script_id: Uint64::new(32),
            call_data: CallData {
                application: "twitter".to_string(),
                call_data: "7b22757365726e616d65223a224c756361675f5f2335323337227d".to_string(),
            },
            client_id: "desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc-twitter-goldrake".to_string(),
        };

        let timeout_height = TimeoutHeight {
            revision_number: Uint64::new(0),
            revision_height: Uint64::new(0),
        };

        let msg = ProfilesMsg::link_application(
            Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
            data.clone(),
            oracle_req.call_data.call_data.clone(),
            "123".to_string(),
            "123".to_string(),
            timeout_height.clone(),
            1,
        );
        let expected = ProfilesMsg::LinkApplication {
            sender: Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
            link_data: data,
            call_data: oracle_req.call_data.call_data,
            source_port: "123".to_string(),
            source_channel: "123".to_string(),
            timeout_height,
            timeout_timestamp: Uint64::new(1),
        };
        assert_eq!(expected, msg)
    }
}
