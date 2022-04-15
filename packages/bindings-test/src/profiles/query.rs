#[cfg(test)]
mod tests {
    use crate::chain_communication::DesmosCli;
    use cosmwasm_std::QueryRequest;
    use desmos_bindings::profiles::models_profile::{Pictures, Profile};
    use desmos_bindings::profiles::query::ProfilesQuery;
    use desmos_bindings::query::DesmosQuery;
    use serde_json::Value;
    use test_contract::msg::QueryMsg::DesmosChain;

    #[test]
    fn test_query_profile() {
        let desmos_cli = DesmosCli::default();

        let query_msg = DesmosChain {
            request: QueryRequest::Custom(DesmosQuery::Profiles(ProfilesQuery::Profile {
                user: String::from("desmos1jnpfa06xhflyjh6klwlrq8mk55s53czh6ncdm3"),
            })),
        };

        let contract_address = desmos_cli.get_contract_by_code(1);

        let result = desmos_cli.wasm_query(&contract_address, &query_msg);
        let decode = base64::decode(result.data.data.to_base64()).unwrap();
        let result = String::from_utf8(decode).unwrap();
        let json: Value = serde_json::from_str(&result).unwrap();
        let profile: Profile =
            serde_json::from_value(json.get("profile").unwrap().clone()).unwrap();

        assert_eq!(profile.dtag, "user1");
        assert_eq!(profile.nickname, "");
        assert_eq!(profile.bio, "");
        assert_eq!(
            profile.pictures,
            Pictures {
                profile: "".to_string(),
                cover: "".to_string(),
            }
        )
    }
}
