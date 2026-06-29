#![forbid(unsafe_code)]

use apfs_core::{inspect_bytes, InspectStatus};

#[test]
fn synthetic_nxsb_fixture_is_detected() {
    let bytes = include_bytes!("../../../fixtures/synthetic-nxsb-block0.bin");
    let report = inspect_bytes(bytes);
    assert_eq!(report.status, InspectStatus::ApfsContainerDetected);
}

#[test]
fn non_apfs_fixture_reports_not_apfs_or_refusal() {
    let bytes = include_bytes!("../../../fixtures/example-non-apfs.bin");
    let report = inspect_bytes(bytes);
    assert!(matches!(report.status, InspectStatus::NotApfs | InspectStatus::Refused));
}
