#![forbid(unsafe_code)]

use apfs_crypto::{crypto_readiness, CryptoReadinessStatus};
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FeatureReadinessStatus {
    ScaffoldedReadOnly,
    PolicyOnly,
    UnsupportedUntilRealFixture,
    BlockedUntilDedicatedSpec,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct FeatureReadinessReport {
    pub schema_version: String,
    pub feature: String,
    pub status: FeatureReadinessStatus,
    pub implemented_scope: Vec<String>,
    pub missing_production_steps: Vec<String>,
    pub safety_constraints: Vec<String>,
    pub next_track: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct UnicodeCasePolicyReport {
    pub schema_version: String,
    pub input_name: String,
    pub case_sensitive_volume: bool,
    pub safe_for_host_path: bool,
    pub normalized_for_display: String,
    pub lookup_policy: String,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct MetadataFeatureReport {
    pub schema_version: String,
    pub feature_family: String,
    pub synthetic_support: bool,
    pub production_read_support: bool,
    pub production_write_support: bool,
    pub modeled_fields: Vec<String>,
    pub missing_fields: Vec<String>,
    pub safety_constraints: Vec<String>,
}

#[must_use]
pub fn analyze_unicode_case_policy(
    name: &str,
    case_sensitive_volume: bool,
) -> UnicodeCasePolicyReport {
    let mut warnings = Vec::new();
    let mut errors = Vec::new();

    if name.is_empty() {
        errors.push("empty APFS names are not valid for lookup".to_owned());
    }
    if name.contains('\0') {
        errors.push("NUL bytes are never allowed in APFS-RS host-facing paths".to_owned());
    }
    if name.contains('/') || name.contains('\\') {
        errors.push(
            "this policy check accepts one path component only; separators are refused".to_owned(),
        );
    }
    if name == "." || name == ".." {
        errors.push("dot and parent components are refused for host extraction safety".to_owned());
    }
    if name.chars().any(char::is_control) {
        warnings
            .push("control characters require explicit escaping in host-facing output".to_owned());
    }
    if !name.is_ascii() {
        warnings.push(
            "Unicode normalization/case-folding is policy-only until real APFS fixture validation"
                .to_owned(),
        );
    }
    if !case_sensitive_volume {
        warnings.push("case-insensitive lookup is not production-ready until APFS normalization rules are validated".to_owned());
    }

    UnicodeCasePolicyReport {
        schema_version: "0.19.0".to_owned(),
        input_name: name.to_owned(),
        case_sensitive_volume,
        safe_for_host_path: errors.is_empty(),
        normalized_for_display: name.escape_default().to_string(),
        lookup_policy: if case_sensitive_volume {
            "byte-preserving component comparison until production APFS normalization is validated"
                .to_owned()
        } else {
            "diagnostic-only case-insensitive policy; production lookup remains blocked pending real fixtures".to_owned()
        },
        warnings,
        errors,
    }
}

#[must_use]
#[allow(clippy::too_many_lines)]
pub fn feature_readiness(feature: &str) -> FeatureReadinessReport {
    let normalized = feature.to_ascii_lowercase().replace('_', "-");
    match normalized.as_str() {
        "unicode" | "case" | "unicode-case" | "case-policy" => FeatureReadinessReport {
            schema_version: "0.19.0".to_owned(),
            feature: "unicode-case-policy".to_owned(),
            status: FeatureReadinessStatus::PolicyOnly,
            implemented_scope: vec![
                "host-facing one-component path safety policy".to_owned(),
                "case-sensitive vs case-insensitive readiness report".to_owned(),
            ],
            missing_production_steps: vec![
                "validate APFS normalization and case-folding against macOS fixtures".to_owned(),
                "add collision handling for case-insensitive volumes".to_owned(),
            ],
            safety_constraints: vec![
                "no path traversal".to_owned(),
                "no lossy silent normalization".to_owned(),
            ],
            next_track: "0034-unicode-case-policy".to_owned(),
        },
        "xattr" | "xattrs" | "resource-fork" | "resource-forks" => FeatureReadinessReport {
            schema_version: "0.19.0".to_owned(),
            feature: "xattrs-resource-forks".to_owned(),
            status: FeatureReadinessStatus::ScaffoldedReadOnly,
            implemented_scope: vec![
                "metadata capability scaffold".to_owned(),
                "read-only reporting contract".to_owned(),
            ],
            missing_production_steps: vec![
                "parse production APFS extended-field records".to_owned(),
                "extract xattr/resource-fork bytes from real fixtures".to_owned(),
                "map resource fork output safely for Windows users".to_owned(),
            ],
            safety_constraints: vec![
                "read-only".to_owned(),
                "no host alternate-data-stream writes by default".to_owned(),
            ],
            next_track: "0035-xattr-resource-fork-readiness".to_owned(),
        },
        "sparse" | "clone" | "clones" | "sparse-clone" => FeatureReadinessReport {
            schema_version: "0.19.0".to_owned(),
            feature: "sparse-clones".to_owned(),
            status: FeatureReadinessStatus::ScaffoldedReadOnly,
            implemented_scope: vec![
                "extent-feature reporting contract".to_owned(),
                "read-only safety constraints".to_owned(),
            ],
            missing_production_steps: vec![
                "decode APFS extent records from production filesystem trees".to_owned(),
                "distinguish holes from allocated zeroed extents".to_owned(),
                "report shared physical extents without double-counting".to_owned(),
            ],
            safety_constraints: vec![
                "stream extraction only".to_owned(),
                "bounded allocation".to_owned(),
                "do not materialize sparse holes unless requested".to_owned(),
            ],
            next_track: "0036-sparse-clone-readiness".to_owned(),
        },
        "compression" | "compressed" => FeatureReadinessReport {
            schema_version: "0.19.0".to_owned(),
            feature: "compression".to_owned(),
            status: FeatureReadinessStatus::UnsupportedUntilRealFixture,
            implemented_scope: vec![
                "algorithm readiness catalog".to_owned(),
                "decompression safety constraints".to_owned(),
            ],
            missing_production_steps: vec![
                "add APFS decmpfs metadata parser".to_owned(),
                "evaluate zlib/lzvn/lzfse crates with licence review".to_owned(),
                "add decompression bomb guards".to_owned(),
                "verify byte hashes against macOS fixtures".to_owned(),
            ],
            safety_constraints: vec![
                "bounded output size".to_owned(),
                "streaming where possible".to_owned(),
                "unknown compression method refuses file read".to_owned(),
            ],
            next_track: "0037-compression-readiness".to_owned(),
        },
        "snapshot" | "snapshots" | "roles" | "volume-roles" => FeatureReadinessReport {
            schema_version: "0.19.0".to_owned(),
            feature: "snapshots-volume-roles".to_owned(),
            status: FeatureReadinessStatus::UnsupportedUntilRealFixture,
            implemented_scope: vec![
                "readiness report".to_owned(),
                "role/snapshot safety constraints".to_owned(),
            ],
            missing_production_steps: vec![
                "parse production volume role flags".to_owned(),
                "discover snapshot metadata trees".to_owned(),
                "mount/extract selected snapshots read-only".to_owned(),
            ],
            safety_constraints: vec![
                "read-only snapshots only".to_owned(),
                "no snapshot creation/deletion".to_owned(),
            ],
            next_track: "0038-snapshot-role-readiness".to_owned(),
        },
        "encryption" | "crypto" => FeatureReadinessReport {
            schema_version: "0.19.0".to_owned(),
            feature: "software-encryption-read".to_owned(),
            status: FeatureReadinessStatus::BlockedUntilDedicatedSpec,
            implemented_scope: vec![
                match crypto_readiness().status {
                    CryptoReadinessStatus::PolicyOnly => {
                        "policy-only encryption readiness report".to_owned()
                    }
                    CryptoReadinessStatus::NotImplemented => {
                        "encryption readiness remains unimplemented".to_owned()
                    }
                },
                "explicit refusal for password recovery, key extraction, and hardware-bound unlock"
                    .to_owned(),
                "host-facing diagnostics never log secret material".to_owned(),
            ],
            missing_production_steps: vec![
                "accepted crypto/key-handling spec".to_owned(),
                "security review".to_owned(),
                "secret redaction tests".to_owned(),
                "software-encrypted fixture".to_owned(),
            ],
            safety_constraints: vec![
                "no password recovery".to_owned(),
                "no key extraction".to_owned(),
                "no hardware-bound encryption bypass".to_owned(),
                "no secret material in logs".to_owned(),
            ],
            next_track: "future-software-encryption-read".to_owned(),
        },
        _ => FeatureReadinessReport {
            schema_version: "0.19.0".to_owned(),
            feature: normalized,
            status: FeatureReadinessStatus::PolicyOnly,
            implemented_scope: vec!["unknown feature report scaffold".to_owned()],
            missing_production_steps: vec![
                "classify feature and create Codev/Conductor track".to_owned()
            ],
            safety_constraints: vec![
                "unsupported features default to read-only diagnostic behaviour".to_owned(),
            ],
            next_track: "unclassified-feature".to_owned(),
        },
    }
}

#[must_use]
pub fn metadata_feature_report(feature_family: &str) -> MetadataFeatureReport {
    let readiness = feature_readiness(feature_family);
    let (modeled_fields, missing_fields) = match readiness.feature.as_str() {
        "xattrs-resource-forks" => (
            vec![
                "name".to_owned(),
                "logical_size".to_owned(),
                "storage_policy".to_owned(),
            ],
            vec![
                "extended-field record offsets".to_owned(),
                "resource fork payload extents".to_owned(),
                "Finder info semantics".to_owned(),
            ],
        ),
        "sparse-clones" => (
            vec![
                "logical_size".to_owned(),
                "physical_size".to_owned(),
                "shared_extent_hint".to_owned(),
            ],
            vec![
                "hole map".to_owned(),
                "clone lineage".to_owned(),
                "physical extent reference counts".to_owned(),
            ],
        ),
        _ => (
            vec!["feature_name".to_owned(), "readiness".to_owned()],
            readiness.missing_production_steps.clone(),
        ),
    };

    MetadataFeatureReport {
        schema_version: "0.19.0".to_owned(),
        feature_family: readiness.feature,
        synthetic_support: true,
        production_read_support: false,
        production_write_support: false,
        modeled_fields,
        missing_fields,
        safety_constraints: readiness.safety_constraints,
    }
}

#[cfg(test)]
mod tests {
    use super::{analyze_unicode_case_policy, feature_readiness, FeatureReadinessStatus};

    #[test]
    fn refuses_path_separators_in_name_policy() {
        let report = analyze_unicode_case_policy("../secret", true);
        assert!(!report.safe_for_host_path);
        assert!(!report.errors.is_empty());
    }

    #[test]
    fn compression_is_not_claimed_supported() {
        let report = feature_readiness("compression");
        assert_eq!(
            report.status,
            FeatureReadinessStatus::UnsupportedUntilRealFixture
        );
    }

    #[test]
    fn encryption_feature_stays_policy_only_and_redacts_secrets() {
        let report = feature_readiness("encryption");

        assert_eq!(
            report.status,
            FeatureReadinessStatus::BlockedUntilDedicatedSpec
        );
        assert!(report
            .implemented_scope
            .iter()
            .any(|line| line.contains("policy-only encryption readiness report")));
        assert!(report
            .safety_constraints
            .iter()
            .any(|line| line.contains("no secret material in logs")));
    }
}
