use std::{env, path::PathBuf};

use crate::code_generator::{CodeGenerator, CosmosProject};
use syn::__private::quote::{format_ident, quote};

mod code_generator;
mod git;
mod mod_gen;
mod transform;
mod transformers;

/// The desmos commit or tag to be cloned and used to build the proto files
const DESMOS_REV: &str = "581e029ce1c0f84413a961258b4d2048ab386496";

/// Directory where the desmos submodule is located
const DESMOS_DIR: &str = "../../dependencies/desmos/";

/// URL where the desmos repository is located
const DESMOS_REPO_URL: &str = "https://github.com/desmos-labs/desmos.git";

/// The directory generated proto files go into in this repo
const OUT_DIR: &str = "../std/src/proto/";

/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";

pub fn run() {
    git::try_clone_desmos_repo();

    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == "--update-deps") {
        git::update_desmos_repo();
    }

    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let out_dir: PathBuf = OUT_DIR.parse().unwrap();

    let desmos_project = CosmosProject {
        name: "desmos".into(),
        version: DESMOS_REV.into(),
        project_dir: DESMOS_DIR.into(),
    };

    let code_generator = CodeGenerator::new(out_dir, tmp_build_dir, desmos_project, vec![]);

    code_generator.generate();
}
