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
