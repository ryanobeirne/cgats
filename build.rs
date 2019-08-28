use rustc_tools_util::get_commit_hash;
use clap::crate_version;

fn main() {
    let mut crate_version = crate_version!().to_owned();
    let commit_hash = get_commit_hash().unwrap_or_default();

    if !commit_hash.is_empty() {
        crate_version = format!("{} {}", crate_version, commit_hash);
    } else {
        println!("cargo:warning={}", "Git commit not found!")
    }

    println!("cargo:rustc-env=CRATE_VERSION={}", crate_version);
}