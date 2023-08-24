mod msg;
mod query;

use crate::chain_communication::DesmosCli;
use cosmwasm_std::Addr;

use desmos_bindings::cosmos_types::PageRequest;
use desmos_bindings::posts::msg::PostsMsg;
use desmos_bindings::posts::types::Post;

pub fn create_sample_post(subspace_id: u64, contract_address: &str) -> &Post {
    let desmos_cli = DesmosCli::default();

    // Create a post
    desmos_cli
        .execute_contract(contract, PostsMsg::create_post(
            subspace_id,
            0,
            None,
            "Sample post",
            Some(Entities {
                urls: vec![Url {
                    start: 0,
                    end: 1,
                    url: "https://ipfs.infura.io/ipfs/QmT3AenKHkhCeesTUdnarqUVu91mmBk1cxQknxnUd79gY7"
                        .into(),
                    display_url: "IPFS".into(),
                }],
                hashtags: vec![],
                mentions: vec![],
            }),
            vec![],
            vec![],
            Addr::unchecked(contract_address),
            None,
            ReplySetting::Everyone,
            vec![],
        ))
        .assert_success();

    // query the created post
    let result: QuerySubspacePostsResponse = desmos_cli.wasm_query(
        contract_address,
        DesmosChain {
            request: QuerySubspacePostsRequest {
                subspace_id,
                pagination: Some(PageRequest {
                    key: vec![],
                    limit: 1,
                    offset: 0,
                    count_total: false,
                    reverse: true,
                }),
            }
            .into(),
        },
    );

    result.posts.first().unwrap()
}
