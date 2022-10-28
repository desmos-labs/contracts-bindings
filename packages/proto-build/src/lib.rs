use std::{env, path::PathBuf};

use crate::code_generator::{CodeGenerator, CosmosProject};
use syn::__private::quote::{format_ident, quote};

mod code_generator;
mod git;
mod mod_gen;
mod transform;
mod transformers;

/// The desmos commit or tag to be cloned and used to build the proto files
const DESMOS_REV: &str = "v4.6.1";

/// Directory where the desmos submodule is located
const DESMOS_DIR: &str = "../../dependencies/desmos/";

/// The directory generated proto files go into in this repo
const OUT_DIR: &str = "../std/src/proto/";

/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";

pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == "--update-deps") {
        git::update_submodules();
    }

    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let out_dir: PathBuf = OUT_DIR.parse().unwrap();

    let desmos_project = CosmosProject {
        name: "desmos".to_string(),
        version: DESMOS_REV.to_string(),
        project_dir: DESMOS_DIR.to_string(),
    };

    let code_generator = CodeGenerator::new(
        out_dir,
        tmp_build_dir,
        desmos_project,
        vec![],
    );

    code_generator.generate();
}