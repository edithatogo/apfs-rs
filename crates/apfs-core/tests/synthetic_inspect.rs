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
    assert!(matches!(
        report.status,
        InspectStatus::NotApfs | InspectStatus::Refused
    ));
}

#[test]
fn checkpoint_ring_fixture_selects_newest_valid_candidate() {
    let bytes = include_bytes!("../../../fixtures/synthetic-checkpoint-ring.img");
    let report = inspect_bytes(bytes);
    assert_eq!(report.status, InspectStatus::ApfsContainerDetected);
    let scan = report
        .checkpoint_scan
        .as_ref()
        .expect("checkpoint scan should be present for the fixture");
    assert_eq!(scan.candidates.len(), 2);
    assert_eq!(scan.latest_valid_xid, Some(15));
    assert_eq!(scan.latest_valid_block, Some(3));
}
