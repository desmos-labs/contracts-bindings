//! Contains useful mocks of the Desmos x/profiles module's types made to be used in any test.

use crate::stargate::profiles::types::{
    oracle_request::CallData,
    query_application_link_owners_response::ApplicationLinkOwnerDetails,
    query_chain_link_owners_response::ChainLinkOwnerDetails,
    result::{Success, Sum},
    ApplicationLink, ApplicationLinkState, Bech32Address, ChainConfig, ChainLink,
    DTagTransferRequest, Data, OracleRequest, Pictures, Profile, Proof,
    QueryApplicationLinkByClientIdResponse, QueryApplicationLinkOwnersResponse,
    QueryApplicationLinksResponse, QueryChainLinkOwnersResponse, QueryChainLinksResponse,
    QueryDefaultExternalAddressesResponse, QueryIncomingDTagTransferRequestsResponse,
    QueryProfileResponse, Result as AppResult, SignatureValueType, SingleSignature,
};
use crate::stargate::types::Secp256k1PublicKey;

use chrono::DateTime;
use cosmwasm_std::Binary;
use desmos_std::shim::Timestamp;

/// Represents the mock dtag for unit test.
pub const MOCK_DTAG: &str = "dtag";

/// Represents the mock user for unit test.
pub const MOCK_USER: &str = "user";

/// Represents the mock receiver for unit test.
pub const MOCK_RECEIVER: &str = "receiver";

/// Represents the mock chain link address prefix for unit test.
pub const MOCK_CHAIN_LINK_ADDRESS_PREFIX: &str = "cosmos";

/// Represents the mock chain name of the chain link for unit test.
pub const MOCK_CHAIN_LINK_CHAIN_NAME: &str = "cosmos";

/// Represents the mock destination address of the chain link for unit test.
pub const MOCK_CHAIN_LINK_ADDRESS: &str = "cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2";

/// Represents the mock application of the application link for unit test.
pub const MOCK_APPLICATION_LINK_APPLICATION: &str = "twitter";

/// Represents the mock username of the application link for unit test.
pub const MOCK_APPLICATION_LINK_USERNAME: &str = "goldrake";

/// Represents the mock client id of the application link for unit test.
pub const MOCK_APPLICATION_LINK_CLIENT_ID: &str =
    "desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc-twitter-goldrake";

/// Struct that contains some utility methods to mock data of the Desmos
/// x/profiles module.
pub struct MockProfilesQueries {}

impl MockProfilesQueries {
    /// Gets a mocked instance of [`Profile`].
    pub fn get_mocked_profile() -> Profile {
        Profile {
            /// Account information should not be used in the contract layer.
            account: None,
            dtag: MOCK_DTAG.into(),
            nickname: "Goldrake".into(),
            bio: "This is Goldrake".into(),
            pictures: Some(Pictures {
                profile: "ipfs://profile.com".into(),
                cover: "ipfs://cover.com".to_string(),
            }),
            creation_date: Some(Timestamp::from(DateTime::from(
                DateTime::parse_from_rfc3339("2022-02-21T13:18:27.257641Z").unwrap(),
            ))),
        }
    }

    /// Gets a mocked instance of [`DTagTransferRequest`].
    pub fn get_mocked_dtag_transfer_request() -> DTagTransferRequest {
        DTagTransferRequest {
            dtag_to_trade: MOCK_DTAG.into(),
            sender: MOCK_USER.into(),
            receiver: MOCK_RECEIVER.into(),
        }
    }

    /// Gets a mocked instance of [`ChainLink`].
    pub fn get_mocked_chain_link() -> ChainLink {
        ChainLink {
            user: MOCK_USER.into(),
            address: Some(Bech32Address {
                value: MOCK_CHAIN_LINK_ADDRESS.into(),
                prefix: MOCK_CHAIN_LINK_ADDRESS_PREFIX.into(),
            }.into()),
            proof: Some(Proof {
                pub_key: Some(Secp256k1PublicKey{
                    key: Binary::from_base64("AyRUhKXAY6zOCjjFkPN78Q29sBKHjUx4VSZQ4HXh66IM").unwrap().to_vec(),
                }.into()),
                signature: Some(SingleSignature {
                    value_type: SignatureValueType::Raw.into(),
                    signature: Binary::from_base64("AyRUhKXAY6zOCjjFkPN78Q29sBKHjUx4VSZQ4HXh66IM").unwrap().to_vec(),
                }.into()),
                plain_text: "636f736d6f733138786e6d6c7a71727172367a74353236706e637a786536357a6b33663478676d6e6470786e32".into(),
            }),
            chain_config: Some(ChainConfig { name: MOCK_CHAIN_LINK_CHAIN_NAME.into() }),
            creation_time: Some(Timestamp::from(DateTime::from(
                DateTime::parse_from_rfc3339("2022-02-21T13:18:57.800827Z").unwrap(),
            ))),
        }
    }

    /// Gets a mocked instance of [`ChainLinkOwnerDetails`].
    pub fn get_mocked_chain_link_owner_details() -> ChainLinkOwnerDetails {
        ChainLinkOwnerDetails {
            user: MOCK_USER.into(),
            chain_name: MOCK_CHAIN_LINK_CHAIN_NAME.into(),
            target: MOCK_CHAIN_LINK_ADDRESS.into(),
        }
    }

    /// Gets a mocked instance of [`ApplicationLink`].
    pub fn get_mocked_application_link() -> ApplicationLink {
        ApplicationLink {
            user: MOCK_USER.into(),
            data: Some(Data {
                application: MOCK_APPLICATION_LINK_APPLICATION.into(),
                username: MOCK_APPLICATION_LINK_USERNAME.into(),
            }),
            state: ApplicationLinkState::VerificationSuccess.into(),
            oracle_request: Some(OracleRequest {
                id: 537807,
                oracle_script_id: 32,
                call_data: Some(CallData {
                    application: MOCK_APPLICATION_LINK_APPLICATION.into(),
                    call_data: "7b22757365726e616d65223a224c756361675f5f2335323337227d".into(),
                }),
                client_id: MOCK_APPLICATION_LINK_CLIENT_ID.into(),
            }),
            result: Some(AppResult {
                sum: Some(Sum::Success(Success {
                    value: "4c756361675f5f2345423337".into(),
                    signature: "9690d734171298eb4cc9636c36d8507535264c1fdb136c9095a6a50c41ccffa"
                        .into(),
                })),
            }),
            creation_time: Some(Timestamp::from(DateTime::from(
                DateTime::parse_from_rfc3339("2022-02-21T13:18:57.800827Z").unwrap(),
            ))),
            expiration_time: Some(Timestamp::from(DateTime::from(
                DateTime::parse_from_rfc3339("2023-02-21T13:18:57.800827Z").unwrap(),
            ))),
        }
    }

    /// Gets a mocked instance of [`ApplicationLinkOwnerDetails`].
    pub fn get_mocked_application_link_owner_details() -> ApplicationLinkOwnerDetails {
        ApplicationLinkOwnerDetails {
            user: MOCK_USER.into(),
            application: MOCK_APPLICATION_LINK_APPLICATION.into(),
            username: MOCK_APPLICATION_LINK_USERNAME.into(),
        }
    }

    /// Function that mocks a [`QueryProfileResponse`].
    pub fn get_mocked_profile_response() -> QueryProfileResponse {
        QueryProfileResponse {
            profile: Some(Self::get_mocked_profile().into()),
        }
    }

    /// Function that mocks a [`QueryIncomingDTagTransferRequestsResponse`].
    pub fn get_mocked_incoming_dtag_transfer_requests_response(
    ) -> QueryIncomingDTagTransferRequestsResponse {
        QueryIncomingDTagTransferRequestsResponse {
            requests: vec![Self::get_mocked_dtag_transfer_request()],
            pagination: None,
        }
    }

    /// Function that mocks a [`QueryChainLinksResponse`].
    pub fn get_mocked_chain_links_response() -> QueryChainLinksResponse {
        QueryChainLinksResponse {
            links: vec![Self::get_mocked_chain_link()],
            pagination: None,
        }
    }

    /// Function that mocks a [`QueryChainLinkOwnersResponse`].
    pub fn get_mocked_chain_link_owners_response() -> QueryChainLinkOwnersResponse {
        QueryChainLinkOwnersResponse {
            owners: vec![Self::get_mocked_chain_link_owner_details()],
            pagination: None,
        }
    }

    /// Function that mocks a [`QueryDefaultExternalAddressesResponse`].
    pub fn get_mocked_default_external_addresses_response() -> QueryDefaultExternalAddressesResponse
    {
        QueryDefaultExternalAddressesResponse {
            links: vec![Self::get_mocked_chain_link()],
            pagination: None,
        }
    }

    /// Function that mocks a [`QueryApplicationLinksResponse`]
    pub fn get_mocked_application_links_response() -> QueryApplicationLinksResponse {
        QueryApplicationLinksResponse {
            links: vec![Self::get_mocked_application_link()],
            pagination: None,
        }
    }

    /// Function that mocks a [`QueryApplicationLinkByClientIdResponse`]
    pub fn get_mocked_application_link_by_client_id_response(
    ) -> QueryApplicationLinkByClientIdResponse {
        QueryApplicationLinkByClientIdResponse {
            link: Some(Self::get_mocked_application_link()),
        }
    }

    /// Function that mocks a [`QueryApplicationLinkOwnersResponse`]
    pub fn get_mocked_application_link_owners_response() -> QueryApplicationLinkOwnersResponse {
        QueryApplicationLinkOwnersResponse {
            owners: vec![Self::get_mocked_application_link_owner_details()],
            pagination: None,
        }
    }
}
