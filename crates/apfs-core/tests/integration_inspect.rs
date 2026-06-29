#![forbid(unsafe_code)]

use apfs_core::{inspect_bytes, InspectStatus};

fn synthetic_nxsb() -> Vec<u8> {
    let mut block = vec![0u8; 4096];
    block[32..36].copy_from_slice(b"NXSB");
    block[36..40].copy_from_slice(&4096u32.to_le_bytes());
    block[40..48].copy_from_slice(&16u64.to_le_bytes());
    block[180..184].copy_from_slice(&1u32.to_le_bytes());
    block
}

#[test]
fn inspect_reports_apfs_or_checksum_refusal_for_synthetic_nxsb() {
    let report = inspect_bytes(&synthetic_nxsb());
    assert!(matches!(report.status, InspectStatus::ApfsContainerDetected | InspectStatus::Refused));
    assert!(report.safety.read_only);
}

#[test]
fn inspect_reports_not_apfs_for_zero_block() {
    let report = inspect_bytes(&vec![0u8; 4096]);
    assert_eq!(report.status, InspectStatus::NotApfs);
}
