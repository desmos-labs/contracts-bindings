#[cfg(test)]
mod tests {
    use crate::chain_communication::DesmosCli;
    use crate::consts::{TEST_SUBSPACE, USER1_ADDRESS, USER2_ADDRESS};
    use desmos_bindings::relationships::types::{
        QueryBlocksRequest, QueryBlocksResponse, QueryRelationshipsRequest,
        QueryRelationshipsResponse,
    };
    use test_contract::msg::QueryMsg::DesmosChain;

    #[test]
    fn test_query_all_relationships() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1, 0);

        let query = DesmosChain {
            request: QueryRelationshipsRequest {
                subspace_id: TEST_SUBSPACE,
                user: "".into(),
                counterparty: "".into(),
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
        let contract_address = desmos_cli.get_contract_by_code(1, 0);

        let query = DesmosChain {
            request: QueryRelationshipsRequest {
                subspace_id: TEST_SUBSPACE,
                user: USER1_ADDRESS.into(),
                counterparty: "".into(),
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
        let contract_address = desmos_cli.get_contract_by_code(1, 0);

        let query = DesmosChain {
            request: QueryRelationshipsRequest {
                subspace_id: TEST_SUBSPACE,
                user: USER1_ADDRESS.into(),
                counterparty: USER2_ADDRESS.into(),
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
        let contract_address = desmos_cli.get_contract_by_code(1, 0);

        let query = DesmosChain {
            request: QueryBlocksRequest {
                subspace_id: TEST_SUBSPACE,
                blocker: "".into(),
                blocked: "".into(),
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
        let contract_address = desmos_cli.get_contract_by_code(1, 0);

        let query = DesmosChain {
            request: QueryBlocksRequest {
                subspace_id: TEST_SUBSPACE,
                blocker: USER2_ADDRESS.into(),
                blocked: "".into(),
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
        let contract_address = desmos_cli.get_contract_by_code(1, 0);

        let query = DesmosChain {
            request: QueryBlocksRequest {
                subspace_id: TEST_SUBSPACE,
                blocker: USER2_ADDRESS.into(),
                blocked: USER1_ADDRESS.into(),
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
