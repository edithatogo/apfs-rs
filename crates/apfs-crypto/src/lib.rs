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
    pub user_supplied_unlock_material_supported: bool,
    pub password_recovery_supported: bool,
    pub hardware_bound_unlock_supported: bool,
    pub logs_secret_material: bool,
}

#[must_use]
pub fn crypto_readiness() -> CryptoReadiness {
    CryptoReadiness {
        status: CryptoReadinessStatus::PolicyOnly,
        user_supplied_unlock_material_supported: false,
        password_recovery_supported: false,
        hardware_bound_unlock_supported: false,
        logs_secret_material: false,
    }
}

#[cfg(test)]
mod tests {
    use super::{crypto_readiness, CryptoReadinessStatus};

    #[test]
    fn readiness_defaults_to_policy_only_and_refuses_secret_material() {
        let report = crypto_readiness();

        assert_eq!(report.status, CryptoReadinessStatus::PolicyOnly);
        assert!(!report.user_supplied_unlock_material_supported);
        assert!(!report.password_recovery_supported);
        assert!(!report.hardware_bound_unlock_supported);
        assert!(!report.logs_secret_material);
    }
}
