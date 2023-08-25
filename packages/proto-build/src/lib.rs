use std::{env, path::PathBuf};

use crate::code_generator::{CodeGenerator, CosmosProject};
use syn::__private::quote::{format_ident, quote};

mod code_generator;
mod git;
mod mod_gen;
mod transform;
mod transformers;

/// The desmos commit or tag to be cloned and used to build the proto files
const DESMOS_REV: &str = "v6.0.0";

/// Directory where the desmos submodule is located
const DESMOS_DIR: &str = "../../dependencies/desmos/";

/// URL where the desmos repository is located
const DESMOS_REPO_URL: &str = "https://github.com/desmos-labs/desmos.git";

/// Directory where the cosmos-sdk submodule is located
const COSMOS_SDK_DIR: &str = "../../dependencies/cosmos-sdk/";

/// The Cosmos SDK commit or tag to be cloned and used to build the proto files
const COSMOS_SDK_REV: &str = "v0.47.4-desmos";

/// URL where the Cosmos SDK repository is located
const COSMOS_SDK_REPO_URL: &str = "https://github.com/desmos-labs/cosmos-sdk.git";

/// Directory where the cosmos-sdk submodule is located
const IBC_DIR: &str = "../../dependencies/ibc/";

/// The IBC commit or tag to be cloned and used to build the proto files
const IBC_REV: &str = "v7.2.0";

/// URL where the IBC repository is located
const IBC_REPO_URL: &str = "https://github.com/cosmos/ibc-go.git";

/// The directory generated proto files go into in this repo
const OUT_DIR: &str = "../bindings/src/proto/";

/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";

pub fn run() {
    git::try_clone_repo("Desmos", DESMOS_REPO_URL, DESMOS_DIR, DESMOS_REV);
    git::try_clone_repo(
        "Cosmos SDK",
        COSMOS_SDK_REPO_URL,
        COSMOS_SDK_DIR,
        COSMOS_SDK_REV,
    );
    git::try_clone_repo("IBC", IBC_REPO_URL, IBC_DIR, IBC_REV);

    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == "--update-deps") {
        git::update_repo("Desmos", DESMOS_DIR, DESMOS_REV);
        git::update_repo("Cosmos SDK", COSMOS_SDK_DIR, COSMOS_SDK_REV);
        git::update_repo("IBC", IBC_DIR, IBC_REV);
    }

    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let out_dir: PathBuf = OUT_DIR.parse().unwrap();

    let desmos_project = CosmosProject {
        name: "desmos".into(),
        version: DESMOS_REV.into(),
        project_dir: DESMOS_DIR.into(),
        include_mods: vec![
            "profiles/v3".to_string(),
            "relationships/v1".to_string(),
            "subspaces/v3".to_string(),
            "posts/v3".to_string(),
            "reactions/v1".to_string(),
            "reports/v1".to_string(),
            "tokenfactory/v1".to_string(),
        ],
    };

    let cosmos_project = CosmosProject {
        name: "cosmos".to_string(),
        version: COSMOS_SDK_REV.to_string(),
        project_dir: COSMOS_SDK_DIR.to_string(),
        include_mods: vec![
            "authz/v1beta1/authz.proto".to_string(),
            "base/query/v1beta1/pagination.proto".to_string(),
            "base/v1beta1/coin.proto".to_string(),
            "crypto/ed25519".to_string(),
            "crypto/secp256k1".to_string(),
            "crypto/secp256r1".to_string(),
            "crypto/multisig".to_string(),
            "tx/signing".to_string(),
            "upgrade/v1beta1/upgrade.proto".to_string(),
            "feegrant/v1beta1/feegrant.proto".to_string(),
        ],
    };

    let ibc_project = CosmosProject {
        name: "ibc".to_string(),
        version: IBC_REV.to_string(),
        project_dir: IBC_DIR.to_string(),
        include_mods: vec!["core/client/v1/client.proto".to_string()],
    };

    let code_generator = CodeGenerator::new(
        out_dir,
        tmp_build_dir,
        desmos_project,
        vec![cosmos_project, ibc_project],
    );

    code_generator.generate();
}
