#![forbid(unsafe_code)]

use anyhow::{Context, Result};
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
    /// Validate release automation configuration.
    ReleaseAutomationAudit,
    /// Run the aggregate bleeding-edge repo hardening audit.
    BleedingEdgeRepoAudit,
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
    /// Write a release-evidence scaffold.
    ReleaseEvidence,
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
        Command::ReleaseAutomationAudit => {
            run_python_tool("tools/release_automation_audit.py", &[])
        }
        Command::BleedingEdgeRepoAudit => run_python_tool("tools/bleeding_edge_repo_audit.py", &[]),
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
    let mut index = Vec::new();
    for (idx, issue) in issues.iter().enumerate() {
        let title = issue
            .get("title")
            .and_then(JsonValue::as_str)
            .unwrap_or("real fixture mismatch");
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
        let track_id = format!(
            "generated-{:04}-{}",
            idx + 1,
            if slug.is_empty() {
                "real-fixture-mismatch".to_owned()
            } else {
                slug
            }
        );
        let track_dir = out_dir.join("conductor/tracks").join(&track_id);
        fs::create_dir_all(&track_dir)?;
        fs::write(track_dir.join("metadata.json"), format!("{{\n  \"track_id\": \"{track_id}\",\n  \"source\": \"real-fixture-feedback\",\n  \"status\": \"generated\"\n}}\n"))?;
        fs::write(track_dir.join("spec.md"), format!("# Generated Spec: {}\n\nSource feedback issue generated from `{}`.\n\n```json\n{}\n```\n", title, feedback_json.display(), serde_json::to_string_pretty(issue)?))?;
        fs::write(track_dir.join("plan.md"), "# Generated Plan\n\n1. Reproduce the mismatch with the referenced real fixture.\n2. Add or adjust a synthetic fixture only if it preserves clean-room safety.\n3. Implement the smallest parser correction.\n4. Update Codev and Conductor reviews.\n")?;
        index.push(JsonValue::String(track_id));
    }
    fs::write(
        out_dir.join("task-index.json"),
        serde_json::to_string_pretty(&index)? + "\n",
    )?;
    println!(
        "promote-feedback: generated {} track stubs in {}",
        issues.len(),
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

fn release_evidence() -> Result<()> {
    let dir = PathBuf::from("target/release-evidence");
    fs::create_dir_all(&dir)?;
    fs::write(
        dir.join("README.md"),
        "# APFS-RS Release Evidence\n\nGenerated scaffold. This is not production release evidence yet.\n",
    )?;
    println!("release-evidence: wrote {}", dir.display());
    Ok(())
}
