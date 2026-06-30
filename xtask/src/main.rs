#![forbid(unsafe_code)]

use anyhow::{Context, Result};
use apfs_win::windows_write_beta_governance_report;
use apfs_write_lab::write_lab_evidence_report;
use clap::{Parser, Subcommand};
use serde_json::Value as JsonValue;
use std::{
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
    process::Command as ProcessCommand,
};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "xtask")]
#[command(about = "APFS-RS repository automation tasks")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Validate Codev registries against JSON schemas and cross-links.
    RegistryCheck,
    /// Validate Conductor context-management files and historical tracks.
    ConductorCheck,
    /// Validate Conductor track review/archive closeout evidence.
    ConductorTrackCloseout,
    /// Run lightweight safety policy checks.
    SafetyCheck,
    /// Run Python/static precompile checks for environments without Rust compiler access.
    PrecompileCheck,
    /// Validate synthetic fixture structure and expected parser-development semantics.
    SyntheticOracleCheck,
    /// Generate `TRACEABILITY.md`.
    TraceabilityMatrix,
    /// Generate `LOOP_DASHBOARD.md` from `REMAINING_ELEMENTS.yaml`.
    LoopDashboard,
    /// Validate CLI command variants, match arms, and docs.
    CliContractCheck,
    /// Validate Codev/Conductor/requirements/remaining-ledger alignment.
    ContextIntegrityCheck,
    /// Run all available cargoless preflight checks.
    ReleasePreflight {
        /// Regenerate `SHA256SUMS.txt`.
        #[arg(long)]
        write_manifest: bool,
    },
    /// Build a redacted diagnostics bundle from JSON reports.
    DiagnosticsBundle {
        /// Output directory.
        #[arg(long)]
        out: PathBuf,
        /// JSON report files to include.
        reports: Vec<PathBuf>,
    },
    /// Generate a cargoless Rust API surface map.
    RustApiMap,
    /// Generate a CLI command contract snapshot.
    CliContract,
    /// Generate a public API surface snapshot.
    ApiSurfaceSnapshot,
    /// Generate source metrics.
    SourceMetrics,
    /// Validate the safety case.
    SafetyCaseCheck,
    /// Generate the next batched loop plan.
    NextLoopPlan,
    /// Validate Windows read-only adapter readiness scaffolding.
    WindowsReadinessCheck,
    /// Validate APFS repair governance and refusal scaffolding.
    RepairGovernanceAudit,
    /// Validate fuzz target scaffolding.
    FuzzScaffoldCheck,
    /// Validate a real or synthetic fixture manifest.
    FixtureManifestCheck {
        manifest: PathBuf,
    },
    /// Compare inspect JSON to a fixture manifest and write feedback artifacts.
    RealFixtureFeedback {
        inspect_json: PathBuf,
        manifest_json: PathBuf,
        out_dir: PathBuf,
    },
    /// Promote feedback JSON into generated Codev and Conductor task stubs.
    PromoteFeedback {
        feedback_json: PathBuf,
        out_dir: PathBuf,
    },
    /// Promote Cargo output into generated Codev/Conductor triage tasks.
    CargoTriage {
        cargo_log: PathBuf,
        out_dir: PathBuf,
    },
    /// Validate local handoff and platform setup scaffolding.
    HandoffReadinessCheck,
    /// Validate release/provenance scaffolding.
    ReleaseScaffoldCheck,
    /// Validate dynamic version metadata and version ledgers.
    VersionConsistencyCheck,
    /// Validate profiling budget configuration.
    ProfilingBudgetCheck,
    /// Validate profiling plan targets.
    ProfilingPlanAudit,
    /// Validate benchmark regression scaffolding.
    BenchmarkRegressionAudit,
    /// Validate long-running fuzz, property, mutation, and coverage hardening scaffolding.
    LongRunningHardeningAudit,
    /// Validate release automation configuration.
    ReleaseAutomationAudit,
    /// Run the aggregate bleeding-edge repo hardening audit.
    BleedingEdgeRepoAudit,
    /// Generate the mature release readiness dashboard and train summary.
    MatureReleaseReadinessDashboard,
    /// Validate branch protection and required-check governance readiness.
    BranchProtectionGovernanceAudit,
    /// Validate hosted Renovate lifecycle and dependency-update governance readiness.
    RenovateLifecycleAudit,
    /// Validate local handoff config files.
    ConfigSanityCheck,
    /// Run local environment doctor.
    LocalEnvDoctor {
        /// Optional JSON output path.
        #[arg(long)]
        json: Option<PathBuf>,
    },
    /// Generate handoff status.
    HandoffStatus {
        /// Write `HANDOFF_STATUS.md`/JSON.
        #[arg(long)]
        write: bool,
    },
    /// Generate repository manifest.
    RepoManifest {
        /// Write `REPO_MANIFEST.md`/JSON.
        #[arg(long)]
        write: bool,
    },
    /// Validate known uncompiled risk ledger.
    KnownRiskCheck,
    /// Plan or run the local compile loop.
    LocalCompileLoop {
        /// Execute cargo commands if cargo exists.
        #[arg(long)]
        execute: bool,
    },
    /// Run cargoless Cargo workspace audit.
    CargoWorkspaceAudit,
    /// Validate macOS fixture generator dry-run safety.
    MacosFixtureDryRun,
    /// Generate `WinFsp` callback matrix.
    WinfspCallbackMatrix,
    /// Generate production gap report.
    ProductionGapReport,
    /// Inventory tools available in the current environment.
    CurrentEnvironmentInventory,
    /// Classify remaining work by what can be completed in this environment.
    CurrentEnvRemaining,
    /// Generate Cargo path dependency graph.
    CargoDependencyGraph,
    /// Generate synthetic negative parser fixtures.
    SyntheticNegativeFixtures,
    /// Generate test/control matrix.
    TestMatrix,
    /// Audit handoff archive manifest.
    HandoffArchiveAudit,

    /// Run APFS offset and synthetic fixture byte-layout audit.
    ApfsOffsetAudit,
    /// Generate cargoless golden-output expectations for synthetic fixtures.
    GoldenOutputs,
    /// Run dependency license/policy audit.
    DependencyPolicyAudit,
    /// Export remaining backlog as issue stubs.
    BacklogIssueExport,
    /// Run all current-environment self-tests.
    CurrentEnvSelftest,
    /// Print task context for a capability ID.
    TaskContext {
        capability_id: String,
    },
    /// Write a release-publication evidence scaffold.
    ReleaseEvidence,
    /// Write an image-only write-lab crash-evidence scaffold.
    WriteLabEvidence,
    /// Write a Windows write-beta governance scaffold.
    WindowsWriteGovernance,
    /// Validate APFS format governance and refusal scaffolding.
    FormatGovernanceAudit,
    QualityGateCheck,
    DocsSiteCheck,
    TestScaffoldAudit,
}

#[allow(clippy::too_many_lines)]
fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::RegistryCheck => registry_check(),
        Command::ConductorCheck => conductor_check(),
        Command::ConductorTrackCloseout => {
            run_python_tool("tools/conductor_track_closeout.py", &[])
        }
        Command::SafetyCheck => safety_check(),
        Command::PrecompileCheck => precompile_check(),
        Command::CliContractCheck => run_python_tool("tools/cli_contract_check.py", &[]),
        Command::ContextIntegrityCheck => run_python_tool("tools/context_integrity_check.py", &[]),
        Command::SyntheticOracleCheck => run_python_tool("tools/synthetic_fixture_oracle.py", &[]),
        Command::TraceabilityMatrix => run_python_tool("tools/traceability_matrix.py", &[]),
        Command::LoopDashboard => run_python_tool("tools/loop_dashboard.py", &[]),
        Command::ReleasePreflight { write_manifest } => {
            if write_manifest {
                run_python_tool("tools/release_preflight.py", &["--write-manifest"])
            } else {
                run_python_tool("tools/release_preflight.py", &[])
            }
        }
        Command::DiagnosticsBundle { out, reports } => diagnostics_bundle(&out, &reports),
        Command::RustApiMap => run_python_tool("tools/rust_api_map.py", &[]),
        Command::CliContract => run_python_tool("tools/cli_contract_snapshot.py", &[]),
        Command::ApiSurfaceSnapshot => run_python_tool("tools/api_surface_snapshot.py", &[]),
        Command::SourceMetrics => run_python_tool("tools/source_metrics.py", &[]),
        Command::SafetyCaseCheck => run_python_tool("tools/safety_case_check.py", &[]),
        Command::NextLoopPlan => run_python_tool("tools/next_loop_plan.py", &[]),
        Command::WindowsReadinessCheck => run_python_tool("tools/windows_readiness_check.py", &[]),
        Command::RepairGovernanceAudit => repair_governance(),
        Command::FuzzScaffoldCheck => fuzz_scaffold_check(),
        Command::FixtureManifestCheck { manifest } => fixture_manifest_check(&manifest),
        Command::RealFixtureFeedback {
            inspect_json,
            manifest_json,
            out_dir,
        } => real_fixture_feedback(&inspect_json, &manifest_json, &out_dir),
        Command::PromoteFeedback {
            feedback_json,
            out_dir,
        } => promote_feedback(&feedback_json, &out_dir),
        Command::CargoTriage { cargo_log, out_dir } => cargo_triage(&cargo_log, &out_dir),
        Command::HandoffReadinessCheck => run_python_tool("tools/handoff_readiness_check.py", &[]),
        Command::ReleaseScaffoldCheck => run_python_tool("tools/release_scaffold_check.py", &[]),
        Command::VersionConsistencyCheck => {
            run_python_tool("tools/version_consistency_check.py", &[])
        }
        Command::ProfilingBudgetCheck => run_python_tool("tools/profiling_budget_check.py", &[]),
        Command::ProfilingPlanAudit => run_python_tool("tools/profiling_plan_audit.py", &[]),
        Command::BenchmarkRegressionAudit => {
            run_python_tool("tools/benchmark_regression_audit.py", &[])
        }
        Command::LongRunningHardeningAudit => long_running_hardening(),
        Command::ReleaseAutomationAudit => {
            run_python_tool("tools/release_automation_audit.py", &[])
        }
        Command::BleedingEdgeRepoAudit => run_python_tool("tools/bleeding_edge_repo_audit.py", &[]),
        Command::MatureReleaseReadinessDashboard => {
            run_python_tool("tools/mature_release_readiness_dashboard.py", &[])
        }
        Command::BranchProtectionGovernanceAudit => branch_protection_governance(),
        Command::RenovateLifecycleAudit => renovate_lifecycle_audit(),
        Command::ConfigSanityCheck => run_python_tool("tools/config_sanity_check.py", &[]),
        Command::LocalEnvDoctor { json } => {
            if let Some(path) = json {
                let path_string = path.display().to_string();
                run_python_tool(
                    "tools/local_env_doctor.py",
                    &["--json", path_string.as_str()],
                )
            } else {
                run_python_tool("tools/local_env_doctor.py", &[])
            }
        }
        Command::HandoffStatus { write } => {
            if write {
                run_python_tool("tools/handoff_status.py", &["--write"])
            } else {
                run_python_tool("tools/handoff_status.py", &[])
            }
        }
        Command::RepoManifest { write } => {
            if write {
                run_python_tool("tools/repo_manifest.py", &["--write"])
            } else {
                run_python_tool("tools/repo_manifest.py", &[])
            }
        }
        Command::KnownRiskCheck => run_python_tool("tools/known_risk_check.py", &[]),
        Command::LocalCompileLoop { execute } => {
            if execute {
                run_python_tool("tools/local_compile_loop.py", &["--execute"])
            } else {
                run_python_tool("tools/local_compile_loop.py", &[])
            }
        }
        Command::CargoWorkspaceAudit => run_python_tool("tools/cargo_workspace_audit.py", &[]),
        Command::MacosFixtureDryRun => run_python_tool("tools/macos_fixture_dry_run.py", &[]),
        Command::WinfspCallbackMatrix => run_python_tool("tools/winfsp_callback_matrix.py", &[]),
        Command::ProductionGapReport => run_python_tool("tools/production_gap_report.py", &[]),
        Command::CurrentEnvironmentInventory => {
            run_python_tool("tools/current_environment_inventory.py", &[])
        }
        Command::CurrentEnvRemaining => run_python_tool("tools/current_env_remaining.py", &[]),
        Command::CargoDependencyGraph => run_python_tool("tools/cargo_dependency_graph.py", &[]),
        Command::SyntheticNegativeFixtures => {
            run_python_tool("tools/synthetic_negative_fixture_generator.py", &[])
        }
        Command::TestMatrix => run_python_tool("tools/test_matrix_generator.py", &[]),
        Command::HandoffArchiveAudit => run_python_tool("tools/handoff_archive_audit.py", &[]),

        Command::ApfsOffsetAudit => run_python_tool("tools/apfs_offset_audit.py", &[]),
        Command::GoldenOutputs => run_python_tool("tools/golden_output_generator.py", &[]),
        Command::DependencyPolicyAudit => {
            run_python_tool("tools/dependency_license_policy_check.py", &[])
        }
        Command::BacklogIssueExport => run_python_tool("tools/backlog_issue_export.py", &[]),
        Command::CurrentEnvSelftest => run_python_tool("tools/current_env_selftest.py", &[]),
        Command::TaskContext { capability_id } => task_context(&capability_id),
        Command::ReleaseEvidence => release_evidence(),
        Command::WriteLabEvidence => write_lab_evidence(),
        Command::WindowsWriteGovernance => windows_write_governance(),
        Command::FormatGovernanceAudit => format_governance(),
        Command::QualityGateCheck => run_python_tool("tools/quality_gate_check.py", &[]),
        Command::DocsSiteCheck => run_python_tool("tools/docs_site_static_check.py", &[]),
        Command::TestScaffoldAudit => run_python_tool("tools/test_scaffold_audit.py", &[]),
    }
}

fn registry_check() -> Result<()> {
    let pairs = [
        (
            "codev/resources/capabilities.yaml",
            "codev/resources/schemas/capabilities.schema.json",
        ),
        (
            "codev/resources/safety-gates.yaml",
            "codev/resources/schemas/safety-gates.schema.json",
        ),
    ];

    for (yaml, schema) in pairs {
        validate_schema(yaml, schema)?;
    }
    validate_capability_safety_gate_links()?;
    conductor_check()?;
    println!("registry-check: schema, cross-registry, and Conductor validation passed");
    Ok(())
}

fn conductor_check() -> Result<()> {
    let required_roots = [
        "conductor/product.md",
        "conductor/product-guidelines.md",
        "conductor/tech-stack.md",
        "conductor/workflow.md",
        "conductor/tracks.md",
        "conductor/history.md",
        "REQUIREMENTS.md",
        "DESIGN.md",
        "REMAINING_ELEMENTS.md",
        "conductor/skills/conductor-context-management/SKILL.md",
        ".claude/skills/conductor-context-management/SKILL.md",
        ".gemini/skills/conductor-context-management/SKILL.md",
        ".agents/skills/conductor-context-management/SKILL.md",
    ];
    for path in required_roots {
        require_non_empty(path)?;
    }

    let mut track_count = 0usize;
    for entry in fs::read_dir("conductor/tracks").context("read conductor/tracks")? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let track = entry.file_name().to_string_lossy().to_string();
        let base = format!("conductor/tracks/{track}");
        require_non_empty(format!("{base}/metadata.json"))?;
        require_non_empty(format!("{base}/spec.md"))?;
        require_non_empty(format!("{base}/plan.md"))?;
        let metadata_text = fs::read_to_string(format!("{base}/metadata.json"))?;
        let metadata: JsonValue = serde_json::from_str(&metadata_text)
            .with_context(|| format!("parse {base}/metadata.json"))?;
        if metadata.get("track_id").and_then(JsonValue::as_str) != Some(track.as_str()) {
            anyhow::bail!("{base}/metadata.json track_id does not match directory name");
        }
        track_count += 1;
    }
    if track_count == 0 {
        anyhow::bail!("no Conductor tracks found");
    }
    run_python_tool("tools/conductor_track_closeout.py", &[])?;
    println!("conductor-check: passed with {track_count} historical tracks");
    Ok(())
}

fn require_non_empty(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    let text = fs::read_to_string(path)
        .with_context(|| format!("read required file {}", path.display()))?;
    if text.trim().is_empty() {
        anyhow::bail!("required file {} is empty", path.display());
    }
    Ok(())
}

fn validate_schema(yaml_path: &str, schema_path: &str) -> Result<()> {
    let yaml_text = fs::read_to_string(yaml_path).with_context(|| format!("read {yaml_path}"))?;
    let yaml_value: serde_yaml::Value =
        serde_yaml::from_str(&yaml_text).with_context(|| format!("parse {yaml_path}"))?;
    let instance: JsonValue =
        serde_json::to_value(yaml_value).with_context(|| format!("convert {yaml_path}"))?;

    let schema_text =
        fs::read_to_string(schema_path).with_context(|| format!("read {schema_path}"))?;
    let schema: JsonValue =
        serde_json::from_str(&schema_text).with_context(|| format!("parse {schema_path}"))?;
    let validator =
        jsonschema::validator_for(&schema).with_context(|| format!("compile {schema_path}"))?;

    if let Err(error) = validator.validate(&instance) {
        anyhow::bail!("{yaml_path} failed schema validation: {error}");
    }
    Ok(())
}

fn validate_capability_safety_gate_links() -> Result<()> {
    let capabilities_text = fs::read_to_string("codev/resources/capabilities.yaml")
        .context("read capabilities registry")?;
    let gates_text = fs::read_to_string("codev/resources/safety-gates.yaml")
        .context("read safety-gate registry")?;
    let capabilities: serde_yaml::Value =
        serde_yaml::from_str(&capabilities_text).context("parse capabilities registry")?;
    let gates: serde_yaml::Value =
        serde_yaml::from_str(&gates_text).context("parse safety-gate registry")?;

    let gate_map = yaml_get(&gates, "gates")
        .and_then(serde_yaml::Value::as_mapping)
        .context("safety-gates.yaml must contain a mapping at `gates`")?;
    let capability_map = yaml_get(&capabilities, "capabilities")
        .and_then(serde_yaml::Value::as_mapping)
        .context("capabilities.yaml must contain a mapping at `capabilities`")?;

    for (capability_id, capability) in capability_map {
        let capability_id = capability_id
            .as_str()
            .unwrap_or("<non-string capability id>");
        let Some(gates) =
            yaml_get(capability, "safety_gates").and_then(serde_yaml::Value::as_sequence)
        else {
            anyhow::bail!("capability {capability_id} has no safety_gates sequence");
        };
        for gate in gates {
            let gate_name = gate.as_str().unwrap_or("<non-string safety gate>");
            let gate_key = serde_yaml::Value::String(gate_name.to_owned());
            if !gate_map.contains_key(&gate_key) {
                anyhow::bail!(
                    "capability {capability_id} references undefined safety gate {gate_name}"
                );
            }
        }
    }
    Ok(())
}

fn yaml_get<'a>(value: &'a serde_yaml::Value, key: &str) -> Option<&'a serde_yaml::Value> {
    {
        let key_value = serde_yaml::Value::String(key.to_owned());
        value.as_mapping()?.get(&key_value)
    }
}

fn precompile_check() -> Result<()> {
    run_python_tool("tools/precompile_static_check.py", &[])?;
    run_python_tool("tools/config_sanity_check.py", &[])?;
    run_python_tool("tools/known_risk_check.py", &[])?;
    run_python_tool("tools/cli_contract_check.py", &[])?;
    run_python_tool("tools/context_integrity_check.py", &[])?;
    run_python_tool("tools/conductor_track_closeout.py", &[])?;
    run_python_tool("tools/version_consistency_check.py", &[])?;
    run_python_tool("tools/profiling_budget_check.py", &[])?;
    run_python_tool("tools/profiling_plan_audit.py", &[])?;
    run_python_tool("tools/benchmark_regression_audit.py", &[])?;
    run_python_tool("tools/release_automation_audit.py", &[])?;
    run_python_tool("tools/bleeding_edge_repo_audit.py", &[])
}

fn run_python_tool(script: &str, args: &[&str]) -> Result<()> {
    let status = ProcessCommand::new("python3")
        .arg(script)
        .args(args)
        .status()
        .with_context(|| format!("run {script}"))?;
    if !status.success() {
        anyhow::bail!("{script} failed");
    }
    Ok(())
}

fn safety_check() -> Result<()> {
    scan_for_unsafe_blocks()?;
    scan_for_suspicious_physical_write_terms()?;
    println!("safety-check: passed");
    Ok(())
}

fn scan_for_unsafe_blocks() -> Result<()> {
    for entry in WalkDir::new("crates")
        .into_iter()
        .chain(WalkDir::new("xtask"))
    {
        let entry = entry?;
        if !entry.file_type().is_file()
            || entry.path().extension().and_then(|ext| ext.to_str()) != Some("rs")
        {
            continue;
        }
        let path = entry.path();
        let text = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
        for (line_no, line) in text.lines().enumerate() {
            let trimmed = line.trim();
            let detector_implementation_line = path.ends_with("xtask/src/main.rs")
                && (trimmed.contains("trimmed.starts_with")
                    || trimmed.contains("trimmed.contains"));
            let suspicious = trimmed.starts_with("unsafe ")
                || trimmed.contains(" unsafe ")
                || trimmed.contains("unsafe{")
                || trimmed.contains("unsafe {");
            let allowed_lint = trimmed.contains("forbid(unsafe_code)");
            if suspicious && !allowed_lint && !detector_implementation_line {
                anyhow::bail!(
                    "unreviewed unsafe-looking code at {}:{}",
                    path.display(),
                    line_no + 1
                );
            }
        }
    }
    Ok(())
}

fn scan_for_suspicious_physical_write_terms() -> Result<()> {
    let suspicious_terms = [
        "GENERIC_WRITE",
        "FILE_WRITE_DATA",
        "raw_write",
        "write_at_device",
        "exclusive_write",
    ];
    for entry in WalkDir::new("crates") {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        let text = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
        for term in suspicious_terms {
            if text.contains(term) {
                anyhow::bail!(
                    "suspicious physical-write term `{term}` found in {}",
                    path.display()
                );
            }
        }
    }
    Ok(())
}

fn diagnostics_bundle(out: &Path, reports: &[PathBuf]) -> Result<()> {
    if reports.is_empty() {
        anyhow::bail!("diagnostics-bundle requires at least one JSON report");
    }
    let mut args: Vec<String> = vec!["--out".to_owned(), out.display().to_string()];
    args.extend(reports.iter().map(|path| path.display().to_string()));
    let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();
    run_python_tool("tools/diagnostics_bundle.py", &arg_refs)
}

fn fuzz_scaffold_check() -> Result<()> {
    require_non_empty("fuzz/Cargo.toml")?;
    require_non_empty("fuzz/fuzz_targets/object_header.rs")?;
    require_non_empty("fuzz/fuzz_targets/nx_superblock.rs")?;
    println!("fuzz-scaffold-check: passed");
    Ok(())
}

fn fixture_manifest_check(path: &Path) -> Result<()> {
    let text = fs::read_to_string(path)
        .with_context(|| format!("read fixture manifest {}", path.display()))?;
    let manifest: JsonValue = serde_json::from_str(&text)
        .with_context(|| format!("parse fixture manifest {}", path.display()))?;
    for field in [
        "schema_version",
        "fixture_id",
        "source_type",
        "created_with",
        "apfs_features",
        "expected_artifacts",
        "capability_ids",
        "redaction",
    ] {
        if manifest.get(field).is_none() {
            anyhow::bail!(
                "fixture manifest {} is missing required field `{field}`",
                path.display()
            );
        }
    }
    let redaction = manifest
        .get("redaction")
        .and_then(JsonValue::as_object)
        .context("manifest redaction must be an object")?;
    if redaction
        .get("contains_personal_data")
        .and_then(JsonValue::as_bool)
        .unwrap_or(true)
    {
        anyhow::bail!(
            "fixture manifest {} is marked as containing personal data",
            path.display()
        );
    }
    if redaction
        .get("contains_secret_material")
        .and_then(JsonValue::as_bool)
        .unwrap_or(true)
    {
        anyhow::bail!(
            "fixture manifest {} is marked as containing secret material",
            path.display()
        );
    }
    println!("fixture-manifest-check: {} passed", path.display());
    Ok(())
}

fn real_fixture_feedback(inspect_json: &Path, manifest_json: &Path, out_dir: &Path) -> Result<()> {
    let inspect_text = fs::read_to_string(inspect_json)
        .with_context(|| format!("read inspect JSON {}", inspect_json.display()))?;
    let manifest_text = fs::read_to_string(manifest_json)
        .with_context(|| format!("read manifest JSON {}", manifest_json.display()))?;
    let inspect: JsonValue = serde_json::from_str(&inspect_text)
        .with_context(|| format!("parse inspect JSON {}", inspect_json.display()))?;
    let manifest: JsonValue = serde_json::from_str(&manifest_text)
        .with_context(|| format!("parse manifest JSON {}", manifest_json.display()))?;

    let mut issues: Vec<String> = Vec::new();
    let mut matched: Vec<String> = Vec::new();

    for field in ["schema_version", "status", "safety"] {
        if inspect.get(field).is_some() {
            matched.push(field.to_owned());
        } else {
            issues.push(format!("inspect JSON missing required field `{field}`"));
        }
    }

    let redaction = manifest.get("redaction").and_then(JsonValue::as_object);
    if redaction
        .and_then(|r| r.get("contains_personal_data"))
        .and_then(JsonValue::as_bool)
        != Some(false)
    {
        issues
            .push("manifest must explicitly set redaction.contains_personal_data=false".to_owned());
    }
    if redaction
        .and_then(|r| r.get("contains_secret_material"))
        .and_then(JsonValue::as_bool)
        != Some(false)
    {
        issues.push(
            "manifest must explicitly set redaction.contains_secret_material=false".to_owned(),
        );
    }

    let expected_fields = manifest
        .get("expected_inspect_fields")
        .and_then(JsonValue::as_object);
    if let Some(expected_fields) = expected_fields {
        for (path, expected) in expected_fields {
            let observed = json_path_get(&inspect, path);
            if observed == Some(expected) {
                matched.push(path.clone());
            } else {
                issues.push(format!(
                    "expected `{path}` to be {expected}, observed {observed:?}"
                ));
            }
        }
    } else {
        issues.push(
            "manifest has no expected_inspect_fields section; feedback is generic only".to_owned(),
        );
    }

    fs::create_dir_all(out_dir)?;
    let status = if issues.is_empty() {
        "matched"
    } else {
        "needs_tasks"
    };
    let report = serde_json::json!({
        "schema_version": "0.15.0",
        "status": status,
        "inspect_json": inspect_json.display().to_string(),
        "manifest_json": manifest_json.display().to_string(),
        "fixture_id": manifest.get("fixture_id"),
        "matched_fields": matched,
        "issues": issues,
        "safety_note": "This feedback check reads JSON artifacts only and does not open, mount, decrypt, repair, format, or write APFS media."
    });
    fs::write(
        out_dir.join("real-fixture-feedback.json"),
        serde_json::to_string_pretty(&report)? + "\n",
    )?;
    fs::write(
        out_dir.join("real-fixture-feedback.md"),
        format_feedback_markdown(&report),
    )?;
    println!("real-fixture-feedback: wrote {}", out_dir.display());
    Ok(())
}

fn json_path_get<'a>(value: &'a JsonValue, path: &str) -> Option<&'a JsonValue> {
    let mut current = value;
    for part in path.split('.') {
        current = current.get(part)?;
    }
    Some(current)
}

fn format_feedback_markdown(report: &JsonValue) -> String {
    let mut out = String::from("# APFS-RS Real Fixture Feedback\n\n");
    let _ = write!(
        out,
        "Status: `{}`\n\n",
        report
            .get("status")
            .and_then(JsonValue::as_str)
            .unwrap_or("unknown")
    );
    out.push_str("## Safety note\n\n");
    out.push_str(
        report
            .get("safety_note")
            .and_then(JsonValue::as_str)
            .unwrap_or("No safety note."),
    );
    out.push_str("\n\n## Issues\n\n");
    if let Some(issues) = report.get("issues").and_then(JsonValue::as_array) {
        if issues.is_empty() {
            out.push_str("- None\n");
        } else {
            for issue in issues {
                let _ = writeln!(out, "- {}", issue.as_str().unwrap_or("<non-string issue>"));
            }
        }
    }
    out
}

fn normalize_promoted_issue(issue: &JsonValue, index: usize) -> JsonValue {
    let fallback_field = format!("issue-{:04}", index + 1);
    match issue {
        JsonValue::String(text) => serde_json::json!({
            "field": fallback_field,
            "message": text,
            "severity": "error",
            "suggested_track": "0012-real-fixture-feedback-loop",
            "title": text,
            "raw": text,
        }),
        JsonValue::Object(map) => {
            let title = map
                .get("title")
                .and_then(JsonValue::as_str)
                .or_else(|| map.get("field").and_then(JsonValue::as_str))
                .or_else(|| map.get("message").and_then(JsonValue::as_str))
                .unwrap_or("real fixture mismatch");
            let field = map
                .get("field")
                .and_then(JsonValue::as_str)
                .or_else(|| map.get("path").and_then(JsonValue::as_str))
                .unwrap_or(&fallback_field);
            let severity = map
                .get("severity")
                .and_then(JsonValue::as_str)
                .unwrap_or("error");
            let message = map
                .get("message")
                .and_then(JsonValue::as_str)
                .or_else(|| map.get("title").and_then(JsonValue::as_str))
                .unwrap_or("No message supplied");
            let suggested_track = map
                .get("suggested_track")
                .and_then(JsonValue::as_str)
                .unwrap_or("0012-real-fixture-feedback-loop");
            serde_json::json!({
                "field": field,
                "message": message,
                "raw": issue,
                "severity": severity,
                "suggested_track": suggested_track,
                "title": title,
            })
        }
        _ => serde_json::json!({
            "field": fallback_field,
            "message": "Unsupported feedback issue shape",
            "raw": issue,
            "severity": "error",
            "suggested_track": "0012-real-fixture-feedback-loop",
            "title": "real fixture mismatch",
        }),
    }
}

fn promoted_issue_slug(title: &str) -> String {
    let slug = title
        .to_ascii_lowercase()
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect::<String>();
    let slug = slug
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    if slug.is_empty() {
        "real-fixture-mismatch".to_owned()
    } else {
        slug
    }
}

fn promote_feedback(feedback_json: &Path, out_dir: &Path) -> Result<()> {
    fs::create_dir_all(out_dir).with_context(|| format!("create {}", out_dir.display()))?;
    let text = fs::read_to_string(feedback_json)
        .with_context(|| format!("read feedback JSON {}", feedback_json.display()))?;
    let feedback: JsonValue = serde_json::from_str(&text)
        .with_context(|| format!("parse feedback JSON {}", feedback_json.display()))?;
    let issues = feedback
        .get("issues")
        .and_then(JsonValue::as_array)
        .cloned()
        .unwrap_or_default();
    let normalized_issues: Vec<JsonValue> = issues
        .iter()
        .enumerate()
        .map(|(idx, issue)| normalize_promoted_issue(issue, idx))
        .collect();
    let mut index = Vec::new();
    for (idx, issue) in normalized_issues.iter().enumerate() {
        let title = issue
            .get("title")
            .and_then(JsonValue::as_str)
            .unwrap_or("real fixture mismatch");
        let slug = promoted_issue_slug(title);
        let track_id = format!("generated-{:04}-{}", idx + 1, slug);
        let track_dir = out_dir.join("conductor/tracks").join(&track_id);
        fs::create_dir_all(&track_dir)?;
        let field = issue
            .get("field")
            .and_then(JsonValue::as_str)
            .unwrap_or(&format!("issue-{:04}", idx + 1))
            .to_owned();
        let severity = issue
            .get("severity")
            .and_then(JsonValue::as_str)
            .unwrap_or("error");
        let message = issue
            .get("message")
            .and_then(JsonValue::as_str)
            .unwrap_or("No message supplied");
        let suggested_track = issue
            .get("suggested_track")
            .and_then(JsonValue::as_str)
            .unwrap_or("0012-real-fixture-feedback-loop");
        let issue_json = serde_json::to_string_pretty(issue)?;
        fs::write(
            track_dir.join("metadata.json"),
            format!(
                "{{\n  \"track_id\": \"{track_id}\",\n  \"source\": \"real-fixture-feedback\",\n  \"status\": \"generated\",\n  \"field\": \"{field}\",\n  \"severity\": \"{severity}\",\n  \"suggested_track\": \"{suggested_track}\"\n}}\n"
            ),
        )?;
        fs::write(
            track_dir.join("spec.md"),
            format!(
                "# Generated Spec: {}\n\nSource feedback issue generated from `{}`.\n\n```json\n{}\n```\n",
                title,
                feedback_json.display(),
                issue_json
            ),
        )?;
        fs::write(
            track_dir.join("plan.md"),
            format!(
                "# Generated Plan\n\n1. Reproduce the mismatch with the referenced real fixture.\n2. Add or adjust a synthetic fixture only if it preserves clean-room safety.\n3. Implement the smallest parser correction.\n4. Update Codev and Conductor reviews.\n\n## Issue details\n\n- Field: `{field}`\n- Severity: `{severity}`\n- Suggested track: `{suggested_track}`\n- Message: `{message}`\n"
            ),
        )?;
        index.push(JsonValue::String(track_id));
    }
    fs::write(
        out_dir.join("task-index.json"),
        serde_json::to_string_pretty(&index)? + "\n",
    )?;
    println!(
        "promote-feedback: generated {} track stubs in {}",
        normalized_issues.len(),
        out_dir.display()
    );
    Ok(())
}

fn cargo_triage(cargo_log: &Path, out_dir: &Path) -> Result<()> {
    let args = [
        cargo_log.display().to_string(),
        out_dir.display().to_string(),
    ];
    let refs: Vec<&str> = args.iter().map(String::as_str).collect();
    run_python_tool("tools/cargo_error_to_tracks.py", &refs)
}

#[allow(clippy::unnecessary_wraps)]
fn task_context(capability_id: &str) -> Result<()> {
    println!("task-context for {capability_id}");
    println!("read codev/resources/capabilities.yaml and codev/resources/safety-gates.yaml");
    println!(
        "read conductor/tracks.md and the matching conductor/tracks/<track>/spec.md + plan.md"
    );
    println!("current implementation focus: M-001 through M-058, ending with local-handoff tooling, config sanity, environment doctor, repository manifest, and uncompiled-risk ledger");
    Ok(())
}

fn release_publication_readiness() -> JsonValue {
    let root = workspace_root();
    let schema_version =
        std::env::var("APFS_RS_VERSION").unwrap_or_else(|_| env!("CARGO_PKG_VERSION").to_owned());
    let release_scaffold = fs::read_to_string(root.join("RELEASE_SCAFFOLD.md")).unwrap_or_default();
    let release_automation =
        fs::read_to_string(root.join("RELEASE_AUTOMATION.md")).unwrap_or_default();
    let provenance_verification =
        fs::read_to_string(root.join("PROVENANCE_VERIFICATION.md")).unwrap_or_default();
    let winget_manifest =
        fs::read_to_string(root.join("packaging/windows/winget/apfs-rs.yaml")).unwrap_or_default();
    let release_workflow =
        fs::read_to_string(root.join(".github/workflows/release.yml")).unwrap_or_default();
    let release_automation_workflow =
        fs::read_to_string(root.join(".github/workflows/release-automation.yml"))
            .unwrap_or_default();
    let provenance_workflow =
        fs::read_to_string(root.join(".github/workflows/provenance-verify.yml"))
            .unwrap_or_default();

    let artifacts = [
        (
            "release scaffold",
            !release_scaffold.trim().is_empty(),
            "RELEASE_SCAFFOLD.md describes the planned SBOM, attestations, checksums, and winget gates.",
        ),
        (
            "release automation",
            !release_automation.trim().is_empty(),
            "RELEASE_AUTOMATION.md and the release-automation workflow keep cargo-dist and release-plz configured.",
        ),
        (
            "provenance verification",
            !provenance_verification.trim().is_empty(),
            "PROVENANCE_VERIFICATION.md and provenance-verify workflow document attestation checks.",
        ),
        (
            "winget manifest placeholder",
            !winget_manifest.contains("0.0.0-placeholder"),
            "Winget manifest still carries a placeholder version until a signed release exists.",
        ),
        (
            "release workflow",
            release_workflow.contains("attest-build-provenance")
                && release_workflow.contains("cargo test --workspace"),
            "Release workflow is configured for checksums, attestations, and workspace tests.",
        ),
        (
            "release automation workflow",
            release_automation_workflow.contains("release-plz release --dry-run --allow-dirty --config release-plz.toml")
                && release_automation_workflow.contains("dist plan --allow-dirty"),
            "Release automation workflow runs cargo-dist and release-plz in dry-run mode.",
        ),
        (
            "provenance workflow",
            provenance_workflow.contains("attest-build-provenance"),
            "Provenance workflow performs an attest-build-provenance dry run.",
        ),
    ];
    let configured_artifacts: Vec<JsonValue> = artifacts
        .iter()
        .map(|(name, configured, note)| {
            serde_json::json!({
                "name": name,
                "configured": configured,
                "note": note,
            })
        })
        .collect();
    let missing_steps: Vec<String> = artifacts
        .iter()
        .filter_map(|(name, configured, _)| {
            if *configured {
                None
            } else {
                Some((*name).to_owned())
            }
        })
        .collect();

    serde_json::json!({
        "schema_version": schema_version,
        "track": "M-130",
        "status": "scaffolded_read_only",
        "public_release_ready": false,
        "configured_artifacts": configured_artifacts,
        "missing_steps": missing_steps,
        "safety_constraints": [
            "no public release is claimed from scaffold alone",
            "no physical-device writes",
            "no encryption bypass",
            "no unreviewed release publication",
        ],
        "release_notes": [
            "SBOM, provenance, and winget paths are configured but not published.",
            "Signed release publication remains gated on passing CI and maintainer approval."
        ],
    })
}

fn release_evidence() -> Result<()> {
    let dir = workspace_root().join("target/release-evidence");
    fs::create_dir_all(&dir)?;
    let report = release_publication_readiness();
    fs::write(
        dir.join("README.md"),
        "# APFS-RS Release Evidence\n\nGenerated scaffold. This is not production release evidence yet.\n",
    )?;
    fs::write(
        dir.join("release-publication-report.json"),
        serde_json::to_string_pretty(&report)? + "\n",
    )?;
    let mut markdown = String::from("# APFS-RS Release Publication Readiness\n\n");
    markdown.push_str("Status: `scaffolded_read_only`.\n\n");
    markdown.push_str("## Configured artifacts\n\n");
    if let Some(artifacts) = report
        .get("configured_artifacts")
        .and_then(JsonValue::as_array)
    {
        for artifact in artifacts {
            let name = artifact
                .get("name")
                .and_then(JsonValue::as_str)
                .unwrap_or("<artifact>");
            let configured = artifact
                .get("configured")
                .and_then(JsonValue::as_bool)
                .unwrap_or(false);
            let note = artifact
                .get("note")
                .and_then(JsonValue::as_str)
                .unwrap_or("");
            let _ = writeln!(markdown, "- `{name}`: {configured} - {note}");
        }
    }
    markdown.push_str("\n## Missing steps\n\n");
    if let Some(missing) = report.get("missing_steps").and_then(JsonValue::as_array) {
        if missing.is_empty() {
            markdown.push_str("- None\n");
        } else {
            for item in missing {
                let _ = writeln!(markdown, "- {}", item.as_str().unwrap_or("<missing>"));
            }
        }
    }
    fs::write(dir.join("release-publication-report.md"), markdown)?;
    println!("release-evidence: wrote {}", dir.display());
    Ok(())
}

fn write_lab_evidence() -> Result<()> {
    let dir = workspace_root().join("target/write-lab-evidence");
    fs::create_dir_all(&dir)?;
    let report = write_lab_evidence_report();
    fs::write(
        dir.join("README.md"),
        "# APFS-RS Image-Only Write-Lab Evidence\n\nGenerated scaffold. This is not production write evidence yet.\n",
    )?;
    fs::write(
        dir.join("write-lab-evidence-report.json"),
        serde_json::to_string_pretty(&report)? + "\n",
    )?;
    let mut markdown = String::from("# APFS-RS Image-Only Write-Lab Evidence\n\n");
    markdown.push_str("Status: `disposable_image_only`.\n\n");
    markdown.push_str("## Planned operations\n\n");
    for operation in &report.planned_operations {
        let _ = writeln!(markdown, "- `{operation}`");
    }
    markdown.push_str("\n## Safety constraints\n\n");
    for constraint in &report.safety_constraints {
        let _ = writeln!(markdown, "- {constraint}");
    }
    markdown.push_str("\n## Evidence notes\n\n");
    for note in &report.evidence_notes {
        let _ = writeln!(markdown, "- {note}");
    }
    fs::write(dir.join("write-lab-evidence-report.md"), markdown)?;
    println!("write-lab-evidence: wrote {}", dir.display());
    Ok(())
}

fn windows_write_governance() -> Result<()> {
    let dir = workspace_root().join("target/windows-write-governance");
    fs::create_dir_all(&dir)?;
    let report = windows_write_beta_governance_report();
    fs::write(
        dir.join("README.md"),
        "# APFS-RS Windows Write-Beta Governance\n\nGenerated scaffold. This is not a write beta.\n",
    )?;
    fs::write(
        dir.join("windows-write-governance-report.json"),
        serde_json::to_string_pretty(&report)? + "\n",
    )?;
    let mut markdown = String::from("# APFS-RS Windows Write-Beta Governance\n\n");
    markdown.push_str("Status: `blocked_until_accepted_write_lab_evidence`.\n\n");
    markdown.push_str("## Prerequisites\n\n");
    for item in &report.prerequisites {
        let _ = writeln!(markdown, "- {item}");
    }
    markdown.push_str("\n## Required gates\n\n");
    for gate in &report.required_gates {
        let _ = writeln!(markdown, "- {gate}");
    }
    markdown.push_str("\n## Rollback plan\n\n");
    for step in &report.rollback_plan {
        let _ = writeln!(markdown, "- {step}");
    }
    markdown.push_str("\n## Refused operations\n\n");
    for operation in &report.refused_operations {
        let _ = writeln!(markdown, "- `{operation}`");
    }
    fs::write(dir.join("windows-write-governance-report.md"), markdown)?;
    println!("windows-write-governance: wrote {}", dir.display());
    Ok(())
}

fn repair_governance_report() -> JsonValue {
    serde_json::json!({
        "schema_version": "0.18.0",
        "track": "M-134",
        "status": "blocked_until_accepted_destructive_test_evidence",
        "prerequisites": [
            "accepted destructive-test evidence on disposable images",
            "maintainer approval for any repair beta",
            "production claim guard passes before release claims",
            "no physical-device repair path exists in the implementation"
        ],
        "required_gates": [
            "repair_governance_audit",
            "production_claim_guard",
            "safety_case_check",
        ],
        "rollback_plan": [
            "disable repair-governance feature flags",
            "revert to read-only inspection and extraction paths",
            "publish refusal notes before any public repair claim",
        ],
        "refused_operations": [
            "repair",
            "fsck",
            "recover",
            "rebuild-catalog",
            "metadata-rewrite",
            "physical-device-write",
        ],
        "safety_constraints": [
            "read-only default until accepted destructive-test evidence exists",
            "no physical-device writes",
            "no encryption bypass",
            "no password recovery or repair bypass",
            "no live repair beta without maintainer approval",
        ],
        "evidence_notes": [
            "governance-only scaffold; it does not enable APFS repair",
            "the track remains blocked until destructive-test evidence is accepted",
        ],
    })
}

fn repair_governance() -> Result<()> {
    let dir = workspace_root().join("target/repair-governance");
    fs::create_dir_all(&dir)?;
    let report = repair_governance_report();
    fs::write(
        dir.join("README.md"),
        "# APFS-RS APFS Repair Governance\n\nGenerated scaffold. This is not APFS repair.\n",
    )?;
    fs::write(
        dir.join("repair-governance-report.json"),
        serde_json::to_string_pretty(&report)? + "\n",
    )?;
    let mut markdown = String::from("# APFS-RS APFS Repair Governance\n\n");
    markdown.push_str("Status: `blocked_until_accepted_destructive_test_evidence`.\n\n");
    markdown.push_str("## Prerequisites\n\n");
    for item in report
        .get("prerequisites")
        .and_then(JsonValue::as_array)
        .into_iter()
        .flatten()
    {
        let _ = writeln!(markdown, "- {}", item.as_str().unwrap_or("<prerequisite>"));
    }
    markdown.push_str("\n## Required gates\n\n");
    for gate in report
        .get("required_gates")
        .and_then(JsonValue::as_array)
        .into_iter()
        .flatten()
    {
        let _ = writeln!(markdown, "- {}", gate.as_str().unwrap_or("<gate>"));
    }
    markdown.push_str("\n## Rollback plan\n\n");
    for step in report
        .get("rollback_plan")
        .and_then(JsonValue::as_array)
        .into_iter()
        .flatten()
    {
        let _ = writeln!(markdown, "- {}", step.as_str().unwrap_or("<rollback step>"));
    }
    markdown.push_str("\n## Refused operations\n\n");
    for operation in report
        .get("refused_operations")
        .and_then(JsonValue::as_array)
        .into_iter()
        .flatten()
    {
        let _ = writeln!(
            markdown,
            "- `{}`",
            operation.as_str().unwrap_or("<operation>")
        );
    }
    fs::write(dir.join("repair-governance-report.md"), markdown)?;
    println!("repair-governance: wrote {}", dir.display());
    Ok(())
}

fn format_governance_report() -> JsonValue {
    serde_json::json!({
        "schema_version": "0.18.0",
        "track": "M-135",
        "status": "blocked_until_accepted_destructive_test_evidence",
        "prerequisites": [
            "accepted destructive-test evidence on disposable images",
            "maintainer approval for any format beta",
            "production claim guard passes before release claims",
            "no physical-device format path exists in the implementation"
        ],
        "required_gates": [
            "format_governance_audit",
            "production_claim_guard",
            "safety_case_check",
        ],
        "rollback_plan": [
            "disable format-governance feature flags",
            "revert to read-only inspection and extraction paths",
            "publish refusal notes before any public format claim",
        ],
        "refused_operations": [
            "format",
            "mkfs",
            "erase",
            "initialize",
            "metadata-rewrite",
            "physical-device-write",
        ],
        "safety_constraints": [
            "read-only default until accepted destructive-test evidence exists",
            "no physical-device writes",
            "no encryption bypass",
            "no password recovery or format bypass",
            "no live format beta without maintainer approval",
        ],
        "evidence_notes": [
            "governance-only scaffold; it does not enable APFS format",
            "the track remains blocked until destructive-test evidence is accepted",
        ],
    })
}

fn format_governance() -> Result<()> {
    let dir = workspace_root().join("target/format-governance");
    fs::create_dir_all(&dir)?;
    let report = format_governance_report();
    fs::write(
        dir.join("README.md"),
        "# APFS-RS APFS Format Governance\n\nGenerated scaffold. This is not APFS format.\n",
    )?;
    fs::write(
        dir.join("format-governance-report.json"),
        serde_json::to_string_pretty(&report)? + "\n",
    )?;
    let mut markdown = String::from("# APFS-RS APFS Format Governance\n\n");
    markdown.push_str("Status: `blocked_until_accepted_destructive_test_evidence`.\n\n");
    markdown.push_str("## Prerequisites\n\n");
    for item in report
        .get("prerequisites")
        .and_then(JsonValue::as_array)
        .into_iter()
        .flatten()
    {
        let _ = writeln!(markdown, "- {}", item.as_str().unwrap_or("<prerequisite>"));
    }
    markdown.push_str("\n## Required gates\n\n");
    for gate in report
        .get("required_gates")
        .and_then(JsonValue::as_array)
        .into_iter()
        .flatten()
    {
        let _ = writeln!(markdown, "- {}", gate.as_str().unwrap_or("<gate>"));
    }
    markdown.push_str("\n## Rollback plan\n\n");
    for step in report
        .get("rollback_plan")
        .and_then(JsonValue::as_array)
        .into_iter()
        .flatten()
    {
        let _ = writeln!(markdown, "- {}", step.as_str().unwrap_or("<rollback step>"));
    }
    markdown.push_str("\n## Refused operations\n\n");
    for operation in report
        .get("refused_operations")
        .and_then(JsonValue::as_array)
        .into_iter()
        .flatten()
    {
        let _ = writeln!(
            markdown,
            "- `{}`",
            operation.as_str().unwrap_or("<operation>")
        );
    }
    fs::write(dir.join("format-governance-report.md"), markdown)?;
    println!("format-governance: wrote {}", dir.display());
    Ok(())
}

fn long_running_hardening_report() -> JsonValue {
    serde_json::json!({
        "schema_version": "0.18.0",
        "track": "M-136",
        "status": "scaffolded_read_only",
        "release_gate_ready": false,
        "configured_hardening": [
            {
                "name": "profiling_budget_check",
                "configured": true,
                "evidence": "profiling/profile_plan.json, profiling_budget_check.py, and profiling workflow"
            },
            {
                "name": "benchmark_regression_audit",
                "configured": true,
                "evidence": "BENCHMARK_REGRESSION_PLAN.md, benches/, and profiling/"
            },
            {
                "name": "bleeding_edge_repo_audit",
                "configured": true,
                "evidence": "aggregates hardening audits without running destructive operations"
            },
            {
                "name": "quality_gate_check",
                "configured": true,
                "evidence": "quality gates, fuzz, mutation, coverage, and profiling scaffolding remain documented"
            }
        ],
        "required_gates": [
            "profiling_budget_check",
            "benchmark_regression_audit",
            "bleeding_edge_repo_audit",
            "quality_gate_check",
        ],
        "safety_constraints": [
            "read-only default until long-running gates are actually enforced",
            "no physical-device writes",
            "no encryption bypass",
            "no production claim without sustained gate evidence",
            "no media mutation from the hardening audit",
        ],
        "evidence_notes": [
            "governance scaffolding for long-running hardening",
            "the release gate remains configured rather than executed by this track",
        ],
    })
}

fn long_running_hardening() -> Result<()> {
    let dir = workspace_root().join("target/long-running-hardening");
    fs::create_dir_all(&dir)?;
    let report = long_running_hardening_report();
    fs::write(
        dir.join("README.md"),
        "# APFS-RS Long-Running Hardening\n\nGenerated scaffold. This is not a sustained release gate yet.\n",
    )?;
    fs::write(
        dir.join("long-running-hardening-report.json"),
        serde_json::to_string_pretty(&report)? + "\n",
    )?;
    let mut markdown = String::from("# APFS-RS Long-Running Hardening\n\n");
    markdown.push_str("Status: `scaffolded_read_only`.\n\n");
    markdown.push_str("## Configured hardening\n\n");
    for item in report
        .get("configured_hardening")
        .and_then(JsonValue::as_array)
        .into_iter()
        .flatten()
    {
        let name = item
            .get("name")
            .and_then(JsonValue::as_str)
            .unwrap_or("<gate>");
        let configured = item
            .get("configured")
            .and_then(JsonValue::as_bool)
            .unwrap_or(false);
        let evidence = item
            .get("evidence")
            .and_then(JsonValue::as_str)
            .unwrap_or("");
        let _ = writeln!(markdown, "- `{name}`: {configured} - {evidence}");
    }
    markdown.push_str("\n## Required gates\n\n");
    for gate in report
        .get("required_gates")
        .and_then(JsonValue::as_array)
        .into_iter()
        .flatten()
    {
        let _ = writeln!(markdown, "- {}", gate.as_str().unwrap_or("<gate>"));
    }
    markdown.push_str("\n## Safety constraints\n\n");
    for constraint in report
        .get("safety_constraints")
        .and_then(JsonValue::as_array)
        .into_iter()
        .flatten()
    {
        let _ = writeln!(
            markdown,
            "- {}",
            constraint.as_str().unwrap_or("<constraint>")
        );
    }
    fs::write(dir.join("long-running-hardening-report.md"), markdown)?;
    println!("long-running-hardening: wrote {}", dir.display());
    Ok(())
}

fn branch_protection_governance_report() -> JsonValue {
    serde_json::json!({
        "schema_version": "0.18.0",
        "track": "M-137",
        "status": "admin_readiness_only",
        "required_workflows": [
            "ci.yml",
            "quality-gates.yml",
            "strict-quality.yml",
            "workflow-security.yml",
        ],
        "required_checks": [
            "cargo fmt --all -- --check",
            "cargo test --workspace",
            "cargo xtask registry-check",
            "cargo xtask conductor-check",
            "cargo xtask safety-check",
            "cargo xtask precompile-check",
        ],
        "external_permissions": [
            "repository administration permission",
            "branch protection configuration permission",
            "required-check management permission",
        ],
        "safety_constraints": [
            "governance-only readiness; no repository admin mutation is performed",
            "no physical-device writes",
            "no encryption bypass",
            "no production claim from readiness alone",
        ],
        "evidence_notes": [
            "the workflows already exist and are gated in CI",
            "this track documents the admin boundary and required-check set",
        ],
    })
}

fn branch_protection_governance() -> Result<()> {
    let dir = workspace_root().join("target/branch-protection-governance");
    fs::create_dir_all(&dir)?;
    let report = branch_protection_governance_report();
    fs::write(
        dir.join("README.md"),
        "# APFS-RS Branch Protection Governance\n\nGenerated scaffold. This is not a repository admin action.\n",
    )?;
    fs::write(
        dir.join("branch-protection-governance-report.json"),
        serde_json::to_string_pretty(&report)? + "\n",
    )?;
    let mut markdown = String::from("# APFS-RS Branch Protection Governance\n\n");
    markdown.push_str("Status: `admin_readiness_only`.\n\n");
    markdown.push_str("## Required workflows\n\n");
    for workflow in report
        .get("required_workflows")
        .and_then(JsonValue::as_array)
        .into_iter()
        .flatten()
    {
        let _ = writeln!(markdown, "- {}", workflow.as_str().unwrap_or("<workflow>"));
    }
    markdown.push_str("\n## Required checks\n\n");
    for check in report
        .get("required_checks")
        .and_then(JsonValue::as_array)
        .into_iter()
        .flatten()
    {
        let _ = writeln!(markdown, "- `{}`", check.as_str().unwrap_or("<check>"));
    }
    markdown.push_str("\n## External permissions\n\n");
    for permission in report
        .get("external_permissions")
        .and_then(JsonValue::as_array)
        .into_iter()
        .flatten()
    {
        let _ = writeln!(
            markdown,
            "- {}",
            permission.as_str().unwrap_or("<permission>")
        );
    }
    fs::write(dir.join("branch-protection-governance-report.md"), markdown)?;
    println!("branch-protection-governance: wrote {}", dir.display());
    Ok(())
}

fn renovate_lifecycle_report() -> JsonValue {
    let root = workspace_root();
    let renovate_path = root.join("renovate.json");
    let renovate_text = fs::read_to_string(&renovate_path).unwrap_or_default();
    let renovate: JsonValue = serde_json::from_str(&renovate_text).unwrap_or_else(|_| {
        serde_json::json!({
            "parse_error": true,
        })
    });

    let managed_files = [
        ".github/workflows/ci.yml",
        ".github/workflows/quality-gates.yml",
        ".github/workflows/strict-quality.yml",
        ".github/workflows/workflow-security.yml",
    ];
    let required_managers = [
        "cargo",
        "github-actions",
        "npm",
        "dockerfile",
        "pip_requirements",
    ];

    let package_rules = renovate
        .get("packageRules")
        .and_then(JsonValue::as_array)
        .cloned()
        .unwrap_or_default();

    serde_json::json!({
        "schema_version": "0.18.0",
        "track": "M-138",
        "status": "hosted_renovate_ready",
        "renovate_configured": renovate.get("parse_error").is_none(),
        "renovate_mode": renovate.get("mode").cloned().unwrap_or(JsonValue::Null),
        "silent_mode_enabled": renovate
            .get("mode")
            .and_then(JsonValue::as_str)
            .map(|mode| mode == "silent")
            .unwrap_or(false),
        "dependabot_config_present": root.join(".github/dependabot.yml").exists()
            || root.join(".github/dependabot.yaml").exists(),
        "required_managers": required_managers,
        "enabled_managers": renovate.get("enabledManagers").cloned().unwrap_or(JsonValue::Null),
        "managed_workflows": managed_files,
        "package_rules_count": package_rules.len(),
        "safety_constraints": [
            "read-only lifecycle evidence only; no repository-admin mutation is performed",
            "no physical-device writes",
            "no encryption bypass",
            "dependency automation remains policy-gated by renovate.json and local checks",
        ],
        "evidence_notes": [
            "renovate.json exists and is checked by local sanity validation",
            "renovate.json runs in silent mode so hosted automation does not create or update dependency issues",
            "Dependabot config files are forbidden so Renovate remains the active update path",
            "workflow-security and release gates remain separate from dependency automation",
        ],
    })
}

fn renovate_lifecycle_audit() -> Result<()> {
    let dir = workspace_root().join("target/renovate-lifecycle");
    fs::create_dir_all(&dir)?;
    let report = renovate_lifecycle_report();
    fs::write(
        dir.join("README.md"),
        "# APFS-RS Hosted Renovate Lifecycle\n\nGenerated scaffold. This is not a repository-admin action.\n",
    )?;
    fs::write(
        dir.join("renovate-lifecycle-report.json"),
        serde_json::to_string_pretty(&report)? + "\n",
    )?;
    let mut markdown = String::from("# APFS-RS Hosted Renovate Lifecycle\n\n");
    markdown.push_str("Status: `hosted_renovate_ready`.\n\n");
    markdown.push_str("## Required managers\n\n");
    for manager in report
        .get("required_managers")
        .and_then(JsonValue::as_array)
        .into_iter()
        .flatten()
    {
        let _ = writeln!(markdown, "- {}", manager.as_str().unwrap_or("<manager>"));
    }
    markdown.push_str("\n## Managed workflows\n\n");
    for workflow in report
        .get("managed_workflows")
        .and_then(JsonValue::as_array)
        .into_iter()
        .flatten()
    {
        let _ = writeln!(markdown, "- {}", workflow.as_str().unwrap_or("<workflow>"));
    }
    markdown.push_str("\n## Safety constraints\n\n");
    for constraint in report
        .get("safety_constraints")
        .and_then(JsonValue::as_array)
        .into_iter()
        .flatten()
    {
        let _ = writeln!(
            markdown,
            "- {}",
            constraint.as_str().unwrap_or("<constraint>")
        );
    }
    fs::write(dir.join("renovate-lifecycle-report.md"), markdown)?;
    println!("renovate-lifecycle: wrote {}", dir.display());
    Ok(())
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask manifest has workspace parent")
        .to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::{
        branch_protection_governance, branch_protection_governance_report, format_governance,
        format_governance_report, long_running_hardening, long_running_hardening_report,
        release_publication_readiness, renovate_lifecycle_audit, renovate_lifecycle_report,
        repair_governance, repair_governance_report, windows_write_beta_governance_report,
        windows_write_governance, write_lab_evidence_report,
    };
    use apfs_win::WindowsWriteBetaGovernanceStatus;
    use apfs_write_lab::WriteLabEvidenceStatus;

    #[test]
    fn release_publication_readiness_remains_scaffolded_and_not_published() {
        let report = release_publication_readiness();

        assert_eq!(report["track"], "M-130");
        assert_eq!(report["status"], "scaffolded_read_only");
        assert_eq!(report["public_release_ready"], false);
        assert!(report["configured_artifacts"]
            .as_array()
            .expect("configured artifacts")
            .iter()
            .any(|artifact| artifact["name"] == "winget manifest placeholder"
                && artifact["configured"] == false));
        assert!(report["missing_steps"]
            .as_array()
            .expect("missing steps")
            .iter()
            .any(|step| step == "winget manifest placeholder"));
    }

    #[test]
    fn write_lab_evidence_remains_disposable_image_only() {
        let report = write_lab_evidence_report();

        assert_eq!(report.track, "M-132");
        assert_eq!(report.status, WriteLabEvidenceStatus::DisposableImageOnly);
        assert!(!report.physical_media_enabled);
        assert!(report.crash_injection_required);
        assert!(report
            .safety_constraints
            .iter()
            .any(|constraint| constraint == "no physical-device writes"));
    }

    #[test]
    fn windows_write_governance_remains_blocked_until_write_lab_evidence() {
        let report = windows_write_beta_governance_report();

        assert_eq!(
            report.status,
            WindowsWriteBetaGovernanceStatus::BlockedUntilAcceptedWriteLabEvidence
        );
        assert!(report
            .required_gates
            .iter()
            .any(|gate| gate == "production_claim_guard"));
        windows_write_governance().expect("windows write governance evidence");
    }

    #[test]
    fn repair_governance_remains_blocked_until_destructive_test_evidence() {
        let report = repair_governance_report();

        assert_eq!(
            report["status"],
            "blocked_until_accepted_destructive_test_evidence"
        );
        assert!(report["required_gates"]
            .as_array()
            .expect("required gates")
            .iter()
            .any(|gate| gate == "safety_case_check"));
        assert!(report["safety_constraints"]
            .as_array()
            .expect("safety constraints")
            .iter()
            .any(|constraint| constraint == "no physical-device writes"));
        repair_governance().expect("repair governance evidence");
    }

    #[test]
    fn format_governance_remains_blocked_until_destructive_test_evidence() {
        let report = format_governance_report();

        assert_eq!(
            report["status"],
            "blocked_until_accepted_destructive_test_evidence"
        );
        assert!(report["required_gates"]
            .as_array()
            .expect("required gates")
            .iter()
            .any(|gate| gate == "format_governance_audit"));
        assert!(report["safety_constraints"]
            .as_array()
            .expect("safety constraints")
            .iter()
            .any(|constraint| constraint == "no physical-device writes"));
        format_governance().expect("format governance evidence");
    }

    #[test]
    fn long_running_hardening_remains_scaffolded_read_only() {
        let report = long_running_hardening_report();

        assert_eq!(report["status"], "scaffolded_read_only");
        assert!(!report["release_gate_ready"].as_bool().unwrap_or(true));
        assert!(report["required_gates"]
            .as_array()
            .expect("required gates")
            .iter()
            .any(|gate| gate == "profiling_budget_check"));
        assert!(report["safety_constraints"]
            .as_array()
            .expect("safety constraints")
            .iter()
            .any(|constraint| constraint == "no production claim without sustained gate evidence"));
        long_running_hardening().expect("long-running hardening evidence");
    }

    #[test]
    fn branch_protection_governance_remains_admin_readiness_only() {
        let report = branch_protection_governance_report();

        assert_eq!(report["track"], "M-137");
        assert_eq!(report["status"], "admin_readiness_only");
        assert!(report["required_checks"]
            .as_array()
            .expect("required checks")
            .iter()
            .any(|check| check == "cargo xtask registry-check"));
        assert!(report["external_permissions"]
            .as_array()
            .expect("external permissions")
            .iter()
            .any(|permission| permission == "repository administration permission"));
        branch_protection_governance().expect("branch protection governance evidence");
    }

    #[test]
    fn renovate_lifecycle_remains_hosted_and_dependabot_free() {
        let report = renovate_lifecycle_report();

        assert_eq!(report["track"], "M-138");
        assert_eq!(report["status"], "hosted_renovate_ready");
        assert_eq!(report["renovate_configured"], true);
        assert_eq!(report["renovate_mode"], "silent");
        assert_eq!(report["silent_mode_enabled"], true);
        assert_eq!(report["dependabot_config_present"], false);
        assert!(report["required_managers"]
            .as_array()
            .expect("required managers")
            .iter()
            .any(|manager| manager == "github-actions"));
        renovate_lifecycle_audit().expect("renovate lifecycle evidence");
    }
}
