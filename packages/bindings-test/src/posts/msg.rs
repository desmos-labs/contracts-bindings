#[cfg(test)]
mod test {
    use crate::chain_communication::DesmosCli;
    use crate::consts::{
        TEST_DELETABLE_ATTACHMENT_ID, TEST_POLL_ID, TEST_SUBSPACE, TEST_SUBSPACE_DELETABLE_POST_ID,
        TEST_SUBSPACE_EDITABLE_POST_ID,
    };
    use crate::posts::create_test_post;

    use chrono::DateTime;
    use cosmwasm_std::Addr;
    use desmos_bindings::posts::msg::PostsMsg;
    use desmos_bindings::posts::types::{
        poll::ProvidedAnswer, AttachmentContent, Entities, Media, Poll, PostReference,
        PostReferenceType, ReplySetting, Url,
    };
    use desmos_bindings::subspaces::msg::SubspacesMsg;
    use test_contract::msg::ExecuteMsg;

    #[test]
    fn test_create_post() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1, 0);

        let msg = PostsMsg::create_post(
            TEST_SUBSPACE,
            0,
            None,
            "Post text",
            None,
            vec![],
            vec![],
            Addr::unchecked(&contract_address),
            None,
            ReplySetting::Everyone,
            vec![PostReference {
                r#type: PostReferenceType::Reply.into(),
                post_id: TEST_SUBSPACE_EDITABLE_POST_ID,
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
        let contract_address = desmos_cli.get_contract_by_code(1, 0);

        let msg = PostsMsg::edit_post(
            TEST_SUBSPACE,
            TEST_SUBSPACE_EDITABLE_POST_ID,
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
        let contract_address = desmos_cli.get_contract_by_code(1, 0);

        let msg = PostsMsg::delete_post(
            TEST_SUBSPACE,
            TEST_SUBSPACE_DELETABLE_POST_ID,
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
        let contract_address = desmos_cli.get_contract_by_code(1, 0);

        let msg_add_media = PostsMsg::add_post_attachment(
            TEST_SUBSPACE,
            TEST_SUBSPACE_EDITABLE_POST_ID,
            AttachmentContent::Media(Media {
                mime_type: "test-mime".into(),
                uri: "https://test.com/image.png".into(),
            }),
            Addr::unchecked(&contract_address),
        );

        let msg_add_poll = PostsMsg::add_post_attachment(
            TEST_SUBSPACE,
            TEST_SUBSPACE_EDITABLE_POST_ID,
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
                end_date: Some(
                    DateTime::from(
                        DateTime::parse_from_rfc3339("2140-01-01T10:00:20.021Z").unwrap(),
                    )
                    .into(),
                ),
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
        let contract_address = desmos_cli.get_contract_by_code(1, 0);

        let msg = PostsMsg::remove_post_attachment(
            TEST_SUBSPACE,
            TEST_SUBSPACE_EDITABLE_POST_ID,
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
        let contract_address = desmos_cli.get_contract_by_code(1, 0);

        let msg = PostsMsg::answer_poll(
            TEST_SUBSPACE,
            TEST_SUBSPACE_EDITABLE_POST_ID,
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

    #[test]
    fn test_move_post() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1, 0);

        // Create target subspace for moving post
        desmos_cli
            .execute_contract(
                &contract_address,
                vec![SubspacesMsg::create_subspace(
                    "Test target subspace",
                    "",
                    Addr::unchecked(&contract_address),
                    Addr::unchecked(&contract_address),
                )
                .into()],
            )
            .assert_success();

        // Get target subspace and post
        let target_subspace = desmos_cli
            .query_subspaces(None)
            .subspaces
            .last()
            .unwrap()
            .clone();
        let post = create_test_post(TEST_SUBSPACE, &contract_address);

        let move_post_msg = PostsMsg::move_post(
            TEST_SUBSPACE,
            post.id,
            target_subspace.id,
            0,
            Addr::unchecked(&contract_address),
        );

        desmos_cli
            .execute_contract(&contract_address, vec![move_post_msg.into()])
            .assert_success()
    }

    #[test]
    fn test_request_then_cancel_post_owner_transfer() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1, 0);
        let receiver_contract_address = desmos_cli.get_contract_by_code(1, 1);

        let post = create_test_post(TEST_SUBSPACE, &contract_address);
        let request_post_owner_transfer_msg = PostsMsg::request_post_owner_transfer(
            TEST_SUBSPACE,
            post.id,
            Addr::unchecked(&receiver_contract_address),
            Addr::unchecked(&contract_address),
        );

        let cancel_post_owner_transfer_request_msg = PostsMsg::cancel_post_owner_transfer_request(
            TEST_SUBSPACE,
            post.id,
            Addr::unchecked(&contract_address),
        );

        desmos_cli
            .execute_contract(
                &contract_address,
                vec![
                    request_post_owner_transfer_msg.into(),
                    cancel_post_owner_transfer_request_msg.into(),
                ],
            )
            .assert_success()
    }

    #[test]
    fn test_request_then_accept_post_owner_transfer() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1, 0);
        let receiver_contract_address = desmos_cli.get_contract_by_code(1, 1);

        // Create a post owner transfer request to receiver
        let post = create_test_post(TEST_SUBSPACE, &contract_address);
        let request_post_owner_transfer_msg = PostsMsg::request_post_owner_transfer(
            TEST_SUBSPACE,
            post.id,
            Addr::unchecked(&receiver_contract_address),
            Addr::unchecked(&contract_address),
        );
        desmos_cli
            .execute_contract(
                &contract_address,
                vec![request_post_owner_transfer_msg.into()],
            )
            .assert_success();

        // Receiver accepts a post owner transfer request
        let accept_post_owner_transfer_request_msg = PostsMsg::accept_post_owner_transfer_request(
            TEST_SUBSPACE,
            post.id,
            Addr::unchecked(&receiver_contract_address),
        );
        desmos_cli
            .execute_contract(
                &receiver_contract_address,
                vec![accept_post_owner_transfer_request_msg.into()],
            )
            .assert_success()
    }

    #[test]
    fn test_request_then_refuse_post_owner_transfer() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1, 0);
        let receiver_contract_address = desmos_cli.get_contract_by_code(1, 1);

        // Create a post owner transfer request to receiver
        let post = create_test_post(TEST_SUBSPACE, &contract_address);
        let request_post_owner_transfer_msg = PostsMsg::request_post_owner_transfer(
            TEST_SUBSPACE,
            post.id,
            Addr::unchecked(&receiver_contract_address),
            Addr::unchecked(&contract_address),
        );
        desmos_cli
            .execute_contract(
                &contract_address,
                vec![request_post_owner_transfer_msg.into()],
            )
            .assert_success();

        // Receiver refuses a post owner transfer request
        let refuse_post_owner_transfer_request_msg = PostsMsg::refuse_post_owner_transfer_request(
            TEST_SUBSPACE,
            post.id,
            Addr::unchecked(&receiver_contract_address),
        );
        desmos_cli
            .execute_contract(
                &receiver_contract_address,
                vec![refuse_post_owner_transfer_request_msg.into()],
            )
            .assert_success()
    }
}
