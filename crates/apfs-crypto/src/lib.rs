#![forbid(unsafe_code)]

/// Conservative state for software-encryption support.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CryptoReadinessStatus {
    NotImplemented,
    PolicyOnly,
}

/// Readiness report for future software-encryption read support.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CryptoReadiness {
    pub status: CryptoReadinessStatus,
    pub password_recovery_supported: bool,
    pub hardware_bound_unlock_supported: bool,
    pub logs_secret_material: bool,
}

#[must_use]
pub fn crypto_readiness() -> CryptoReadiness {
    CryptoReadiness {
        status: CryptoReadinessStatus::PolicyOnly,
        password_recovery_supported: false,
        hardware_bound_unlock_supported: false,
        logs_secret_material: false,
    }
}
