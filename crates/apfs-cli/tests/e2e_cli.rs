#![forbid(unsafe_code)]

use assert_cmd::Command;
use predicates::str::contains;
use std::io::Write;
use tempfile::{NamedTempFile, TempDir};

#[test]
fn version_json_reports_dynamic_metadata() {
    let mut cmd = Command::cargo_bin("apfs").expect("apfs binary");
    cmd.arg("version").arg("--json");
    let output = cmd.assert().success().get_output().stdout.clone();
    let json: serde_json::Value = serde_json::from_slice(&output).expect("version json");
    assert_eq!(json["workspace_version"], env!("APFS_RS_VERSION"));
    assert!(json["git_sha"]
        .as_str()
        .is_some_and(|value| !value.is_empty()));
    assert_eq!(json["writes_to_apfs_media"], false);
}

#[test]
fn inspect_json_reports_not_apfs_for_plain_file() {
    let mut file = NamedTempFile::new().expect("temp file");
    writeln!(file, "not apfs").expect("write fixture");
    let mut cmd = Command::cargo_bin("apfs").expect("apfs binary");
    cmd.arg("inspect").arg(file.path()).arg("--json");
    cmd.assert().success().stdout(contains("schema_version"));
}

#[test]
fn inspect_logging_is_redacted_and_keeps_json_stdout() {
    let mut file = NamedTempFile::new().expect("temp file");
    writeln!(file, "not apfs").expect("write fixture");
    let mut cmd = Command::cargo_bin("apfs").expect("apfs binary");
    cmd.arg("--log-level")
        .arg("info")
        .arg("inspect")
        .arg(file.path())
        .arg("--json");
    let output = cmd.assert().success().get_output().clone();
    let stdout_json: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("inspect stdout json");
    assert!(stdout_json.get("schema_version").is_some());

    let stderr = String::from_utf8(output.stderr).expect("stderr utf8");
    assert!(stderr.contains("\"event\":\"cli.start\""));
    assert!(stderr.contains("\"event\":\"image.open_read_only\""));
    assert!(stderr.contains("\"source_name_redacted\""));
    assert!(!stderr.contains(file.path().to_string_lossy().as_ref()));
}

#[test]
fn stat_json_reports_mapped_filesystem_metadata() {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_path = manifest_dir.join("../../fixtures/synthetic-directory-listing.img");
    let mut cmd = Command::cargo_bin("apfs").expect("apfs binary");
    cmd.arg("stat")
        .arg(fixture_path)
        .arg("--name")
        .arg("hello.txt")
        .arg("--json");
    let output = cmd.assert().success().get_output().stdout.clone();
    let json: serde_json::Value = serde_json::from_slice(&output).expect("stat json");
    assert_eq!(json["status"], "found");
    assert_eq!(json["metadata"]["name"], "hello.txt");
    assert_eq!(json["metadata"]["object_id"], 4000);
    assert_eq!(json["metadata"]["logical_size"], 43);
    assert_eq!(json["metadata"]["physical_block"], 40);
}

#[test]
fn stat_json_reports_directory_entries_without_physical_blocks() {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_path = manifest_dir.join("../../fixtures/synthetic-directory-listing.img");
    let mut cmd = Command::cargo_bin("apfs").expect("apfs binary");
    cmd.arg("stat")
        .arg(fixture_path)
        .arg("--name")
        .arg("Documents")
        .arg("--json");
    let output = cmd.assert().success().get_output().stdout.clone();
    let json: serde_json::Value = serde_json::from_slice(&output).expect("stat json");
    assert_eq!(json["status"], "found");
    assert_eq!(json["metadata"]["name"], "Documents");
    assert_eq!(json["metadata"]["object_id"], 4001);
    assert_eq!(json["metadata"]["logical_size"], 0);
    assert_eq!(json["metadata"]["physical_block"], serde_json::Value::Null);
    assert_eq!(json["metadata"]["has_physical_block"], false);
}

#[test]
fn extract_json_writes_full_file_from_synthetic_extents() {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_path = manifest_dir.join("../../fixtures/synthetic-file-extract.img");
    let dest = TempDir::new().expect("temp dir");
    let mut cmd = Command::cargo_bin("apfs").expect("apfs binary");
    cmd.arg("extract")
        .arg(fixture_path)
        .arg("--name")
        .arg("hello.txt")
        .arg("--dest")
        .arg(dest.path())
        .arg("--json");
    let output = cmd.assert().success().get_output().stdout.clone();
    let json: serde_json::Value = serde_json::from_slice(&output).expect("extract json");
    assert_eq!(json["status"], "written");
    assert_eq!(json["requested_name"], "hello.txt");
    assert_eq!(json["wrote_bytes"], 43);
    let extracted = std::fs::read(dest.path().join("hello.txt")).expect("extracted file");
    assert_eq!(extracted, b"Hello from APFS-RS synthetic file preview!\n");
}

#[test]
fn extract_rejects_path_traversal_names() {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_path = manifest_dir.join("../../fixtures/synthetic-file-extract.img");
    let dest = TempDir::new().expect("temp dir");
    let mut cmd = Command::cargo_bin("apfs").expect("apfs binary");
    cmd.arg("extract")
        .arg(fixture_path)
        .arg("--name")
        .arg("../escape")
        .arg("--dest")
        .arg(dest.path())
        .arg("--json");
    cmd.assert()
        .failure()
        .stderr(contains("unsafe synthetic extraction name"));
}
