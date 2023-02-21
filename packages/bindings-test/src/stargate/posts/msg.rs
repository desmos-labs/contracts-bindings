#[cfg(test)]
mod test {
    use crate::chain_communication::DesmosCli;
    use crate::consts::{
        TEST_DELETABLE_ATTACHMENT_ID, TEST_POLL_ID, TEST_SUBSPACE, TEST_SUBSPACE_DELETABLE_POST_ID,
        TEST_SUBSPACE_EDITABLE_POST_ID,
    };
    use chrono::{DateTime, Utc};
    use cosmwasm_std::Addr;
    use desmos_bindings::stargate::posts::msg::PostsMsgBuilder;
    use desmos_bindings::stargate::posts::types::{
        AttachmentContent, Entities, Media, Poll, Attachment, PostReference, PostReferenceType,
        poll::ProvidedAnswer, ReplySetting, Url,
    };
    use test_contract::msg::ExecuteMsg;

    #[test]
    fn test_create_post() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let msg = PostsMsgBuilder::create_post(
            TEST_SUBSPACE.into(),
            0,
            None,
            Some("Post text"),
            None,
            vec![],
            vec![],
            Addr::unchecked(&contract_address),
            None,
            ReplySetting::Everyone,
            vec![PostReference {
                r#type: PostReferenceType::Reply.into(),
                post_id: TEST_SUBSPACE_EDITABLE_POST_ID.into(),
                position: 0,
            }],
        );

        desmos_cli
            .wasm_execute(
                &contract_address,
                &ExecuteMsg::DesmosMessages {
                    msgs: vec![msg.into()],
                },
            )
            .assert_success();
    }

    #[test]
    fn test_edit_post() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let msg = PostsMsgBuilder::edit_post(
            TEST_SUBSPACE.into(),
            TEST_SUBSPACE_EDITABLE_POST_ID.into(),
            None,
            Some(Entities {
                hashtags: vec![],
                mentions: vec![],
                urls: vec![Url {
                    start: 0,
                    end: 1,
                    url:
                        "https://ipfs.infura.io/ipfs/QmT3AenKHkhCeesTUdnarqUVu91mmBk1cxQknxnUd79gY7"
                            .into(),
                    display_url: "IPFS".into(),
                }],
            }),
            vec![],
            Addr::unchecked(&contract_address),
        );

        desmos_cli
            .wasm_execute(
                &contract_address,
                &ExecuteMsg::DesmosMessages {
                    msgs: vec![msg.into()],
                },
            )
            .assert_success();
    }

    #[test]
    fn test_delete_post() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let msg = PostsMsgBuilder::delete_post(
            TEST_SUBSPACE.into(),
            TEST_SUBSPACE_DELETABLE_POST_ID.into(),
            Addr::unchecked(&contract_address),
        );

        desmos_cli
            .wasm_execute(
                &contract_address,
                &ExecuteMsg::DesmosMessages {
                    msgs: vec![msg.into()],
                },
            )
            .assert_success();
    }

    #[test]
    fn test_add_media_post_attachment() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let msg_add_media = PostsMsgBuilder::add_post_attachment(
            TEST_SUBSPACE.into(),
            TEST_SUBSPACE_EDITABLE_POST_ID.into(),
            AttachmentContent::Media(Media {
                mime_type: "test-mime".into(),
                uri: "https://test.com/image.png".into(),
            }),
            Addr::unchecked(&contract_address),
        );

        let msg_add_poll = PostsMsgBuilder::add_post_attachment(
            TEST_SUBSPACE.into(),
            TEST_SUBSPACE_EDITABLE_POST_ID.into(),
            AttachmentContent::Poll(Poll {
                question: "Test question?".to_string(),
                provided_answers: vec![
                    ProvidedAnswer {
                        text: "Answer 1".into(),
                        attachments: vec![],
                    },
                    ProvidedAnswer {
                        text: "Answer 2".into(),
                        attachments: vec![],
                    },
                ],
                end_date: Some(DateTime::from(
                    DateTime::parse_from_rfc3339("2140-01-01T10:00:20.021Z").unwrap(),
                ).into()),
                allows_multiple_answers: false,
                allows_answer_edits: false,
                final_tally_results: None,
            }),
            Addr::unchecked(&contract_address),
        );

        desmos_cli
            .wasm_execute(
                &contract_address,
                &ExecuteMsg::DesmosMessages {
                    msgs: vec![msg_add_media.into(), msg_add_poll.into()],
                },
            )
            .assert_success();
    }

    #[test]
    fn test_remove_post_attachment() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let msg = PostsMsgBuilder::remove_post_attachment(
            TEST_SUBSPACE.into(),
            TEST_SUBSPACE_EDITABLE_POST_ID.into(),
            TEST_DELETABLE_ATTACHMENT_ID,
            Addr::unchecked(&contract_address),
        );

        desmos_cli
            .wasm_execute(
                &contract_address,
                &ExecuteMsg::DesmosMessages {
                    msgs: vec![msg.into()],
                },
            )
            .assert_success();
    }

    #[test]
    fn test_answer_poll() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let msg = PostsMsgBuilder::answer_poll(
            TEST_SUBSPACE.into(),
            TEST_SUBSPACE_EDITABLE_POST_ID.into(),
            TEST_POLL_ID,
            vec![0],
            Addr::unchecked(&contract_address),
        );

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
