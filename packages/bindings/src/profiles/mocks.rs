//! Contains some useful mocks of the Desmos x/profiles module's types made to be used in any test.

use crate::profiles::models_app_links::ApplicationLinkState;
use crate::profiles::models_chain_links::{Address, SignMode};
use crate::profiles::{
    models_app_links::{AppLinkResult, ApplicationLink, CallData, Data, OracleRequest},
    models_chain_links::{ChainConfig, ChainLink, Proof, Signature},
    models_dtag_requests::DtagTransferRequest,
    models_profile::{Account, Pictures, Profile},
    models_query::{
        QueryApplicationLinkByClientIDResponse, QueryApplicationLinksResponse,
        QueryChainLinksResponse, QueryIncomingDtagTransferRequestResponse, QueryProfileResponse,
    },
    query::ProfilesQuery,
};
use crate::types::PubKey;
use cosmwasm_std::{to_binary, Addr, Binary, ContractResult, Uint64};

/// Struct that contains some utility methods to mock data of the Desmos
/// x/profiles module.
pub struct MockProfilesQueries {}

impl MockProfilesQueries {
    /// Gets a mocked instance of [`Profile`].
    pub fn get_mock_profile() -> Profile {
        Profile {
            account: Account {
                proto_type: "/cosmos.auth.v1beta1.BaseAccount".to_string(),
                address: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
                pub_key: PubKey {
                    proto_type: "/cosmos.crypto.secp256k1.PubKey".to_string(),
                    key: Binary::from_base64("ArlRm0a5fFTHFfKha1LpDd+g3kZlyRBBF4R8PSM8Zo4Y")
                        .unwrap(),
                },
                account_number: Uint64::new(0),
                sequence: Uint64::new(15),
            },
            dtag: "goldrake".to_string(),
            nickname: "Goldrake".to_string(),
            bio: "This is Goldrake".to_string(),
            pictures: Pictures {
                profile: "".to_string(),
                cover: "".to_string(),
            },
            creation_date: "2022-02-21T13:18:27.257641Z".to_string(),
        }
    }

    /// Gets a mocked instance of [`DtagTransferRequest`].
    pub fn get_mock_dtag_transfer_request() -> DtagTransferRequest {
        DtagTransferRequest {
            dtag_to_trade: "goldrake".to_string(),
            sender: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
            receiver: Addr::unchecked("desmos1rfv0f7mx7w9d3jv3h803u38vqym9ygg344asm3"),
        }
    }

    /// Gets a mocked instance of [`ChainLink`].
    pub fn get_mock_chain_link() -> ChainLink {
        ChainLink {
            user: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
            address: Address {
                proto_type: "/desmos.profiles.v2.Bech32Addres".to_string(),
                value: "cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2".to_string(),
                prefix: Some("cosmos".to_string()),
            },
            proof: Proof {
                pub_key: PubKey {
                    proto_type: "/cosmos.crypto.secp256k1.PubKey".to_string(),
                    key: Binary::from_base64("AyRUhKXAY6zOCjjFkPN78Q29sBKHjUx4VSZQ4HXh66IM").unwrap(),
                },
                signature: Signature {
                    proto_type: "/desmos.profiles.v1beta1.SingleSignatureData".to_string(),
                    mode: SignMode::Direct,
                    signature: Binary::from_base64("C7xppu4C4S3dgeC9TVqhyGN1hbMnMbnmWgXQI2WE8t0oHIHhDTqXyZgzhNNYiBO7ulno3G8EXO3Ep5KMFngyFg").unwrap(),
                },
                plain_text: "636f736d6f733138786e6d6c7a71727172367a74353236706e637a786536357a6b33663478676d6e6470786e32".to_string(),
            },
            chain_config: ChainConfig { name: "cosmos".to_string() },
            creation_time: "2022-02-21T13:18:57.800827Z".to_string(),
        }
    }

    /// Gets a mocked instance of [`ApplicationLink`].
    pub fn get_mock_application_link() -> ApplicationLink {
        ApplicationLink {
            user: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
            data: Data {
                application: "twitter".to_string(),
                username: "goldrake".to_string(),
            },
            state: ApplicationLinkState::VerificationSuccess,
            oracle_request: OracleRequest {
                id: Uint64::new(537807),
                oracle_script_id: Uint64::new(32),
                call_data: CallData {
                    application: "twitter".to_string(),
                    call_data: "7b22757365726e616d65223a224c756361675f5f2335323337227d".to_string(),
                },
                client_id: "desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc-twitter-goldrake"
                    .to_string(),
            },
            result: Some(AppLinkResult::Success {
                value: "4c756361675f5f2345423337".to_string(),
                signature: "9690d734171298eb4cc9636c36d8507535264c1fdb136c9095a6a50c41ccffa"
                    .to_string(),
            }),
            creation_time: "2022-02-21T13:18:57.800827Z".to_string(),
        }
    }
}

/// Functions that mocks the profile query responses.
pub fn mock_profiles_query_response(query: &ProfilesQuery) -> ContractResult<Binary> {
    let response = match query {
        ProfilesQuery::Profile { .. } => {
            let profile = MockProfilesQueries::get_mock_profile();
            to_binary(&QueryProfileResponse { profile })
        }
        ProfilesQuery::IncomingDtagTransferRequests { .. } => {
            let incoming_dtag_requests = MockProfilesQueries::get_mock_dtag_transfer_request();
            to_binary(&QueryIncomingDtagTransferRequestResponse {
                requests: vec![incoming_dtag_requests],
                pagination: Default::default(),
            })
        }
        ProfilesQuery::ChainLinks { .. } => {
            let chain_link = MockProfilesQueries::get_mock_chain_link();
            to_binary(&QueryChainLinksResponse {
                links: vec![chain_link],
                pagination: Default::default(),
            })
        }
        ProfilesQuery::ApplicationLinks { .. } => {
            let app_link = MockProfilesQueries::get_mock_application_link();
            to_binary(&QueryApplicationLinksResponse {
                links: vec![app_link],
                pagination: Default::default(),
            })
        }
        ProfilesQuery::ApplicationLinkByChainID { .. } => {
            let app_link = MockProfilesQueries::get_mock_application_link();
            to_binary(&QueryApplicationLinkByClientIDResponse { link: app_link })
        }
    };
    response.into()
}

#[cfg(test)]
mod tests {
    use crate::profiles::{
        mocks::{mock_profiles_query_response, MockProfilesQueries},
        models_query::{
            QueryApplicationLinkByClientIDResponse, QueryApplicationLinksResponse,
            QueryChainLinksResponse, QueryIncomingDtagTransferRequestResponse,
            QueryProfileResponse,
        },
        query::ProfilesQuery,
    };
    use cosmwasm_std::{to_binary, Addr};

    #[test]
    fn test_query_profile() {
        let query = ProfilesQuery::Profile {
            user: "".to_string(),
        };
        let response = mock_profiles_query_response(&query);
        let expected = to_binary(&QueryProfileResponse {
            profile: MockProfilesQueries::get_mock_profile(),
        });
        assert_eq!(response.into_result().ok(), expected.ok())
    }

    #[test]
    fn test_query_incoming_dtag_transfer_requests() {
        let query = ProfilesQuery::IncomingDtagTransferRequests {
            receiver: Addr::unchecked(""),
            pagination: Default::default(),
        };
        let response = mock_profiles_query_response(&query);
        let expected = to_binary(&QueryIncomingDtagTransferRequestResponse {
            requests: vec![MockProfilesQueries::get_mock_dtag_transfer_request()],
            pagination: Default::default(),
        });
        assert_eq!(response.into_result().ok(), expected.ok())
    }

    #[test]
    fn test_query_chain_links() {
        let query = ProfilesQuery::ChainLinks {
            user: Some(Addr::unchecked("")),
            chain_name: Some("".to_string()),
            target: Some("".to_string()),
            pagination: Default::default(),
        };
        let response = mock_profiles_query_response(&query);
        let expected = to_binary(&QueryChainLinksResponse {
            links: vec![MockProfilesQueries::get_mock_chain_link()],
            pagination: Default::default(),
        });
        assert_eq!(response.into_result().ok(), expected.ok())
    }

    #[test]
    fn test_query_app_links() {
        let query = ProfilesQuery::ApplicationLinks {
            user: Some(Addr::unchecked("")),
            application: Some("".to_string()),
            username: Some("".to_string()),
            pagination: Default::default(),
        };
        let response = mock_profiles_query_response(&query);
        let expected = to_binary(&QueryApplicationLinksResponse {
            links: vec![MockProfilesQueries::get_mock_application_link()],
            pagination: Default::default(),
        });
        assert_eq!(response.into_result().ok(), expected.ok())
    }

    #[test]
    fn test_query_application_link_by_chain_id() {
        let query = ProfilesQuery::ApplicationLinkByChainID {
            client_id: "".to_string(),
        };
        let response = mock_profiles_query_response(&query);
        let expected = to_binary(&QueryApplicationLinkByClientIDResponse {
            link: MockProfilesQueries::get_mock_application_link(),
        });
        assert_eq!(response.into_result().ok(), expected.ok())
    }
}
