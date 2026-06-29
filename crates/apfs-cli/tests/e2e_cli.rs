#![forbid(unsafe_code)]

use assert_cmd::Command;
use predicates::str::contains;
use std::io::Write;
use tempfile::NamedTempFile;

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
