#[cfg(test)]
mod tests {
    use crate::chain_communication::DesmosCli;
    use crate::consts::{USER1_ADDRESS, USER2_ADDRESS};

    use desmos_bindings::profiles::types::{
        query_chain_link_owners_response::ChainLinkOwnerDetails, Bech32Address, Pictures, Profile,
    };
    use desmos_bindings::profiles::types::{
        QueryChainLinkOwnersRequest, QueryChainLinkOwnersResponse, QueryChainLinksRequest,
        QueryChainLinksResponse, QueryDefaultExternalAddressesRequest,
        QueryDefaultExternalAddressesResponse, QueryIncomingDTagTransferRequestsRequest,
        QueryIncomingDTagTransferRequestsResponse, QueryProfileRequest, QueryProfileResponse,
    };
    use test_contract::msg::QueryMsg::DesmosChain;

    #[test]
    fn test_query_profile() {
        let desmos_cli = DesmosCli::default();

        let query_msg = DesmosChain {
            request: QueryProfileRequest {
                user: "desmos1jnpfa06xhflyjh6klwlrq8mk55s53czh6ncdm3".into(),
            }
            .into(),
        };

        let contract_address = desmos_cli.get_contract_by_code(1);

        let result: QueryProfileResponse = desmos_cli
            .wasm_query(&contract_address, &query_msg)
            .to_object();

        let profile = Profile::try_from(result.profile.unwrap()).unwrap();
        assert_eq!(profile.dtag, "user1");
        assert_eq!(profile.nickname, "user1");
        assert_eq!(profile.bio, "user1 bio");
        assert_eq!(
            profile.pictures.unwrap(),
            Pictures {
                profile: "".to_string(),
                cover: "".to_string(),
            }
        )
    }

    #[test]
    fn test_query_dtag_transfer() {
        let desmos_cli = DesmosCli::default();

        let query_msg = DesmosChain {
            request: QueryIncomingDTagTransferRequestsRequest {
                receiver: USER1_ADDRESS.into(),
                pagination: None,
            }
            .into(),
        };

        let contract_address = desmos_cli.get_contract_by_code(1);
        let result: QueryIncomingDTagTransferRequestsResponse = desmos_cli
            .wasm_query(&contract_address, &query_msg)
            .to_object();

        let request = result.requests.first().unwrap();

        assert_eq!(request.sender, USER2_ADDRESS);
        assert_eq!(request.receiver, USER1_ADDRESS);
    }

    #[test]
    fn test_query_chain_link_from_address() {
        let desmos_cli = DesmosCli::default();

        let query_msg = DesmosChain {
            request: QueryChainLinksRequest {
                user: USER1_ADDRESS.into(),
                chain_name: "".into(),
                target: "".into(),
                pagination: None,
            }
            .into(),
        };

        let contract_address = desmos_cli.get_contract_by_code(1);
        let result: QueryChainLinksResponse = desmos_cli
            .wasm_query(&contract_address, &query_msg)
            .to_object();

        assert_eq!(2, result.links.len());
        let cosmos_address = result.links.first().unwrap();

        assert_eq!(
            Bech32Address {
                value: "cosmos1wrx0kayjzuf27gaaqult0z576y0xggq00mrc2r".into(),
                prefix: "cosmos".into(),
            },
            Bech32Address::try_from(cosmos_address.clone().address.unwrap()).unwrap(),
        );

        let osmosis_address = result.links.last().unwrap();

        assert_eq!(
            Bech32Address {
                value: "osmo1wrx0kayjzuf27gaaqult0z576y0xggq08qsgu3".into(),
                prefix: "osmo".into(),
            },
            Bech32Address::try_from(osmosis_address.clone().address.unwrap()).unwrap(),
        );
    }

    #[test]
    fn test_query_chain_link_with_chain_name() {
        let desmos_cli = DesmosCli::default();

        let query_msg = DesmosChain {
            request: QueryChainLinksRequest {
                user: USER1_ADDRESS.into(),
                chain_name: "cosmos".into(),
                target: "".into(),
                pagination: None,
            }
            .into(),
        };

        let contract_address = desmos_cli.get_contract_by_code(1);
        let result: QueryChainLinksResponse = desmos_cli
            .wasm_query(&contract_address, &query_msg)
            .to_object();

        let cosmos_address = result.links.first().unwrap();

        assert_eq!(
            Bech32Address {
                value: "cosmos1wrx0kayjzuf27gaaqult0z576y0xggq00mrc2r".into(),
                prefix: "cosmos".into(),
            },
            Bech32Address::try_from(cosmos_address.clone().address.unwrap()).unwrap(),
        );
    }

    #[test]
    fn test_query_chain_link_with_target() {
        let desmos_cli = DesmosCli::default();

        let query_msg = DesmosChain {
            request: QueryChainLinksRequest {
                user: USER1_ADDRESS.into(),
                chain_name: "cosmos".into(),
                target: "cosmos1wrx0kayjzuf27gaaqult0z576y0xggq00mrc2r".into(),
                pagination: None,
            }
            .into(),
        };

        let contract_address = desmos_cli.get_contract_by_code(1);
        let result: QueryChainLinksResponse = desmos_cli
            .wasm_query(&contract_address, &query_msg)
            .to_object();

        let cosmos_address = result.links.first().unwrap();

        assert_eq!(
            Bech32Address {
                value: "cosmos1wrx0kayjzuf27gaaqult0z576y0xggq00mrc2r".into(),
                prefix: "cosmos".into(),
            },
            Bech32Address::try_from(cosmos_address.clone().address.unwrap()).unwrap(),
        );
    }

    #[test]
    fn test_query_chain_link_owners() {
        let desmos_cli = DesmosCli::default();

        let query_msg = DesmosChain {
            request: QueryChainLinkOwnersRequest {
                chain_name: "osmosis".into(),
                target: "osmo1wrx0kayjzuf27gaaqult0z576y0xggq08qsgu3".into(),
                pagination: None,
            }
            .into(),
        };

        let contract_address = desmos_cli.get_contract_by_code(1);
        let result: QueryChainLinkOwnersResponse = desmos_cli
            .wasm_query(&contract_address, &query_msg)
            .to_object();

        let owner = result.owners.first().unwrap();
        let expected = ChainLinkOwnerDetails {
            user: USER1_ADDRESS.into(),
            chain_name: "osmosis".into(),
            target: "osmo1wrx0kayjzuf27gaaqult0z576y0xggq08qsgu3".into(),
        };
        assert_eq!(&expected, owner);
    }

    #[test]
    fn test_query_default_external_addresses() {
        let desmos_cli = DesmosCli::default();
        let query_msg = DesmosChain {
            request: QueryDefaultExternalAddressesRequest {
                owner: USER1_ADDRESS.into(),
                chain_name: "cosmos".into(),
                pagination: None,
            }
            .into(),
        };
        let contract_address = desmos_cli.get_contract_by_code(1);
        let result: QueryDefaultExternalAddressesResponse = desmos_cli
            .wasm_query(&contract_address, &query_msg)
            .to_object();

        let cosmos_address = result.links.first().unwrap();
        assert_eq!(
            Bech32Address {
                value: "cosmos1wrx0kayjzuf27gaaqult0z576y0xggq00mrc2r".into(),
                prefix: "cosmos".into(),
            },
            Bech32Address::try_from(cosmos_address.clone().address.unwrap()).unwrap(),
        );
    }
}
