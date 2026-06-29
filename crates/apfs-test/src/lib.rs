#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::{io::Read, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FixtureManifest {
    pub schema_version: String,
    pub fixture_id: String,
    pub source_type: String,
    pub created_with: CreatedWith,
    pub apfs_features: ApfsFeatureManifest,
    pub expected_artifacts: ExpectedArtifacts,
    pub image_sha256: String,
    pub capability_ids: Vec<String>,
    pub redaction: RedactionManifest,
    pub safe_to_commit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreatedWith {
    pub tool: String,
    pub size: Option<String>,
    pub volume_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApfsFeatureManifest {
    pub encrypted: bool,
    pub compressed: bool,
    pub snapshots: bool,
    pub case_sensitive: bool,
    pub volume_group: bool,
    pub fusion: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExpectedArtifacts {
    pub image: String,
    pub file_hashes: Option<String>,
    pub macos_oracle_redacted: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RedactionManifest {
    pub contains_personal_data: bool,
    pub contains_secret_material: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestValidation {
    pub valid: bool,
    pub issues: Vec<String>,
}

pub fn helper_crate_ready() -> bool {
    true
}

pub fn load_fixture_manifest(path: impl AsRef<Path>) -> Result<FixtureManifest, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    load_fixture_manifest_from_reader(&mut file)
}

pub fn load_fixture_manifest_from_reader(reader: &mut dyn Read) -> Result<FixtureManifest, std::io::Error> {
    let mut text = String::new();
    reader.read_to_string(&mut text)?;
    serde_json::from_str(&text).map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))
}

pub fn validate_fixture_manifest(manifest: &FixtureManifest) -> ManifestValidation {
    let mut issues = Vec::new();
    if manifest.fixture_id.trim().is_empty() {
        issues.push("fixture_id must not be empty".to_owned());
    }
    if manifest.redaction.contains_personal_data {
        issues.push("fixture is marked as containing personal data".to_owned());
    }
    if manifest.redaction.contains_secret_material {
        issues.push("fixture is marked as containing secret material".to_owned());
    }
    if !manifest.apfs_features.encrypted && manifest.capability_ids.iter().any(|id| id.contains("R2")) {
        issues.push("unencrypted fixture should not claim encryption-read capability coverage".to_owned());
    }
    ManifestValidation { valid: issues.is_empty(), issues }
}
