#![forbid(unsafe_code)]

use assert_cmd::Command;
use predicates::str::contains;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn inspect_json_reports_not_apfs_for_plain_file() {
    let mut file = NamedTempFile::new().expect("temp file");
    writeln!(file, "not apfs").expect("write fixture");
    let mut cmd = Command::cargo_bin("apfs").expect("apfs binary");
    cmd.arg("inspect").arg(file.path()).arg("--json");
    cmd.assert().success().stdout(contains("schema_version"));
}
