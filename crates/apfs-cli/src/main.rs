#![forbid(unsafe_code)]

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use apfs_blockdev::ImageBlockDevice;
use apfs_core::{
    btree_cursor_report_in_device, directory_report_in_device, file_read_report_in_device,
    inspect_device, lookup_object_in_device, read_mapped_object_in_device,
    resolver_report_in_device, volume_report_in_device, BTreeCursorStatus, DirectoryReportStatus,
    FileReadReportStatus, InspectReport, InspectStatus, MappedObjectReadStatus, ObjectLookupStatus,
    ObjectMapResolverStatus, VolumeReportStatus,
};
use apfs_features::{analyze_unicode_case_policy, feature_readiness, metadata_feature_report};
use apfs_win::{plan_read_only_mount, winfsp_readonly_callback_matrix};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "apfs")]
#[command(about = "Clean-room APFS inspection tooling")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Inspect an APFS source image read-only.
    Inspect {
        /// Source image path. Raw physical devices are intentionally not supported in this slice.
        source: PathBuf,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Produce the same safety-oriented report as inspect, named for user workflows.
    CompatibilityReport {
        /// Source image path.
        source: PathBuf,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Run a read-only diagnostic doctor over the currently implemented inspection surfaces.
    Doctor {
        /// Source image path.
        source: PathBuf,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Export a redacted diagnostic bundle for bug reports.
    DiagnosticsExport {
        /// Source image path.
        source: PathBuf,
        /// Output directory for the redacted bundle.
        #[arg(long)]
        out: PathBuf,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Lookup an object ID through the current checkpoint-map-backed synthetic OMAP lookup path.
    LookupObject {
        /// Source image path.
        source: PathBuf,
        /// Object identifier to look up.
        #[arg(long)]
        oid: u64,
        /// Transaction ID to resolve at or before.
        #[arg(long)]
        xid: u64,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Report APFS volume superblocks resolved through the current object-map resolver.
    Volumes {
        /// Source image path.
        source: PathBuf,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },

    /// Report the currently available object-map resolver mode without performing a lookup.
    ResolverReport {
        /// Source image path.
        source: PathBuf,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },

    /// Produce a production-shaped B-tree cursor report for a requested OMAP key.
    BtreeCursorReport {
        /// Source image path.
        source: PathBuf,
        /// Object identifier to seek.
        #[arg(long)]
        oid: u64,
        /// Transaction ID to seek at or before.
        #[arg(long)]
        xid: u64,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Resolve an object through the current object-map resolver and read its mapped block header/checksum.
    ReadObject {
        /// Source image path.
        source: PathBuf,
        /// Object identifier to read.
        #[arg(long)]
        oid: u64,
        /// Transaction ID to resolve at or before.
        #[arg(long)]
        xid: u64,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// List entries from the current synthetic filesystem root tree.
    Ls {
        /// Source image path.
        source: PathBuf,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Preview a file from the current synthetic directory/extent scaffold.
    Cat {
        /// Source image path.
        source: PathBuf,
        /// Synthetic file name to preview.
        #[arg(long)]
        name: String,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Report metadata for one synthetic directory entry.
    Stat {
        /// Source image path.
        source: PathBuf,
        /// Synthetic file or directory name.
        #[arg(long)]
        name: String,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Produce the WinFsp read-only callback contract matrix without mounting.
    WinfspCallbackMatrix {
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Produce a Windows read-only WinFsp mount preflight plan without mounting.
    MountPlan {
        /// Source image path.
        source: PathBuf,
        /// Windows drive-letter mount point, for example X:.
        #[arg(long)]
        mountpoint: String,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Export a redacted diagnostics bundle for bug reports and agent workflows.
    DiagnosticsBundle {
        /// Source image path.
        source: PathBuf,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Report host/path Unicode and case-sensitivity policy for one APFS name component.
    PathPolicy {
        /// One APFS name component, not a full path.
        #[arg(long)]
        name: String,
        /// Treat the source volume as case-sensitive for policy purposes.
        #[arg(long, default_value_t = true)]
        case_sensitive: bool,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Report readiness for advanced APFS feature families such as compression or xattrs.
    FeatureReadiness {
        /// Feature family, for example compression, xattrs, sparse-clone, snapshots, encryption.
        #[arg(long)]
        feature: String,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Report the synthetic metadata model for xattrs/resource forks/sparse clone scaffolds.
    MetadataFeatureReport {
        /// Feature family, for example xattrs or sparse-clone.
        #[arg(long)]
        feature: String,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Extract one small synthetic direct-block file preview to a destination directory.
    Extract {
        /// Source image path.
        source: PathBuf,
        /// Synthetic file name to extract.
        #[arg(long)]
        name: String,
        /// Destination directory.
        #[arg(long)]
        dest: PathBuf,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Inspect { source, json } | Command::CompatibilityReport { source, json } => {
            inspect_command(source, json)
        }
        Command::Doctor { source, json } => doctor_command(source, json),
        Command::DiagnosticsExport { source, out, json } => {
            diagnostics_export_command(source, out, json)
        }
        Command::LookupObject {
            source,
            oid,
            xid,
            json,
        } => lookup_object_command(source, oid, xid, json),
        Command::Volumes { source, json } => volumes_command(source, json),
        Command::ResolverReport { source, json } => resolver_report_command(source, json),
        Command::BtreeCursorReport {
            source,
            oid,
            xid,
            json,
        } => btree_cursor_report_command(source, oid, xid, json),
        Command::ReadObject {
            source,
            oid,
            xid,
            json,
        } => read_object_command(source, oid, xid, json),
        Command::Ls { source, json } => ls_command(source, json),
        Command::Cat { source, name, json } => cat_command(source, name, json),
        Command::Stat { source, name, json } => stat_command(source, name, json),
        Command::WinfspCallbackMatrix { json } => winfsp_callback_matrix_command(json),
        Command::MountPlan {
            source,
            mountpoint,
            json,
        } => mount_plan_command(source, mountpoint, json),
        Command::DiagnosticsBundle { source, json } => diagnostics_bundle_command(source, json),
        Command::PathPolicy {
            name,
            case_sensitive,
            json,
        } => path_policy_command(name, case_sensitive, json),
        Command::FeatureReadiness { feature, json } => feature_readiness_command(feature, json),
        Command::MetadataFeatureReport { feature, json } => {
            metadata_feature_report_command(feature, json)
        }
        Command::Extract {
            source,
            name,
            dest,
            json,
        } => extract_command(source, name, dest, json),
    }
}

fn inspect_command(source: PathBuf, json: bool) -> anyhow::Result<()> {
    let device = ImageBlockDevice::open(&source)
        .with_context(|| format!("open {} read-only", source.display()))?;
    let report = inspect_device(&device).context("inspect source")?;

    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }

    print_human_report(&source, &report);

    if matches!(report.status, InspectStatus::Refused) {
        anyhow::bail!("inspect refused; rerun with --json for structured diagnostics");
    }

    Ok(())
}

fn doctor_command(source: PathBuf, json: bool) -> anyhow::Result<()> {
    let device = ImageBlockDevice::open(&source)
        .with_context(|| format!("open {} read-only", source.display()))?;
    let inspect = inspect_device(&device).context("doctor inspect source")?;
    let volume_report = volume_report_in_device(&device).ok();
    let resolver_report = resolver_report_in_device(&device).ok();
    let directory_report = directory_report_in_device(&device).ok();

    let mut blockers: Vec<String> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();
    match inspect.status {
        InspectStatus::ApfsContainerDetected => {}
        InspectStatus::NotApfs => blockers.push(
            "source is not recognised as an APFS container or GPT-wrapped APFS image".to_owned(),
        ),
        InspectStatus::Refused => blockers.push("initial APFS inspection was refused".to_owned()),
    }
    if volume_report
        .as_ref()
        .is_none_or(|report| !matches!(report.status, VolumeReportStatus::Available))
    {
        warnings.push("volume report is unavailable or synthetic-only".to_owned());
    }
    if resolver_report
        .as_ref()
        .is_none_or(|report| !matches!(report.status, ObjectMapResolverStatus::Available))
    {
        warnings.push("object-map resolver is unavailable or synthetic-only".to_owned());
    }
    if directory_report
        .as_ref()
        .is_none_or(|report| !matches!(report.status, DirectoryReportStatus::Available))
    {
        warnings.push("directory listing is unavailable or synthetic-only".to_owned());
    }

    let readiness = if blockers.is_empty() && warnings.is_empty() {
        "synthetic_ready"
    } else if blockers.is_empty() {
        "diagnostic_only"
    } else {
        "blocked"
    };

    let envelope = serde_json::json!({
        "schema_version": "0.18.0",
        "command": "doctor",
        "source_kind": "image",
        "source_name_redacted": source.file_name().and_then(|name| name.to_str()).unwrap_or("<source>"),
        "readiness": readiness,
        "blockers": blockers,
        "warnings": warnings,
        "inspect_status": format!("{:?}", inspect.status),
        "volume_status": volume_report.as_ref().map(|report| format!("{:?}", report.status)),
        "resolver_status": resolver_report.as_ref().map(|report| format!("{:?}", report.status)),
        "directory_status": directory_report.as_ref().map(|report| format!("{:?}", report.status)),
        "implemented_scope": "synthetic/read-only inspection and scaffolded resolver/directory/file-preview paths",
        "not_implemented": [
            "production APFS B-tree traversal",
            "production file extraction",
            "Windows WinFsp mount adapter",
            "compression",
            "encryption",
            "write support",
            "repair",
            "format"
        ],
        "safety": {
            "read_only": true,
            "raw_physical_device_access": false,
            "writes_to_apfs_media": false
        }
    });

    if json {
        println!("{}", serde_json::to_string_pretty(&envelope)?);
        return Ok(());
    }

    println!("APFS-RS doctor report");
    println!("source: {}", source.display());
    println!("readiness: {readiness}");
    if let Some(items) = envelope
        .get("blockers")
        .and_then(serde_json::Value::as_array)
    {
        for item in items {
            println!(
                "blocker: {}",
                item.as_str().unwrap_or("<non-string blocker>")
            );
        }
    }
    if let Some(items) = envelope
        .get("warnings")
        .and_then(serde_json::Value::as_array)
    {
        for item in items {
            println!(
                "warning: {}",
                item.as_str().unwrap_or("<non-string warning>")
            );
        }
    }
    Ok(())
}

fn diagnostics_export_command(source: PathBuf, out: PathBuf, json: bool) -> anyhow::Result<()> {
    let device = ImageBlockDevice::open(&source)
        .with_context(|| format!("open {} read-only", source.display()))?;
    let inspect = inspect_device(&device).context("diagnostic inspect source")?;
    let volume_report = volume_report_in_device(&device).ok();
    let resolver_report = resolver_report_in_device(&device).ok();
    let directory_report = directory_report_in_device(&device).ok();

    fs::create_dir_all(&out)
        .with_context(|| format!("create diagnostics output directory {}", out.display()))?;
    let bundle_path = out.join("apfs-diagnostics-redacted.json");
    let bundle = serde_json::json!({
        "schema_version": "0.18.0",
        "bundle_kind": "apfs-rs-redacted-diagnostics",
        "source": {
            "kind": "image",
            "file_name_redacted": source.file_name().and_then(|name| name.to_str()).unwrap_or("<source>"),
            "full_path_included": false
        },
        "inspect": inspect,
        "volume_report": volume_report,
        "resolver_report": resolver_report,
        "directory_report": directory_report,
        "redaction": {
            "contains_file_contents": false,
            "contains_passwords_or_keys": false,
            "contains_raw_apfs_blocks": false,
            "contains_full_source_path": false,
            "contains_personal_data_by_design": false
        },
        "safety_note": "This diagnostic export reads the source image and writes a redacted host-side JSON bundle only. It never writes to APFS media."
    });
    fs::write(&bundle_path, serde_json::to_string_pretty(&bundle)? + "\n")
        .with_context(|| format!("write diagnostics bundle {}", bundle_path.display()))?;

    let envelope = serde_json::json!({
        "schema_version": "0.18.0",
        "status": "written",
        "bundle_path": bundle_path.display().to_string(),
        "redacted": true,
        "writes_to_apfs_media": false
    });

    if json {
        println!("{}", serde_json::to_string_pretty(&envelope)?);
        return Ok(());
    }

    println!("APFS-RS redacted diagnostics export");
    println!("wrote: {}", bundle_path.display());
    Ok(())
}

fn lookup_object_command(source: PathBuf, oid: u64, xid: u64, json: bool) -> anyhow::Result<()> {
    let device = ImageBlockDevice::open(&source)
        .with_context(|| format!("open {} read-only", source.display()))?;
    let report = lookup_object_in_device(&device, oid, xid).context("lookup object")?;

    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }

    println!("APFS-RS object lookup report");
    println!("source: {}", source.display());
    println!("status: {:?}", report.status);
    println!("requested oid: {}", report.requested_oid);
    println!("requested xid: {}", report.requested_xid);
    if let Some(resolver) = &report.resolver {
        println!("resolver mode: {:?}", resolver.mode);
        println!("resolver strategy: {}", resolver.lookup_strategy);
    }
    if let Some(lookup) = &report.lookup {
        if lookup.matched {
            println!("matched xid: {}", lookup.matched_xid.unwrap_or_default());
            println!(
                "physical block: {}",
                lookup.physical_address.unwrap_or_default()
            );
            println!("size bytes: {}", lookup.size_bytes.unwrap_or_default());
        }
    }
    for warning in &report.warnings {
        eprintln!("warning {}: {}", warning.code, warning.message);
    }
    for error in &report.errors {
        eprintln!("error {}: {}", error.code, error.message);
    }

    if !matches!(report.status, ObjectLookupStatus::Found) {
        anyhow::bail!(
            "object lookup did not find a record; rerun with --json for structured diagnostics"
        );
    }

    Ok(())
}

fn volumes_command(source: PathBuf, json: bool) -> anyhow::Result<()> {
    let device = ImageBlockDevice::open(&source)
        .with_context(|| format!("open {} read-only", source.display()))?;
    let report = volume_report_in_device(&device).context("build APFS volume report")?;

    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }

    println!("APFS-RS volume report");
    println!("source: {}", source.display());
    println!("status: {:?}", report.status);
    println!("volume count: {}", report.volume_count);
    for probe in &report.probes {
        println!("filesystem oid: {}", probe.filesystem_oid);
        if let Some(volume) = &probe.volume {
            println!("  name: {}", volume.volume_name);
            println!("  role: 0x{:04x}", volume.role);
            println!("  root tree oid: {}", volume.root_tree_oid);
            println!("  files: {}", volume.num_files);
            println!("  directories: {}", volume.num_directories);
        }
        for error in &probe.errors {
            eprintln!("  error {}: {}", error.code, error.message);
        }
    }
    for warning in &report.warnings {
        eprintln!("warning {}: {}", warning.code, warning.message);
    }
    for error in &report.errors {
        eprintln!("error {}: {}", error.code, error.message);
    }

    if !matches!(report.status, VolumeReportStatus::Available) {
        anyhow::bail!(
            "volume report is not available; rerun with --json for structured diagnostics"
        );
    }

    Ok(())
}

fn resolver_report_command(source: PathBuf, json: bool) -> anyhow::Result<()> {
    let device = ImageBlockDevice::open(&source)
        .with_context(|| format!("open {} read-only", source.display()))?;
    let report = resolver_report_in_device(&device).context("build object-map resolver report")?;

    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }

    println!("APFS-RS object-map resolver report");
    println!("source: {}", source.display());
    println!("status: {:?}", report.status);
    if let Some(resolver) = &report.resolver {
        println!("mode: {:?}", resolver.mode);
        println!("root block: {}", resolver.root_block_index);
        println!("root level: {}", resolver.root_level);
        println!("index records: {}", resolver.index_record_count);
        println!(
            "additional leaf nodes: {}",
            resolver.additional_leaf_node_count
        );
        println!("aggregate records: {}", resolver.aggregate_record_count);
        println!("lookup strategy: {}", resolver.lookup_strategy);
    }
    for warning in &report.warnings {
        eprintln!("warning {}: {}", warning.code, warning.message);
    }
    for error in &report.errors {
        eprintln!("error {}: {}", error.code, error.message);
    }

    if !matches!(report.status, ObjectMapResolverStatus::Available) {
        anyhow::bail!(
            "object-map resolver is not available; rerun with --json for structured diagnostics"
        );
    }

    Ok(())
}

fn btree_cursor_report_command(
    source: PathBuf,
    oid: u64,
    xid: u64,
    json: bool,
) -> anyhow::Result<()> {
    let device = ImageBlockDevice::open(&source)
        .with_context(|| format!("open {} read-only", source.display()))?;
    let report =
        btree_cursor_report_in_device(&device, oid, xid).context("build B-tree cursor report")?;

    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }

    println!("APFS-RS B-tree cursor report");
    println!("source: {}", source.display());
    println!("status: {:?}", report.status);
    if let Some(cursor) = &report.cursor {
        println!("mode: {:?}", cursor.mode);
        println!("key kind: {}", cursor.key_kind);
        println!("root block: {}", cursor.root_block_index);
        println!("root oid: {}", cursor.root_oid);
        println!("root level: {}", cursor.root_level);
        println!("steps: {}", cursor.steps.len());
        if cursor.lookup.matched {
            println!(
                "matched xid: {}",
                cursor.lookup.matched_xid.unwrap_or_default()
            );
            println!(
                "physical block: {}",
                cursor.lookup.physical_address.unwrap_or_default()
            );
        }
    }
    for warning in &report.warnings {
        eprintln!("warning {}: {}", warning.code, warning.message);
    }
    for error in &report.errors {
        eprintln!("error {}: {}", error.code, error.message);
    }

    if !matches!(report.status, BTreeCursorStatus::Available) {
        anyhow::bail!(
            "B-tree cursor is not available; rerun with --json for structured diagnostics"
        );
    }

    Ok(())
}

fn read_object_command(source: PathBuf, oid: u64, xid: u64, json: bool) -> anyhow::Result<()> {
    let device = ImageBlockDevice::open(&source)
        .with_context(|| format!("open {} read-only", source.display()))?;
    let report = read_mapped_object_in_device(&device, oid, xid).context("read mapped object")?;

    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }

    println!("APFS-RS mapped object read report");
    println!("source: {}", source.display());
    println!("status: {:?}", report.status);
    println!("requested oid: {}", report.requested_oid);
    println!("requested xid: {}", report.requested_xid);
    if let Some(object) = &report.object {
        println!("physical block: {}", object.physical_block_index);
        println!("object oid: {}", object.header.oid);
        println!("object xid: {}", object.header.xid);
        println!("object type: {}", object.object_type_name);
        println!("checksum valid: {}", object.checksum_valid);
    }
    for warning in &report.warnings {
        eprintln!("warning {}: {}", warning.code, warning.message);
    }
    for error in &report.errors {
        eprintln!("error {}: {}", error.code, error.message);
    }

    if matches!(report.status, MappedObjectReadStatus::Refused) {
        anyhow::bail!("mapped object read refused; rerun with --json for structured diagnostics");
    }

    Ok(())
}

fn ls_command(source: PathBuf, json: bool) -> anyhow::Result<()> {
    let device = ImageBlockDevice::open(&source)
        .with_context(|| format!("open {} read-only", source.display()))?;
    let report = directory_report_in_device(&device).context("directory report")?;

    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }

    println!("APFS-RS synthetic directory report");
    println!("source: {}", source.display());
    println!("status: {:?}", report.status);
    println!(
        "volume: {}",
        report.volume_name.as_deref().unwrap_or("<unknown>")
    );
    for entry in &report.entries {
        println!(
            "{}\t{}\t{}\t{}",
            entry.object_id, entry.item_kind_raw, entry.logical_size, entry.name
        );
    }
    for warning in &report.warnings {
        eprintln!("warning {}: {}", warning.code, warning.message);
    }
    for error in &report.errors {
        eprintln!("error {}: {}", error.code, error.message);
    }

    if !matches!(report.status, DirectoryReportStatus::Available) {
        anyhow::bail!("directory report unavailable; rerun with --json for structured diagnostics");
    }
    Ok(())
}

fn cat_command(source: PathBuf, name: String, json: bool) -> anyhow::Result<()> {
    let device = ImageBlockDevice::open(&source)
        .with_context(|| format!("open {} read-only", source.display()))?;
    let report = file_read_report_in_device(&device, &name).context("synthetic file preview")?;

    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }

    println!("APFS-RS synthetic file preview");
    println!("source: {}", source.display());
    println!("status: {:?}", report.status);
    println!("name: {}", report.requested_name);
    if let Some(preview) = &report.content_preview_utf8 {
        println!("{preview}");
    }
    for warning in &report.warnings {
        eprintln!("warning {}: {}", warning.code, warning.message);
    }
    for error in &report.errors {
        eprintln!("error {}: {}", error.code, error.message);
    }

    if matches!(report.status, FileReadReportStatus::Refused) {
        anyhow::bail!("file preview refused; rerun with --json for structured diagnostics");
    }
    Ok(())
}

fn winfsp_callback_matrix_command(json: bool) -> anyhow::Result<()> {
    let matrix = winfsp_readonly_callback_matrix();
    if json {
        println!("{}", serde_json::to_string_pretty(&matrix)?);
        return Ok(());
    }
    println!("APFS-RS WinFsp read-only callback matrix");
    for callback in &matrix.callbacks {
        println!(
            "{}\t{}\t{}",
            callback.callback, callback.decision, callback.note
        );
    }
    println!("safety: {}", matrix.safety_note);
    Ok(())
}

fn mount_plan_command(source: PathBuf, mountpoint: String, json: bool) -> anyhow::Result<()> {
    let plan = plan_read_only_mount(&source.display().to_string(), &mountpoint);
    if json {
        println!("{}", serde_json::to_string_pretty(&plan)?);
        return Ok(());
    }

    println!("APFS-RS Windows read-only mount plan");
    println!("source: {}", source.display());
    println!("mount point: {}", mountpoint);
    println!("status: {:?}", plan.status);
    println!("adapter: {}", plan.adapter);
    println!("read-only: {}", plan.read_only);
    println!("WinFsp required: {}", plan.winfsp_required);
    println!("allowed operations: {}", plan.allowed_operations.join(", "));
    println!("refused operations: {}", plan.refused_operations.join(", "));
    for warning in &plan.warnings {
        eprintln!("warning: {warning}");
    }
    for error in &plan.errors {
        eprintln!("error: {error}");
    }
    Ok(())
}

fn diagnostics_bundle_command(source: PathBuf, json: bool) -> anyhow::Result<()> {
    let device = ImageBlockDevice::open(&source)
        .with_context(|| format!("open {} read-only", source.display()))?;
    let inspect = inspect_device(&device).context("inspect for diagnostics bundle")?;
    let resolver = resolver_report_in_device(&device).ok();
    let volumes = volume_report_in_device(&device).ok();
    let directory = directory_report_in_device(&device).ok();
    let bundle = serde_json::json!({
        "schema_version": "0.18.0",
        "bundle_kind": "redacted_diagnostics",
        "source": {
            "kind": "image",
            "display_name": source.file_name().and_then(|name| name.to_str()).unwrap_or("<redacted>"),
            "size_bytes": inspect.source_size_bytes
        },
        "inspect": {
            "status": inspect.status,
            "layout": inspect.layout,
            "apfs_offset_bytes": inspect.apfs_offset_bytes,
            "container": inspect.container.as_ref().map(|container| serde_json::json!({
                "block_size": container.block_size,
                "block_count": container.block_count,
                "features": container.features,
                "readonly_compatible_features": container.readonly_compatible_features,
                "incompatible_features": container.incompatible_features,
                "filesystem_oid_count": container.filesystem_oids.len(),
                "next_xid": container.next_xid,
                "checkpoint_descriptor_base": container.checkpoint_descriptor_base,
                "checkpoint_descriptor_len": container.checkpoint_descriptor_len,
                "checkpoint_data_base": container.checkpoint_data_base,
                "checkpoint_data_len": container.checkpoint_data_len,
                "omap_oid": container.omap_oid
            })),
            "checkpoint_scan": inspect.checkpoint_scan.as_ref().map(|scan| serde_json::json!({
                "candidate_count": scan.candidates.len(),
                "checkpoint_map_count": scan.checkpoint_maps.len(),
                "latest_valid_xid": scan.latest_valid_xid,
                "container_object_map_available": scan.container_object_map.is_some()
            })),
            "errors": inspect.errors,
            "warnings": inspect.warnings
        },
        "resolver": resolver.as_ref().map(|envelope| serde_json::json!({
            "status": envelope.status,
            "mode": envelope.resolver.as_ref().map(|resolver| resolver.mode),
            "aggregate_record_count": envelope.resolver.as_ref().map(|resolver| resolver.aggregate_record_count)
        })),
        "volumes": volumes.as_ref().map(|envelope| serde_json::json!({
            "status": envelope.status,
            "volume_count": envelope.volume_count,
            "probe_count": envelope.probes.len()
        })),
        "directory": directory.as_ref().map(|envelope| serde_json::json!({
            "status": envelope.status,
            "entry_count": envelope.entry_count,
            "names_redacted": true
        })),
        "redaction": {
            "file_names_redacted": true,
            "file_contents_included": false,
            "secrets_included": false,
            "raw_blocks_included": false
        },
        "safety_note": "Diagnostics are read-only and redacted by default. No APFS media is modified."
    });

    if json {
        println!("{}", serde_json::to_string_pretty(&bundle)?);
        return Ok(());
    }

    println!("APFS-RS redacted diagnostics bundle");
    println!("source: {}", source.display());
    println!("inspect status: {:?}", inspect.status);
    println!("redaction: file names redacted; file contents excluded; secrets excluded; raw blocks excluded");
    Ok(())
}

fn stat_command(source: PathBuf, name: String, json: bool) -> anyhow::Result<()> {
    let device = ImageBlockDevice::open(&source)
        .with_context(|| format!("open {} read-only", source.display()))?;
    let report = directory_report_in_device(&device).context("directory report for stat")?;
    let entry = report
        .entries
        .iter()
        .find(|entry| entry.name == name)
        .cloned();
    let status = if entry.is_some() {
        "found"
    } else if matches!(report.status, DirectoryReportStatus::Available) {
        "not_found"
    } else {
        "refused"
    };
    let envelope = serde_json::json!({
        "schema_version": "0.18.0",
        "source_kind": "image",
        "status": status,
        "requested_name": name,
        "volume_name": report.volume_name,
        "entry": entry,
        "errors": report.errors,
        "warnings": report.warnings,
        "safety": report.safety,
        "note": "Synthetic metadata report; production APFS inode/stat decoding is not implemented yet."
    });

    if json {
        println!("{}", serde_json::to_string_pretty(&envelope)?);
        return Ok(());
    }

    println!("APFS-RS synthetic stat report");
    println!("source: {}", source.display());
    println!("status: {status}");
    if let Some(entry) = envelope.get("entry") {
        println!("entry: {}", serde_json::to_string_pretty(entry)?);
    }
    if status != "found" {
        anyhow::bail!("stat did not find a synthetic directory entry; rerun with --json for structured diagnostics");
    }
    Ok(())
}

fn path_policy_command(name: String, case_sensitive: bool, json: bool) -> anyhow::Result<()> {
    let report = analyze_unicode_case_policy(&name, case_sensitive);
    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }

    println!("APFS-RS path policy report");
    println!("name: {}", report.normalized_for_display);
    println!("case-sensitive volume: {}", report.case_sensitive_volume);
    println!("safe for host path: {}", report.safe_for_host_path);
    println!("lookup policy: {}", report.lookup_policy);
    for warning in &report.warnings {
        eprintln!("warning: {warning}");
    }
    for error in &report.errors {
        eprintln!("error: {error}");
    }
    if !report.safe_for_host_path {
        anyhow::bail!(
            "path policy refused this APFS name component; rerun with --json for details"
        );
    }
    Ok(())
}

fn feature_readiness_command(feature: String, json: bool) -> anyhow::Result<()> {
    let report = feature_readiness(&feature);
    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }

    println!("APFS-RS feature readiness report");
    println!("feature: {}", report.feature);
    println!("status: {:?}", report.status);
    println!("next track: {}", report.next_track);
    for step in &report.missing_production_steps {
        println!("missing: {step}");
    }
    Ok(())
}

fn metadata_feature_report_command(feature: String, json: bool) -> anyhow::Result<()> {
    let report = metadata_feature_report(&feature);
    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }

    println!("APFS-RS metadata feature report");
    println!("feature: {}", report.feature_family);
    println!("synthetic support: {}", report.synthetic_support);
    println!(
        "production read support: {}",
        report.production_read_support
    );
    println!(
        "production write support: {}",
        report.production_write_support
    );
    println!("modeled fields: {}", report.modeled_fields.join(", "));
    for missing in &report.missing_fields {
        println!("missing: {missing}");
    }
    Ok(())
}

fn extract_command(source: PathBuf, name: String, dest: PathBuf, json: bool) -> anyhow::Result<()> {
    validate_safe_output_name(&name)?;
    let device = ImageBlockDevice::open(&source)
        .with_context(|| format!("open {} read-only", source.display()))?;
    let report = file_read_report_in_device(&device, &name).context("synthetic extract preview")?;
    let mut wrote_path: Option<String> = None;
    let mut wrote_bytes: Option<usize> = None;
    let mut status = "refused";

    if matches!(report.status, FileReadReportStatus::Available) {
        let Some(hex) = &report.content_preview_hex else {
            anyhow::bail!("synthetic file report did not include a hex payload preview");
        };
        let bytes = decode_hex(hex)?;
        fs::create_dir_all(&dest)
            .with_context(|| format!("create destination directory {}", dest.display()))?;
        let out_path = dest.join(&name);
        fs::write(&out_path, &bytes)
            .with_context(|| format!("write extracted preview {}", out_path.display()))?;
        wrote_bytes = Some(bytes.len());
        wrote_path = Some(out_path.display().to_string());
        status = "written";
    } else if matches!(report.status, FileReadReportStatus::NotFound) {
        status = "not_found";
    }

    let envelope = serde_json::json!({
        "schema_version": "0.18.0",
        "source_kind": "image",
        "status": status,
        "requested_name": name,
        "destination_directory": dest.display().to_string(),
        "wrote_path": wrote_path,
        "wrote_bytes": wrote_bytes,
        "file_report": report,
        "safety_note": "This writes only to the requested host destination directory; it never writes to APFS media. Current payload is a bounded synthetic direct-block preview, not production APFS extent extraction."
    });

    if json {
        println!("{}", serde_json::to_string_pretty(&envelope)?);
        return Ok(());
    }

    println!("APFS-RS synthetic extract report");
    println!("source: {}", source.display());
    println!("status: {status}");
    if let Some(path) = envelope
        .get("wrote_path")
        .and_then(serde_json::Value::as_str)
    {
        println!("wrote: {path}");
    }
    if status != "written" {
        anyhow::bail!("synthetic extraction did not write a file; rerun with --json for structured diagnostics");
    }
    Ok(())
}

fn validate_safe_output_name(name: &str) -> anyhow::Result<()> {
    let path = Path::new(name);
    if name.is_empty()
        || name.contains('/')
        || name.contains('\\')
        || name == "."
        || name == ".."
        || path.is_absolute()
        || path.components().count() != 1
    {
        anyhow::bail!("unsafe synthetic extraction name `{name}`; provide a single file name without path separators");
    }
    Ok(())
}

fn decode_hex(hex: &str) -> anyhow::Result<Vec<u8>> {
    if hex.len() % 2 != 0 {
        anyhow::bail!("hex payload has odd length");
    }
    let mut out = Vec::with_capacity(hex.len() / 2);
    let bytes = hex.as_bytes();
    for pair in bytes.chunks_exact(2) {
        let hi = hex_value(pair[0])?;
        let lo = hex_value(pair[1])?;
        out.push((hi << 4) | lo);
    }
    Ok(out)
}

fn hex_value(byte: u8) -> anyhow::Result<u8> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => anyhow::bail!("invalid hex byte `{}`", char::from(byte)),
    }
}

fn print_human_report(source: &std::path::Path, report: &InspectReport) {
    println!("APFS-RS inspect report");
    println!("source: {}", source.display());
    println!("status: {:?}", report.status);
    println!("layout: {:?}", report.layout);
    if let Some(offset) = report.apfs_offset_bytes {
        println!("APFS offset: {offset} bytes");
    }
    if let Some(container) = &report.container {
        println!("container: {}", container.uuid);
        println!("block size: {}", container.block_size);
        println!("block count: {}", container.block_count);
        println!(
            "incompatible features: 0x{:016x}",
            container.incompatible_features
        );
        println!(
            "filesystem OIDs parsed from superblock prefix: {:?}",
            container.filesystem_oids
        );
        if let Some(checksum) = &container.checksum {
            println!("object checksum valid: {}", checksum.valid);
            println!("stored checksum: {}", checksum.stored_checksum_hex);
            println!("computed checksum: {}", checksum.computed_checksum_hex);
        }
    }
    if let Some(checkpoints) = &report.checkpoint_scan {
        println!(
            "checkpoint descriptor base block: {}",
            checkpoints.descriptor_base_block
        );
        println!(
            "checkpoint descriptor len blocks: {}",
            checkpoints.descriptor_len_blocks
        );
        println!("checkpoint candidates: {}", checkpoints.candidates.len());
        println!(
            "checkpoint map blocks: {}",
            checkpoints.checkpoint_maps.len()
        );
        if let Some(xid) = checkpoints.latest_valid_xid {
            println!("latest valid checkpoint candidate xid: {xid}");
        }
        if let Some(omap) = &checkpoints.container_object_map {
            println!("container object map block: {}", omap.object_block_index);
            println!(
                "container object map tree oid: {}",
                omap.object_map.tree_oid
            );
            println!(
                "container object map checksum valid: {}",
                omap.object_map.checksum.valid
            );
            if let Some(tree_root) = &omap.tree_root {
                println!(
                    "container object map B-tree root block: {}",
                    tree_root.object_block_index
                );
                println!("B-tree root key count: {}", tree_root.node.key_count);
                println!(
                    "B-tree root checksum valid: {}",
                    tree_root.node.checksum.valid
                );
                println!(
                    "root preliminary OMAP records decoded: {}",
                    tree_root.preliminary_omap_records.len()
                );
                println!(
                    "additional mapped OMAP leaf nodes: {}",
                    tree_root.additional_mapped_leaf_nodes.len()
                );
                println!(
                    "aggregate OMAP records decoded: {}",
                    tree_root.aggregate_omap_record_count
                );
                println!("lookup samples: {}", tree_root.lookup_samples.len());
            }
        }
    }
    for warning in &report.warnings {
        eprintln!("warning {}: {}", warning.code, warning.message);
    }
    for error in &report.errors {
        eprintln!("error {}: {}", error.code, error.message);
    }
}
