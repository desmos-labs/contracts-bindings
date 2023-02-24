#[cfg(test)]
mod test {
    use crate::chain_communication::DesmosCli;
    use crate::consts::{TEST_POLL_ID, TEST_SUBSPACE, TEST_SUBSPACE_EDITABLE_POST_ID};
    use chrono::DateTime;
    use cosmwasm_std::Addr;
    use desmos_bindings::posts::types::{
        poll::ProvidedAnswer, Entities, Poll, Post, ReplySetting, Url,
    };
    use desmos_bindings::posts::types::{
        MsgAnswerPoll, QueryPollAnswersRequest, QueryPollAnswersResponse,
        QueryPostAttachmentsRequest, QueryPostAttachmentsResponse, QueryPostRequest,
        QueryPostResponse, QuerySectionPostsRequest, QuerySectionPostsResponse,
        QuerySubspacePostsRequest, QuerySubspacePostsResponse,
    };
    use test_contract::msg::ExecuteMsg;
    use test_contract::msg::QueryMsg::DesmosChain;

    fn get_editable_post(contract_address: &str) -> Post {
        Post {
            id: TEST_SUBSPACE_EDITABLE_POST_ID.into(),
            subspace_id: TEST_SUBSPACE.into(),
            section_id: 0,
            external_id: "".to_string(),
            text: "Editable post".into(),
            entities: Some(Entities {
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
            tags: vec![],
            author: contract_address.into(),
            conversation_id: 0,
            referenced_posts: vec![],
            reply_settings: ReplySetting::Everyone.into(),
            // Leave the creation date blank since we can't guess it at runtime.
            creation_date: Some(
                DateTime::from(DateTime::parse_from_rfc3339("2140-01-01T10:00:20.021Z").unwrap())
                    .into(),
            ),
            // Leave the last edited date None since we can't guess it at runtime.
            last_edited_date: None,
        }
    }

    fn assert_post_eq(post_l: &Post, post_r: &Post, check_date: bool) {
        assert_eq!(post_l.id, post_r.id);
        assert_eq!(post_l.subspace_id, post_r.subspace_id);
        assert_eq!(post_l.section_id, post_r.section_id);
        assert_eq!(post_l.external_id, post_r.external_id);
        assert_eq!(post_l.text, post_r.text);
        assert_eq!(post_l.entities, post_r.entities);
        assert_eq!(post_l.author, post_r.author);
        assert_eq!(post_l.conversation_id, post_r.conversation_id);
        assert_eq!(post_l.referenced_posts, post_r.referenced_posts);
        assert_eq!(post_l.reply_settings, post_r.reply_settings);
        if check_date {
            assert_eq!(post_l.creation_date, post_r.creation_date);
            assert_eq!(post_l.last_edited_date, post_r.last_edited_date);
        }
    }

    #[test]
    fn test_query_subspace_posts() {
        let desmos_cli = DesmosCli::default();

        let query_msg = DesmosChain {
            request: QuerySubspacePostsRequest {
                subspace_id: TEST_SUBSPACE.into(),
                pagination: None,
            }
            .into(),
        };

        let contract_address = desmos_cli.get_contract_by_code(1);

        let res = desmos_cli.wasm_query(&contract_address, &query_msg);

        println!("{:?}", res.to_string());

        let result: QuerySubspacePostsResponse = res.to_object();

        let post = result.posts.first().unwrap();
        assert_post_eq(&get_editable_post(&contract_address), post, false);
    }

    #[test]
    fn test_query_section_posts() {
        let desmos_cli = DesmosCli::default();

        let query_msg = DesmosChain {
            request: QuerySectionPostsRequest {
                subspace_id: TEST_SUBSPACE.into(),
                section_id: 0,
                pagination: None,
            }
            .into(),
        };

        let contract_address = desmos_cli.get_contract_by_code(1);

        let result: QuerySectionPostsResponse = desmos_cli
            .wasm_query(&contract_address, &query_msg)
            .to_object();

        let post = result.posts.first().unwrap();
        assert_post_eq(&get_editable_post(&contract_address), post, false);
    }

    #[test]
    fn test_query_post() {
        let desmos_cli = DesmosCli::default();

        let query_msg = DesmosChain {
            request: QueryPostRequest {
                subspace_id: TEST_SUBSPACE.into(),
                post_id: TEST_SUBSPACE_EDITABLE_POST_ID.into(),
            }
            .into(),
        };

        let contract_address = desmos_cli.get_contract_by_code(1);

        let result: QueryPostResponse = desmos_cli
            .wasm_query(&contract_address, &query_msg)
            .to_object();

        assert_post_eq(
            &get_editable_post(&contract_address),
            &result.post.unwrap(),
            false,
        );
    }

    #[test]
    fn test_query_post_attachments() {
        let desmos_cli = DesmosCli::default();

        let query_msg = DesmosChain {
            request: QueryPostAttachmentsRequest {
                subspace_id: TEST_SUBSPACE.into(),
                post_id: TEST_SUBSPACE_EDITABLE_POST_ID.into(),
                pagination: None,
            }
            .into(),
        };

        let contract_address = desmos_cli.get_contract_by_code(1);

        let result: QueryPostAttachmentsResponse = desmos_cli
            .wasm_query(&contract_address, &query_msg)
            .to_object();

        // The first attachment should be the test poll.
        let attachment = result.attachments.first().unwrap();
        assert_eq!(attachment.id, TEST_POLL_ID);

        // Convert from the raw attachment content into the enum
        let post_attachment = Poll::try_from(attachment.content.clone().unwrap()).unwrap();

        assert_eq!(
            post_attachment,
            Poll {
                question: "Test question?".to_string(),
                provided_answers: vec![
                    ProvidedAnswer {
                        text: "Answer 1".to_string(),
                        attachments: vec![]
                    },
                    ProvidedAnswer {
                        text: "Answer 2".to_string(),
                        attachments: vec![]
                    }
                ],
                end_date: Some(
                    DateTime::from(
                        DateTime::parse_from_rfc3339("2140-01-01T10:00:20.021Z").unwrap(),
                    )
                    .into()
                ),
                allows_multiple_answers: false,
                allows_answer_edits: true,
                final_tally_results: None
            }
        )
    }

    #[test]
    fn test_query_poll_answers() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let msg = MsgAnswerPoll {
            subspace_id: TEST_SUBSPACE.into(),
            post_id: TEST_SUBSPACE_EDITABLE_POST_ID.into(),
            poll_id: TEST_POLL_ID.into(),
            answers_indexes: vec![0],
            signer: contract_address.clone(),
        };

        desmos_cli
            .wasm_execute(
                &contract_address,
                &ExecuteMsg::DesmosMessages {
                    msgs: vec![msg.into()],
                },
            )
            .assert_success();

        let query_msg = DesmosChain {
            request: QueryPollAnswersRequest {
                subspace_id: TEST_SUBSPACE.into(),
                post_id: TEST_SUBSPACE_EDITABLE_POST_ID.into(),
                poll_id: 1,
                user: "".into(),
                pagination: None,
            }
            .into(),
        };

        let result: QueryPollAnswersResponse = desmos_cli
            .wasm_query(&contract_address, &query_msg)
            .to_object();

        let answer = result.answers.first().unwrap();
        assert_eq!(Addr::unchecked(&contract_address), answer.user);
        assert_eq!(vec![0], answer.answers_indexes)
    }
}
