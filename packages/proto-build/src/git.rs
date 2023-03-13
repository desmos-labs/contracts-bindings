use crate::{DESMOS_DIR, DESMOS_REPO_URL, DESMOS_REV};
use log::info;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use std::process;

fn run_command(command: impl AsRef<OsStr>, args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    let stdout = process::Stdio::inherit();

    let exit_status = process::Command::new(command)
        .args(args)
        .stdout(stdout)
        .status()
        .expect("git exit status missing");

    if !exit_status.success() {
        panic!("git exited with error code: {:?}", exit_status.code());
    }
}

fn run_git(args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    run_command("git", args)
}

pub fn update_desmos_repo() {
    let full_path = |p: &str| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(p);

    info!("Update desmos repository...");
    run_git(&[
        "-C",
        full_path(DESMOS_DIR).to_str().unwrap(),
        "reset",
        "--hard",
        DESMOS_REV,
    ]);
}

pub fn try_clone_desmos_repo() {
    let full_path = |p: &str| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(p);

    if fs::metadata(full_path(DESMOS_DIR).to_str().unwrap()).is_ok() {
        return;
    };

    info!("Clone desmos repository...");
    run_git(&[
        "clone",
        DESMOS_REPO_URL,
        full_path(DESMOS_DIR).to_str().unwrap(),
    ]);
    run_git(&[
        "-C",
        full_path(DESMOS_DIR).to_str().unwrap(),
        "reset",
        "--hard",
        DESMOS_REV,
    ]);
}
