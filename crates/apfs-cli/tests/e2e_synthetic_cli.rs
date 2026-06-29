#![forbid(unsafe_code)]


use std::path::PathBuf;
use std::process::Command;

#[test]
fn inspect_non_apfs_fixture_returns_json() {
    let exe = env!("CARGO_BIN_EXE_apfs");
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_path = manifest_dir.join("../../fixtures/example-non-apfs.bin");
    let output = Command::new(exe)
        .args(["inspect", "--json"])
        .arg(fixture_path)
        .output()
        .expect("run apfs inspect");
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert!(json.get("status").is_some());
}
