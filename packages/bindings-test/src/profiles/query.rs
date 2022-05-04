#[cfg(test)]
mod tests {
    use crate::chain_communication::DesmosCli;
    use crate::consts::{USER1_ADDRESS, USER2_ADDRESS};
    use cosmwasm_std::Addr;
    use desmos_bindings::profiles::models_chain_links::Address;
    use desmos_bindings::profiles::models_profile::Pictures;
    use desmos_bindings::profiles::models_query::{
        QueryChainLinksResponse, QueryIncomingDtagTransferRequestResponse, QueryProfileResponse,
    };
    use desmos_bindings::profiles::query::ProfilesQuery;
    use test_contract::msg::QueryMsg::DesmosChain;

    #[test]
    fn test_query_profile() {
        let desmos_cli = DesmosCli::default();

        let query_msg = DesmosChain {
            request: ProfilesQuery::Profile {
                user: String::from("desmos1jnpfa06xhflyjh6klwlrq8mk55s53czh6ncdm3"),
            }
            .into(),
        };

        let contract_address = desmos_cli.get_contract_by_code(1);

        let result: QueryProfileResponse = desmos_cli
            .wasm_query(&contract_address, &query_msg)
            .to_object();

        assert_eq!(result.profile.dtag, "user1");
        assert_eq!(result.profile.nickname, "user1");
        assert_eq!(result.profile.bio, "user1 bio");
        assert_eq!(
            result.profile.pictures,
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
            request: ProfilesQuery::IncomingDtagTransferRequests {
                receiver: Addr::unchecked(USER1_ADDRESS),
                pagination: None,
            }
            .into(),
        };

        let contract_address = desmos_cli.get_contract_by_code(1);
        let result: QueryIncomingDtagTransferRequestResponse = desmos_cli
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
            request: ProfilesQuery::ChainLinks {
                user: Some(Addr::unchecked(USER1_ADDRESS)),
                chain_name: None,
                target: None,
                pagination: None,
            }
            .into(),
        };

        let contract_address = desmos_cli.get_contract_by_code(1);
        let result: QueryChainLinksResponse = desmos_cli
            .wasm_query(&contract_address, &query_msg)
            .to_object();

        assert_eq!(2, result.links.len());
        assert!(match &result.links.first().unwrap().address {
            Address::Bech32 { value, .. } => {
                value.eq("cosmos1wrx0kayjzuf27gaaqult0z576y0xggq00mrc2r")
            }
            Address::Base58 { .. } => {
                false
            }
            Address::Hex { .. } => {
                false
            }
        });
        assert!(match &result.links.last().unwrap().address {
            Address::Bech32 { value, .. } => {
                value.eq("osmo1wrx0kayjzuf27gaaqult0z576y0xggq08qsgu3")
            }
            Address::Base58 { .. } => {
                false
            }
            Address::Hex { .. } => {
                false
            }
        });
    }

    #[test]
    fn test_query_chain_link_with_chain_name() {
        let desmos_cli = DesmosCli::default();

        let query_msg = DesmosChain {
            request: ProfilesQuery::ChainLinks {
                user: Some(Addr::unchecked(USER1_ADDRESS)),
                chain_name: Some("cosmos".to_string()),
                target: None,
                pagination: None,
            }
            .into(),
        };

        let contract_address = desmos_cli.get_contract_by_code(1);
        let result: QueryChainLinksResponse = desmos_cli
            .wasm_query(&contract_address, &query_msg)
            .to_object();

        assert!(match &result.links.first().unwrap().address {
            Address::Bech32 { value, .. } => {
                value.eq("cosmos1wrx0kayjzuf27gaaqult0z576y0xggq00mrc2r")
            }
            Address::Base58 { .. } => {
                false
            }
            Address::Hex { .. } => {
                false
            }
        });
    }

    #[test]
    fn test_query_chain_link_with_target() {
        let desmos_cli = DesmosCli::default();

        let query_msg = DesmosChain {
            request: ProfilesQuery::ChainLinks {
                user: Some(Addr::unchecked(USER1_ADDRESS)),
                chain_name: Some("cosmos".to_string()),
                target: Some("cosmos1wrx0kayjzuf27gaaqult0z576y0xggq00mrc2r".to_string()),
                pagination: None,
            }
            .into(),
        };

        let contract_address = desmos_cli.get_contract_by_code(1);
        let result: QueryChainLinksResponse = desmos_cli
            .wasm_query(&contract_address, &query_msg)
            .to_object();

        assert!(match &result.links.first().unwrap().address {
            Address::Bech32 { value, .. } => {
                value.eq("cosmos1wrx0kayjzuf27gaaqult0z576y0xggq00mrc2r")
            }
            Address::Base58 { .. } => {
                false
            }
            Address::Hex { .. } => {
                false
            }
        });
    }
}
