#[cfg(test)]
mod tests {
    use crate::chain_communication::DesmosCli;
    use crate::consts::{TEST_SUBSPACE, USER1_ADDRESS, USER2_ADDRESS};
    use cosmwasm_std::Addr;
    use desmos_bindings::legacy::relationships::models_query::{
        QueryBlocksResponse, QueryRelationshipsResponse,
    };
    use desmos_bindings::legacy::relationships::query::RelationshipsQuery;
    use test_contract::msg::QueryMsg::DesmosChain;

    #[test]
    fn test_query_all_relationships() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let query = DesmosChain {
            request: RelationshipsQuery::Relationships {
                subspace_id: TEST_SUBSPACE,
                user: None,
                counterparty: None,
                pagination: None,
            }
            .into(),
        };

        let response: QueryRelationshipsResponse =
            desmos_cli.wasm_query(&contract_address, &query).to_object();

        // Should be only one relationship between user1 and user2
        assert_eq!(1, response.relationships.len());
    }

    #[test]
    fn test_query_user1_relationships() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let query = DesmosChain {
            request: RelationshipsQuery::Relationships {
                subspace_id: TEST_SUBSPACE,
                user: Some(Addr::unchecked(USER1_ADDRESS)),
                counterparty: None,
                pagination: None,
            }
            .into(),
        };

        let response: QueryRelationshipsResponse =
            desmos_cli.wasm_query(&contract_address, &query).to_object();

        // Should be only one relationship between user1 and user2
        assert_eq!(1, response.relationships.len());

        // Check that the relationship is between user1 and user2
        let relationship = response.relationships.first().unwrap();
        assert_eq!(USER1_ADDRESS, relationship.creator.as_str());
        assert_eq!(USER2_ADDRESS, relationship.counterparty.as_str());
        assert_eq!(TEST_SUBSPACE, relationship.subspace_id);
    }

    #[test]
    fn test_query_user1_user2_relationship() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let query = DesmosChain {
            request: RelationshipsQuery::Relationships {
                subspace_id: TEST_SUBSPACE,
                user: Some(Addr::unchecked(USER1_ADDRESS)),
                counterparty: Some(Addr::unchecked(USER2_ADDRESS)),
                pagination: None,
            }
            .into(),
        };

        let response: QueryRelationshipsResponse =
            desmos_cli.wasm_query(&contract_address, &query).to_object();

        // Should be only one relationship between user1 and user2
        assert_eq!(1, response.relationships.len());

        // Check that the relationship is between user1 and user2
        let relationship = response.relationships.first().unwrap();
        assert_eq!(USER1_ADDRESS, relationship.creator.as_str());
        assert_eq!(USER2_ADDRESS, relationship.counterparty.as_str());
        assert_eq!(TEST_SUBSPACE, relationship.subspace_id);
    }

    #[test]
    fn test_query_all_blocks() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let query = DesmosChain {
            request: RelationshipsQuery::Blocks {
                subspace_id: TEST_SUBSPACE,
                blocker: None,
                blocked: None,
                pagination: None,
            }
            .into(),
        };

        let response: QueryBlocksResponse =
            desmos_cli.wasm_query(&contract_address, &query).to_object();

        // Should be only one block between user2 and user1
        assert_eq!(1, response.blocks.len());
    }

    #[test]
    fn test_query_user2_blocks() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let query = DesmosChain {
            request: RelationshipsQuery::Blocks {
                subspace_id: TEST_SUBSPACE,
                blocker: Some(Addr::unchecked(USER2_ADDRESS)),
                blocked: None,
                pagination: None,
            }
            .into(),
        };

        let response: QueryBlocksResponse =
            desmos_cli.wasm_query(&contract_address, &query).to_object();

        // Should be only one block between user2 and user1
        assert_eq!(1, response.blocks.len());

        // Check that the fetched block was created from user2 to block user1
        let block = response.blocks.first().unwrap();
        assert_eq!(TEST_SUBSPACE, block.subspace_id);
        assert_eq!(USER2_ADDRESS, block.blocker.as_str());
        assert_eq!(USER1_ADDRESS, block.blocked.as_str());
    }

    #[test]
    fn test_query_user2_user1_block() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let query = DesmosChain {
            request: RelationshipsQuery::Blocks {
                subspace_id: TEST_SUBSPACE,
                blocker: Some(Addr::unchecked(USER2_ADDRESS)),
                blocked: Some(Addr::unchecked(USER1_ADDRESS)),
                pagination: None,
            }
            .into(),
        };

        let response: QueryBlocksResponse =
            desmos_cli.wasm_query(&contract_address, &query).to_object();

        // Should be only one block between user2 and user1
        assert_eq!(1, response.blocks.len());

        // Check that the fetched block was created from user2 to block user1
        let block = response.blocks.first().unwrap();
        assert_eq!(TEST_SUBSPACE, block.subspace_id);
        assert_eq!(USER2_ADDRESS, block.blocker.as_str());
        assert_eq!(USER1_ADDRESS, block.blocked.as_str());
    }
}
