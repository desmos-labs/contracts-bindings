#[cfg(test)]
mod test {
    use crate::chain_communication::DesmosCli;
    use crate::consts::{TEST_POLL_ID, TEST_SUBSPACE, TEST_SUBSPACE_EDITABLE_POST_ID};
    use cosmwasm_std::{Addr, Uint64};
    use desmos_bindings::posts::models::{Post, PostAttachment, ProvidedAnswer, ReplySetting};
    use desmos_bindings::posts::models_query::{
        QueryPostAttachmentsResponse, QueryPostResponse, QuerySectionPostsResponse,
        QuerySubspacePostsResponse,
    };
    use desmos_bindings::posts::query::PostsQuery;
    use test_contract::msg::QueryMsg::DesmosChain;

    fn get_editable_post(contract_address: &str) -> Post {
        Post {
            id: TEST_SUBSPACE_EDITABLE_POST_ID,
            subspace_id: TEST_SUBSPACE,
            section_id: 0,
            eternal_id: None,
            text: Some("Editable post".to_string()),
            entities: None,
            author: Addr::unchecked(contract_address),
            conversation_id: Some(Uint64::new(0)),
            referenced_posts: vec![],
            reply_settings: ReplySetting::Everyone,
            // Leave the creation date blank since we can't guess it at runtime.
            creation_date: "".to_string(),
            last_edit_date: None,
        }
    }

    fn assert_post_eq(post_l: &Post, post_r: &Post, check_creation_date: bool) {
        assert_eq!(post_l.id, post_r.id);
        assert_eq!(post_l.subspace_id, post_r.subspace_id);
        assert_eq!(post_l.section_id, post_r.section_id);
        assert_eq!(post_l.eternal_id, post_r.eternal_id);
        assert_eq!(post_l.text, post_r.text);
        assert_eq!(post_l.entities, post_r.entities);
        assert_eq!(post_l.author, post_r.author);
        assert_eq!(post_l.conversation_id, post_r.conversation_id);
        assert_eq!(post_l.referenced_posts, post_r.referenced_posts);
        assert_eq!(post_l.reply_settings, post_r.reply_settings);
        if check_creation_date {
            assert_eq!(post_l.creation_date, post_r.creation_date);
        }
        assert_eq!(post_l.last_edit_date, post_r.last_edit_date);
    }

    #[test]
    fn test_query_subspace_posts() {
        let desmos_cli = DesmosCli::default();

        let query_msg = DesmosChain {
            request: PostsQuery::SubspacePosts {
                subspace_id: TEST_SUBSPACE,
                pagination: None,
            }
            .into(),
        };

        let contract_address = desmos_cli.get_contract_by_code(1);

        let result: QuerySubspacePostsResponse = desmos_cli
            .wasm_query(&contract_address, &query_msg)
            .to_object();

        let post = result.posts.first().unwrap();
        assert_post_eq(&get_editable_post(&contract_address), post, false);
    }

    #[test]
    fn test_query_section_posts() {
        let desmos_cli = DesmosCli::default();

        let query_msg = DesmosChain {
            request: PostsQuery::SectionPosts {
                subspace_id: TEST_SUBSPACE,
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
    fn test_query_posts() {
        let desmos_cli = DesmosCli::default();

        let query_msg = DesmosChain {
            request: PostsQuery::Post {
                subspace_id: TEST_SUBSPACE,
                post_id: TEST_SUBSPACE_EDITABLE_POST_ID,
            }
            .into(),
        };

        let contract_address = desmos_cli.get_contract_by_code(1);

        let result: QueryPostResponse = desmos_cli
            .wasm_query(&contract_address, &query_msg)
            .to_object();

        assert_post_eq(&get_editable_post(&contract_address), &result.post, false);
    }

    #[test]
    fn test_query_post_attachments() {
        let desmos_cli = DesmosCli::default();

        let query_msg = DesmosChain {
            request: PostsQuery::PostAttachments {
                subspace_id: TEST_SUBSPACE,
                post_id: TEST_SUBSPACE_EDITABLE_POST_ID,
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
        let post_attachment: PostAttachment =
            PostAttachment::try_from(attachment.content.clone()).unwrap();

        assert_eq!(
            post_attachment,
            PostAttachment::Poll {
                question: "Test question?".to_string(),
                provided_answers: vec![
                    ProvidedAnswer {
                        text: Some("Answer 1".to_string()),
                        attachments: vec![]
                    },
                    ProvidedAnswer {
                        text: Some("Answer 2".to_string()),
                        attachments: vec![]
                    }
                ],
                end_date: "2140-01-01T10:00:20.021Z".to_string(),
                allows_multiple_answers: false,
                allows_answer_edits: false,
                final_tally_results: None
            }
        )
    }
}
