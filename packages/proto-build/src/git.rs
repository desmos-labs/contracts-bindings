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

pub fn update_repo(name: &str, dir: &str, rev: &str) {
    let full_path = |p: &str| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(p);

    info!("Update {} repository...", name);
    run_git(&[
        "-C",
        full_path(dir).to_str().unwrap(),
        "reset",
        "--hard",
        rev,
    ]);
}

pub fn try_clone_repo(name: &str, repo: &str, dir: &str, rev: &str) {
    let full_path = |p: &str| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(p);

    if fs::metadata(full_path(dir).to_str().unwrap()).is_ok() {
        return;
    };

    info!("Clone {} repository...", name);
    run_git(&[
        "clone",
        repo,
        full_path(dir).to_str().unwrap(),
    ]);
    run_git(&[
        "-C",
        full_path(dir).to_str().unwrap(),
        "reset",
        "--hard",
        rev,
    ]);
}
