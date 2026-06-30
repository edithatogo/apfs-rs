#![forbid(unsafe_code)]

/// Scope allowed for future write-lab experiments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WriteLabScope {
    DisposableImageOnly,
}

/// Future operation classes that must pass crash injection before any beta.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlannedLabOperation {
    CreateFile,
    ReplaceFileContents,
    DeleteFile,
    CreateDirectory,
    Rename,
}

use serde::Serialize;

/// Current write-lab readiness. This is intentionally a plan, not an implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriteLabReadiness {
    pub scope: WriteLabScope,
    pub physical_media_enabled: bool,
    pub crash_injection_required: bool,
    pub planned_operations: &'static [PlannedLabOperation],
}

#[must_use]
pub fn write_lab_readiness() -> WriteLabReadiness {
    WriteLabReadiness {
        scope: WriteLabScope::DisposableImageOnly,
        physical_media_enabled: false,
        crash_injection_required: true,
        planned_operations: &[
            PlannedLabOperation::CreateFile,
            PlannedLabOperation::ReplaceFileContents,
            PlannedLabOperation::DeleteFile,
            PlannedLabOperation::CreateDirectory,
            PlannedLabOperation::Rename,
        ],
    }
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WriteLabEvidenceStatus {
    DisposableImageOnly,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct WriteLabEvidenceReport {
    pub schema_version: String,
    pub track: String,
    pub status: WriteLabEvidenceStatus,
    pub scope: String,
    pub physical_media_enabled: bool,
    pub crash_injection_required: bool,
    pub planned_operations: Vec<String>,
    pub safety_constraints: Vec<String>,
    pub evidence_notes: Vec<String>,
}

#[must_use]
pub fn write_lab_evidence_report() -> WriteLabEvidenceReport {
    let readiness = write_lab_readiness();
    let planned_operations = readiness
        .planned_operations
        .iter()
        .map(|operation| match operation {
            PlannedLabOperation::CreateFile => "create-file".to_owned(),
            PlannedLabOperation::ReplaceFileContents => "replace-file-contents".to_owned(),
            PlannedLabOperation::DeleteFile => "delete-file".to_owned(),
            PlannedLabOperation::CreateDirectory => "create-directory".to_owned(),
            PlannedLabOperation::Rename => "rename".to_owned(),
        })
        .collect();

    WriteLabEvidenceReport {
        schema_version: "0.1.0".to_owned(),
        track: "M-132".to_owned(),
        status: WriteLabEvidenceStatus::DisposableImageOnly,
        scope: match readiness.scope {
            WriteLabScope::DisposableImageOnly => "disposable-image-only".to_owned(),
        },
        physical_media_enabled: readiness.physical_media_enabled,
        crash_injection_required: readiness.crash_injection_required,
        planned_operations,
        safety_constraints: vec![
            "no APFS media mutation".to_owned(),
            "no physical-device writes".to_owned(),
            "no encrypted/sealed/damaged/unknown-feature writes".to_owned(),
            "crash injection required before any beta".to_owned(),
        ],
        evidence_notes: vec![
            "disposable image planning only; no write path is implemented".to_owned(),
            "the report is evidence scaffolding for future write-governance review".to_owned(),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::{write_lab_evidence_report, WriteLabEvidenceStatus};

    #[test]
    fn evidence_report_stays_image_only_and_requires_crash_injection() {
        let report = write_lab_evidence_report();

        assert_eq!(report.status, WriteLabEvidenceStatus::DisposableImageOnly);
        assert!(!report.physical_media_enabled);
        assert!(report.crash_injection_required);
        assert!(report
            .safety_constraints
            .iter()
            .any(|line| line.contains("no physical-device writes")));
        assert!(report
            .evidence_notes
            .iter()
            .any(|line| line.contains("no write path is implemented")));
    }
}
