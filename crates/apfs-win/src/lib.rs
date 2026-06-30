#![forbid(unsafe_code)]

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WindowsMountPlanError {
    #[error("source path is empty")]
    EmptySource,
    #[error("mount point is empty")]
    EmptyMountPoint,
    #[error("mount point `{0}` is not a simple Windows drive-letter mount point such as X:")]
    InvalidDriveMountPoint(String),
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WindowsMountPlanStatus {
    ReadyReadOnly,
    Refused,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct WindowsMountPlan {
    pub schema_version: String,
    pub status: WindowsMountPlanStatus,
    pub source: String,
    pub mount_point: String,
    pub adapter: String,
    pub read_only: bool,
    pub winfsp_required: bool,
    pub allowed_operations: Vec<String>,
    pub refused_operations: Vec<String>,
    pub preflight_checks: Vec<String>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
    pub safety_note: String,
}

#[must_use]
pub fn plan_read_only_mount(source: &str, mount_point: &str) -> WindowsMountPlan {
    let mut errors = Vec::new();
    if source.trim().is_empty() {
        errors.push(WindowsMountPlanError::EmptySource.to_string());
    }
    if mount_point.trim().is_empty() {
        errors.push(WindowsMountPlanError::EmptyMountPoint.to_string());
    } else if !is_simple_drive_mount_point(mount_point) {
        errors.push(
            WindowsMountPlanError::InvalidDriveMountPoint(mount_point.to_owned()).to_string(),
        );
    }

    let status = if errors.is_empty() {
        WindowsMountPlanStatus::ReadyReadOnly
    } else {
        WindowsMountPlanStatus::Refused
    };

    WindowsMountPlan {
        schema_version: "0.18.0".to_owned(),
        status,
        source: source.to_owned(),
        mount_point: mount_point.to_owned(),
        adapter: "WinFsp user-mode filesystem adapter placeholder".to_owned(),
        read_only: true,
        winfsp_required: true,
        allowed_operations: vec![
            "getattr/stat".to_owned(),
            "lookup/open-readonly".to_owned(),
            "read".to_owned(),
            "readdir".to_owned(),
            "statfs".to_owned(),
        ],
        refused_operations: vec![
            "create".to_owned(),
            "write".to_owned(),
            "truncate".to_owned(),
            "delete".to_owned(),
            "rename".to_owned(),
            "setattr".to_owned(),
            "physical-device-write".to_owned(),
        ],
        preflight_checks: vec![
            "source opens read-only".to_owned(),
            "APFS inspect status is apfs_container_detected".to_owned(),
            "object-map resolver is available for the selected volume".to_owned(),
            "WinFsp runtime is installed and version-compatible".to_owned(),
            "mount point is unused".to_owned(),
            "all write callbacks are hard-refused".to_owned(),
        ],
        warnings: vec![
            "This is a planning/report scaffold, not a live WinFsp mount implementation yet."
                .to_owned(),
            "The first live adapter must remain read-only and user-mode.".to_owned(),
        ],
        errors,
        safety_note: "This plan never requests a write handle and never mutates APFS media."
            .to_owned(),
    }
}

#[must_use]
pub fn is_simple_drive_mount_point(mount_point: &str) -> bool {
    let bytes = mount_point.as_bytes();
    bytes.len() == 2 && bytes[1] == b':' && bytes[0].is_ascii_alphabetic()
}

#[cfg(test)]
mod tests {
    use super::{
        is_simple_drive_mount_point, plan_read_only_mount, windows_mount_packaging_report,
        WindowsMountPlanStatus,
    };

    #[test]
    fn validates_simple_drive_letters() {
        assert!(is_simple_drive_mount_point("X:"));
        assert!(is_simple_drive_mount_point("z:"));
        assert!(!is_simple_drive_mount_point(""));
        assert!(!is_simple_drive_mount_point("X:/"));
        assert!(!is_simple_drive_mount_point("C:/mount"));
    }

    #[test]
    fn produces_read_only_plan() {
        let plan = plan_read_only_mount("fixture.img", "X:");
        assert_eq!(plan.status, WindowsMountPlanStatus::ReadyReadOnly);
        assert!(plan.read_only);
        assert!(plan.refused_operations.iter().any(|op| op == "write"));
    }

    #[test]
    fn packaging_report_combines_mount_and_callback_scaffolds() {
        let report = windows_mount_packaging_report("fixture.img", "X:");
        assert_eq!(report.package_name, "apfs-win");
        assert!(report.mount_plan.read_only);
        assert!(report
            .callback_policy
            .callbacks
            .iter()
            .any(|callback| callback.callback == "Write" && callback.decision == "refuse"));
        assert!(report
            .smoke_checks
            .iter()
            .any(|check| check.contains("mount-plan")));
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ReadOnlyWinFspCallbackPolicy {
    pub schema_version: String,
    pub adapter: String,
    pub callbacks: Vec<WinFspCallbackContract>,
    pub safety_note: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct WinFspCallbackContract {
    pub callback: String,
    pub decision: String,
    pub note: String,
}

#[must_use]
pub fn winfsp_readonly_callback_matrix() -> ReadOnlyWinFspCallbackPolicy {
    let callbacks = [
        ("Init", "allow", "initialise read-only filesystem instance"),
        ("GetVolumeInfo", "allow", "report read-only volume metadata"),
        (
            "GetSecurityByName",
            "allow",
            "conservative read-only security metadata",
        ),
        ("Create", "refuse", "no file creation in read-only MVP"),
        ("Open", "allow_readonly", "open existing objects read-only"),
        ("Read", "allow", "read file data through apfs-vfs"),
        ("Write", "refuse", "all writes refused"),
        ("Flush", "allow_noop", "no APFS media mutation"),
        ("GetFileInfo", "allow", "stat/getattr mapping"),
        ("SetBasicInfo", "refuse", "metadata mutation refused"),
        ("SetFileSize", "refuse", "truncate/extend refused"),
        ("CanDelete", "refuse", "deletion refused"),
        ("Rename", "refuse", "rename refused"),
        (
            "ReadDirectory",
            "allow",
            "directory listing through apfs-vfs",
        ),
        (
            "GetReparsePoint",
            "allow_if_symlink",
            "symlink metadata where supported",
        ),
        ("SetReparsePoint", "refuse", "metadata mutation refused"),
        ("DeleteReparsePoint", "refuse", "metadata mutation refused"),
    ];
    ReadOnlyWinFspCallbackPolicy {
        schema_version: "0.22.0".to_owned(),
        adapter: "WinFsp".to_owned(),
        callbacks: callbacks
            .into_iter()
            .map(|(callback, decision, note)| WinFspCallbackContract {
                callback: callback.to_owned(),
                decision: decision.to_owned(),
                note: note.to_owned(),
            })
            .collect(),
        safety_note: "Contract scaffold only: live WinFsp mounting is not implemented yet; all write-like callbacks must be refused.".to_owned(),
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct WindowsMountPackagingReport {
    pub schema_version: String,
    pub package_name: String,
    pub package_version: String,
    pub mount_plan: WindowsMountPlan,
    pub callback_policy: ReadOnlyWinFspCallbackPolicy,
    pub smoke_checks: Vec<String>,
    pub packaging_notes: Vec<String>,
}

#[must_use]
pub fn windows_mount_packaging_report(
    source: &str,
    mount_point: &str,
) -> WindowsMountPackagingReport {
    WindowsMountPackagingReport {
        schema_version: "0.18.0".to_owned(),
        package_name: "apfs-win".to_owned(),
        package_version: env!("CARGO_PKG_VERSION").to_owned(),
        mount_plan: plan_read_only_mount(source, mount_point),
        callback_policy: winfsp_readonly_callback_matrix(),
        smoke_checks: vec![
            "apfs mount-plan --json --source <image> --mountpoint X:".to_owned(),
            "apfs winfsp-callback-matrix --json".to_owned(),
            "all write-like WinFsp callbacks are refused".to_owned(),
        ],
        packaging_notes: vec![
            "Adapter remains user-mode and read-only.".to_owned(),
            "No kernel driver, physical-device writes, or live mount lifecycle are implemented yet."
                .to_owned(),
        ],
    }
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WindowsWriteBetaGovernanceStatus {
    BlockedUntilAcceptedWriteLabEvidence,
    PlanningOnly,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct WindowsWriteBetaGovernanceReport {
    pub schema_version: String,
    pub track: String,
    pub status: WindowsWriteBetaGovernanceStatus,
    pub prerequisites: Vec<String>,
    pub required_gates: Vec<String>,
    pub rollback_plan: Vec<String>,
    pub refused_operations: Vec<String>,
    pub safety_constraints: Vec<String>,
    pub evidence_notes: Vec<String>,
}

#[must_use]
pub fn windows_write_beta_governance_report() -> WindowsWriteBetaGovernanceReport {
    WindowsWriteBetaGovernanceReport {
        schema_version: "0.18.0".to_owned(),
        track: "M-133".to_owned(),
        status: WindowsWriteBetaGovernanceStatus::BlockedUntilAcceptedWriteLabEvidence,
        prerequisites: vec![
            "accepted image-only write-lab evidence (M-132)".to_owned(),
            "maintainer approval for any write beta".to_owned(),
            "production claim guard passes before release claims".to_owned(),
        ],
        required_gates: vec![
            "windows_write_governance_audit".to_owned(),
            "write_operations_refused".to_owned(),
            "production_claim_guard".to_owned(),
        ],
        rollback_plan: vec![
            "disable write-beta feature flags".to_owned(),
            "revert to the read-only mount path".to_owned(),
            "publish rollback notes before any public beta claim".to_owned(),
        ],
        refused_operations: vec![
            "create".to_owned(),
            "write".to_owned(),
            "truncate".to_owned(),
            "delete".to_owned(),
            "rename".to_owned(),
            "setattr".to_owned(),
            "physical-device-write".to_owned(),
        ],
        safety_constraints: vec![
            "read-only default until accepted write-lab evidence exists".to_owned(),
            "no physical-device writes".to_owned(),
            "no encryption bypass".to_owned(),
            "no live write beta without maintainer approval".to_owned(),
        ],
        evidence_notes: vec![
            "governance-only scaffold; it does not enable a Windows write beta".to_owned(),
            "the beta remains blocked until image-only crash evidence is accepted".to_owned(),
        ],
    }
}

#[cfg(test)]
mod governance_tests {
    use super::{windows_write_beta_governance_report, WindowsWriteBetaGovernanceStatus};

    #[test]
    fn governance_report_stays_blocked_until_write_lab_evidence() {
        let report = windows_write_beta_governance_report();

        assert_eq!(
            report.status,
            WindowsWriteBetaGovernanceStatus::BlockedUntilAcceptedWriteLabEvidence
        );
        assert!(report
            .prerequisites
            .iter()
            .any(|line| line.contains("accepted image-only write-lab evidence")));
        assert!(report
            .required_gates
            .iter()
            .any(|line| line == "production_claim_guard"));
        assert!(report
            .safety_constraints
            .iter()
            .any(|line| line.contains("no live write beta without maintainer approval")));
    }
}
