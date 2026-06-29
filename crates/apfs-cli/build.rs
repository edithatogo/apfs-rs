#![forbid(unsafe_code)]

use std::{fs, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=../../VERSION");
    println!("cargo:rerun-if-changed=../../.git/HEAD");
    println!("cargo:rerun-if-changed=../../.git/refs/heads/main");
    println!("cargo:rerun-if-changed=../../.git/packed-refs");
    println!("cargo:rerun-if-env-changed=GITHUB_SHA");

    let version = fs::read_to_string("../../VERSION")
        .map(|text| text.trim().to_owned())
        .unwrap_or_else(|_| env!("CARGO_PKG_VERSION").to_owned());
    println!("cargo:rustc-env=APFS_RS_VERSION={version}");
    println!(
        "cargo:rustc-env=APFS_RS_TARGET={}",
        std::env::var("TARGET").unwrap_or_else(|_| "unknown".to_owned())
    );
    println!(
        "cargo:rustc-env=APFS_RS_PROFILE={}",
        std::env::var("PROFILE").unwrap_or_else(|_| "unknown".to_owned())
    );

    let git_sha = std::env::var("GITHUB_SHA")
        .ok()
        .map(|sha| sha.chars().take(12).collect::<String>())
        .filter(|sha| !sha.is_empty())
        .or_else(|| {
            Command::new("git")
                .args(["rev-parse", "--short=12", "HEAD"])
                .output()
                .ok()
                .filter(|output| output.status.success())
                .and_then(|output| String::from_utf8(output.stdout).ok())
                .map(|text| text.trim().to_owned())
                .filter(|text| !text.is_empty())
        })
        .unwrap_or_else(|| "unknown".to_owned());
    println!("cargo:rustc-env=APFS_RS_GIT_SHA={git_sha}");
}
