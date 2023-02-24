use crate::chain_communication::DesmosCli;
use crate::consts::{USER1_ADDRESS, USER2_ADDRESS};
use chrono::DateTime;
use test_contract::msg::ExecuteMsg;

use cosmwasm_std::Addr;
use desmos_bindings::posts::{
    msg::PostsMsg,
    types::{poll::ProvidedAnswer, AttachmentContent, Entities, Media, Poll, ReplySetting, Url},
};
use desmos_bindings::profiles::msg::ProfilesMsg;
use desmos_bindings::reactions::{
    msg::ReactionsMsg,
    types::{FreeTextValue, ReactionValue, RegisteredReactionValue},
};
use desmos_bindings::reports::{
    msg::ReportsMsg,
    types::{PostTarget, ReportTarget, UserTarget},
};
use desmos_bindings::subspaces::{msg::SubspacesMsg, types::Permission};

pub fn setup() {
    let cli = DesmosCli::default();

    let contract = cli.get_contract_by_code(1);

    // Create a profile for the smart contract to allow the creation of posts
    cli.execute_contract(
        &contract,
        vec![ProfilesMsg::save_profile(
            Some("test_profile_posts"),
            Some("contract_nick"),
            Some("test_bio"),
            Some("https://i.imgur.com/X2aK5Bq.jpeg"),
            Some("https://i.imgur.com/X2aK5Bq.jpeg"),
            Addr::unchecked(&contract),
        )],
    )
    .assert_success();

    // Create test subspace owned by the smart contract
    cli.execute_contract(
        &contract,
        vec![SubspacesMsg::create_subspace(
            "Test subspace",
            "",
            Addr::unchecked(&contract),
            Addr::unchecked(&contract),
        )],
    )
    .assert_success();

    // Create a test user group owned by the smart contract
    cli.execute_contract(
        &contract,
        vec![SubspacesMsg::create_user_group(
            1,
            0,
            "Test user group",
            "",
            vec![Permission::EditSubspace],
            vec![Addr::unchecked(USER1_ADDRESS)],
            Addr::unchecked(&contract),
        )],
    )
    .assert_success();

    // Set user1 permissions inside the test subspace
    cli.execute_contract(
        &contract,
        vec![SubspacesMsg::set_user_permissions(
            1,
            0,
            Addr::unchecked(USER1_ADDRESS),
            vec![
                Permission::EditSubspace,
                Permission::DeleteSubspace,
                Permission::ManageGroups,
            ],
            Addr::unchecked(&contract),
        )],
    )
    .assert_success();

    // Create a test post that can be edited
    cli.execute_contract(
        &contract,
        vec![PostsMsg::create_post(
            1,
            0,
            None,
            "Editable post",
            Some(Entities {
                urls: vec![Url {
                    start: 0,
                    end: 1,
                    url:
                        "https://ipfs.infura.io/ipfs/QmT3AenKHkhCeesTUdnarqUVu91mmBk1cxQknxnUd79gY7"
                            .into(),
                    display_url: "IPFS".into(),
                }],
                hashtags: vec![],
                mentions: vec![],
            }),
            vec![],
            vec![],
            Addr::unchecked(&contract),
            None,
            ReplySetting::Everyone,
            vec![],
        )],
    )
    .assert_success();

    // Add a post attachment that can be removed and a poll that can be answered from the tests.
    cli.execute_contract(
        &contract,
        vec![
            PostsMsg::add_post_attachment(
                1,
                1,
                AttachmentContent::Poll(Poll {
                    question: "Test question?".into(),
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
                    allows_answer_edits: true,
                    final_tally_results: None,
                }),
                Addr::unchecked(&contract),
            ),
            PostsMsg::add_post_attachment(
                1,
                1,
                AttachmentContent::Media(Media {
                    mime_type: "test-mime".into(),
                    uri: "https://test.com/image.png".into(),
                }),
                Addr::unchecked(&contract),
            ),
        ],
    )
    .assert_success();

    // Create a test post that can be deleted
    cli.execute_contract(
        &contract,
        vec![PostsMsg::create_post(
            1,
            0,
            None,
            "Deletable post",
            None,
            vec![],
            vec![],
            Addr::unchecked(&contract),
            None,
            ReplySetting::Everyone,
            vec![],
        )],
    )
    .assert_success();

    // Register a editable reaction
    cli.execute_contract(
        &contract,
        vec![ReactionsMsg::add_registered_reaction(
            1,
            "editable_code",
            "editable_value",
            Addr::unchecked(&contract),
        )],
    )
    .assert_success();

    // Register a deletable reaction
    cli.execute_contract(
        &contract,
        vec![ReactionsMsg::add_registered_reaction(
            1,
            "deletable_code",
            "deletable_value",
            Addr::unchecked(&contract),
        )],
    )
    .assert_success();

    // Create a test post that can be the target for reactions
    cli.execute_contract(
        &contract,
        vec![PostsMsg::create_post(
            1,
            0,
            None,
            "Reactions post",
            None,
            vec![],
            vec![],
            Addr::unchecked(&contract),
            None,
            ReplySetting::Everyone,
            vec![],
        )],
    )
    .assert_success();

    // Add a registered reaction to the post
    cli.execute_contract(
        &contract,
        vec![ReactionsMsg::add_reaction(
            1,
            3,
            ReactionValue::Registered(RegisteredReactionValue {
                registered_reaction_id: 1,
            }),
            Addr::unchecked(&contract),
        )],
    )
    .assert_success();

    // Add a free text reaction to the post
    cli.execute_contract(
        &contract,
        vec![ReactionsMsg::add_reaction(
            1,
            3,
            ReactionValue::FreeText(FreeTextValue {
                text: "test".into(),
            }),
            Addr::unchecked(&contract),
        )],
    )
    .assert_success();

    // Add a deletable reaction to the post
    cli.execute_contract(
        &contract,
        vec![ReactionsMsg::add_reaction(
            1,
            3,
            ReactionValue::FreeText(FreeTextValue { text: "del".into() }),
            Addr::unchecked(&contract),
        )],
    )
    .assert_success();

    // Create a test reason that can be used to create a report
    cli.execute_contract(
        &contract,
        vec![ReportsMsg::add_reason(
            1,
            "Test reason",
            "Test reason description",
            Addr::unchecked(&contract),
        )],
    )
    .assert_success();

    // Create a test reason that can be deleted
    cli.execute_contract(
        &contract,
        vec![ReportsMsg::add_reason(
            1,
            "Deletable reason",
            "Deletable reason description",
            Addr::unchecked(&contract),
        )],
    )
    .assert_success();

    // Create some reports that can be queried during the tests, one targeting the USER1 and the other one reporting a post
    cli.execute_contract(
        &contract,
        vec![
            ReportsMsg::create_report(
                1,
                vec![1],
                "",
                Addr::unchecked(&contract),
                ReportTarget::User(UserTarget {
                    user: USER1_ADDRESS.into(),
                }),
            ),
            ReportsMsg::create_report(
                1,
                vec![1],
                "",
                Addr::unchecked(&contract),
                ReportTarget::Post(PostTarget { post_id: 1 }),
            ),
        ],
    )
    .assert_success();

    // Create a report that can be deleted
    cli.execute_contract(
        &contract,
        vec![ReportsMsg::create_report(
            1,
            vec![1],
            "",
            Addr::unchecked(&contract),
            ReportTarget::User(UserTarget {
                user: USER2_ADDRESS.into(),
            }),
        )],
    )
    .assert_success();
}
