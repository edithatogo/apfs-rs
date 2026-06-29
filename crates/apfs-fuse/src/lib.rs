#![forbid(unsafe_code)]

/// Read-only FUSE-style adapter readiness status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuseAdapterReadiness {
    pub adapter_name: &'static str,
    pub read_only: bool,
    pub mount_lifecycle_implemented: bool,
    pub platform_notes: &'static [&'static str],
}

/// Return the current readiness state for future FUSE-compatible adapters.
#[must_use]
pub fn fuse_adapter_readiness() -> FuseAdapterReadiness {
    FuseAdapterReadiness {
        adapter_name: "apfs-fuse-readonly-readiness",
        read_only: true,
        mount_lifecycle_implemented: false,
        platform_notes: &[
            "Linux/FUSE support is not production implemented yet.",
            "macOS/macFUSE support is planned as a parity/testing adapter.",
            "ChromeOS support is expected to begin with CLI extraction and Linux-container/FUSE modes where available.",
        ],
    }
}
