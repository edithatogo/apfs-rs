#![forbid(unsafe_code)]

/// Android support mode options for future integration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AndroidAccessMode {
    LibraryOnly,
    StorageAccessFramework,
    RootedFuseOptional,
}

/// Read-only Android access readiness.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AndroidReadiness {
    pub default_mode: AndroidAccessMode,
    pub raw_block_device_assumed: bool,
    pub write_support: bool,
}

#[must_use]
pub fn android_readiness() -> AndroidReadiness {
    AndroidReadiness {
        default_mode: AndroidAccessMode::LibraryOnly,
        raw_block_device_assumed: false,
        write_support: false,
    }
}
