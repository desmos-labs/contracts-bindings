#[cfg(test)]
mod test {
    use crate::chain_communication::DesmosCli;
    use crate::consts::TEST_SUBSPACE;
    use cosmwasm_std::Addr;
    use desmos_bindings::posts::models::ReplySetting;
    use desmos_bindings::posts::msg::PostsMsg;
    use test_contract::msg::ExecuteMsg;

    #[test]
    fn test_create_post() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let msg = PostsMsg::CreatePost {
            subspace_id: TEST_SUBSPACE,
            section_id: 0,
            external_id: None,
            text: Some("Post text".to_string()),
            entities: None,
            attachments: None,
            author: Addr::unchecked(&contract_address),
            conversation_id: None,
            reply_settings: ReplySetting::Everyone,
            referenced_posts: vec![],
        };

        desmos_cli
            .wasm_execute(
                &contract_address,
                &ExecuteMsg::DesmosMessages {
                    msgs: vec![msg.into()],
                },
            )
            .assert_success();
    }
}
