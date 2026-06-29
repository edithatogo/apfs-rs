#![forbid(unsafe_code)]

use apfs_blockdev::{BlockDeviceError, ReadOnlyBlockDevice};
use apfs_types::{
    gpt_entries_byte_len, lookup_omap_record, parse_btree_node_with_checksum, parse_checkpoint_map_block_with_checksum, parse_gpt_header,
    parse_gpt_partition_entry, parse_nx_superblock, parse_nx_superblock_with_checksum, parse_object_header, parse_omap_index_records_from_btree_node,
    parse_apfs_volume_superblock_with_checksum, parse_omap_phys_with_checksum, parse_omap_records_from_btree_node, parse_synthetic_directory_records_from_btree_node, select_synthetic_btree_child, validate_gpt_entries_checksum, validate_object_checksum,
    BTreeChildSelection, BTreeIndexRecord, BTreeNode, CheckpointMapping, ContainerSuperblock, FileSystemDirectoryRecord, GptEntriesChecksum, GptHeader,
    GptPartitionEntry, ObjectChecksum, ObjectHeader, ObjectMap, OmapLookup, OmapRecord, ParseError, VolumeSuperblock, GPT_SECTOR_SIZE, NX_SUPERBLOCK_MIN_SIZE,
    OBJECT_TYPE_BTREE, OBJECT_TYPE_BTREE_NODE, OBJECT_TYPE_CHECKPOINT_MAP, OBJECT_TYPE_NX_SUPERBLOCK, OBJECT_TYPE_OMAP,
};
use serde::Serialize;
use thiserror::Error;

const DEFAULT_APFS_PROBE_BYTES: usize = 4096;
const MAX_GPT_ENTRIES_BYTES: usize = 16 * 1024 * 1024;
const MAX_CHECKPOINT_SCAN_BLOCKS: u32 = 256;

#[derive(Debug, Error)]
pub enum InspectError {
    #[error(transparent)]
    BlockDevice(#[from] BlockDeviceError),
    #[error(transparent)]
    Parse(#[from] ParseError),
    #[error("GPT partition entries are too large to read safely: {0} bytes")]
    GptEntriesTooLarge(usize),
    #[error("arithmetic overflow while calculating byte offset")]
    ArithmeticOverflow,
    #[error("APFS block size {0} is too large for this inspection build")]
    BlockSizeTooLarge(u32),
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InspectStatus {
    ApfsContainerDetected,
    NotApfs,
    Refused,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SourceLayout {
    DirectContainerAtBlockZero,
    GptWithApfsPartition,
    Unknown,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ObjectLookupStatus {
    Found,
    NotFound,
    Refused,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ObjectMapResolverStatus {
    Available,
    Unavailable,
    Refused,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ObjectMapResolverMode {
    BoundedSyntheticTwoLevelTraversal,
    AggregateDecodedRecords,
    Unavailable,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VolumeReportStatus {
    Available,
    Unavailable,
    Refused,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DirectoryReportStatus {
    Available,
    Unavailable,
    Refused,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FileReadReportStatus {
    Available,
    NotFound,
    Refused,
}

#[derive(Debug, Clone, Serialize)]
pub struct InspectReport {
    pub schema_version: String,
    pub source_kind: String,
    pub source_size_bytes: u64,
    pub status: InspectStatus,
    pub layout: SourceLayout,
    pub apfs_offset_bytes: Option<u64>,
    pub gpt: Option<GptReport>,
    pub container: Option<ContainerSuperblock>,
    pub checkpoint_scan: Option<CheckpointScanReport>,
    pub errors: Vec<Diagnostic>,
    pub warnings: Vec<Diagnostic>,
    pub safety: SafetySummary,
}

#[derive(Debug, Clone, Serialize)]
pub struct ObjectLookupReport {
    pub schema_version: String,
    pub source_kind: String,
    pub source_size_bytes: u64,
    pub status: ObjectLookupStatus,
    pub requested_oid: u64,
    pub requested_xid: u64,
    pub lookup: Option<OmapLookup>,
    pub traversal: Option<SyntheticBTreeTraversalReport>,
    pub resolver: Option<ObjectMapResolverReport>,
    pub errors: Vec<Diagnostic>,
    pub warnings: Vec<Diagnostic>,
    pub safety: SafetySummary,
}

#[derive(Debug, Clone, Serialize)]
pub struct ObjectMapResolverEnvelope {
    pub schema_version: String,
    pub source_kind: String,
    pub source_size_bytes: u64,
    pub status: ObjectMapResolverStatus,
    pub resolver: Option<ObjectMapResolverReport>,
    pub errors: Vec<Diagnostic>,
    pub warnings: Vec<Diagnostic>,
    pub safety: SafetySummary,
}

#[derive(Debug, Clone, Serialize)]
pub struct VolumeReportEnvelope {
    pub schema_version: String,
    pub source_kind: String,
    pub source_size_bytes: u64,
    pub status: VolumeReportStatus,
    pub volume_count: usize,
    pub probes: Vec<VolumeProbeReport>,
    pub errors: Vec<Diagnostic>,
    pub warnings: Vec<Diagnostic>,
    pub safety: SafetySummary,
}

#[derive(Debug, Clone, Serialize)]
pub struct VolumeProbeReport {
    pub filesystem_oid: u64,
    pub lookup: Option<OmapLookup>,
    pub physical_block: Option<u64>,
    pub volume: Option<VolumeSuperblock>,
    pub errors: Vec<Diagnostic>,
    pub warnings: Vec<Diagnostic>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DirectoryReportEnvelope {
    pub schema_version: String,
    pub source_kind: String,
    pub source_size_bytes: u64,
    pub status: DirectoryReportStatus,
    pub volume_name: Option<String>,
    pub filesystem_oid: Option<u64>,
    pub root_tree_oid: Option<u64>,
    pub root_physical_block: Option<u64>,
    pub entry_count: usize,
    pub entries: Vec<FileSystemDirectoryRecord>,
    pub errors: Vec<Diagnostic>,
    pub warnings: Vec<Diagnostic>,
    pub safety: SafetySummary,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileReadReportEnvelope {
    pub schema_version: String,
    pub source_kind: String,
    pub source_size_bytes: u64,
    pub status: FileReadReportStatus,
    pub requested_name: String,
    pub matched_entry: Option<FileSystemDirectoryRecord>,
    pub content_preview_utf8: Option<String>,
    pub content_preview_hex: Option<String>,
    pub full_content_length: Option<usize>,
    pub errors: Vec<Diagnostic>,
    pub warnings: Vec<Diagnostic>,
    pub safety: SafetySummary,
}

#[derive(Debug, Clone, Serialize)]
pub struct ObjectMapResolverReport {
    pub mode: ObjectMapResolverMode,
    pub root_block_index: u64,
    pub root_level: u16,
    pub root_key_count: u32,
    pub bounded_depth_limit: u8,
    pub supports_synthetic_two_level_traversal: bool,
    pub supports_general_btree_traversal: bool,
    pub index_record_count: usize,
    pub root_record_count: usize,
    pub additional_leaf_node_count: usize,
    pub aggregate_record_count: usize,
    pub lookup_strategy: String,
    pub notes: Vec<Diagnostic>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResolvedObjectLookup {
    pub resolver: ObjectMapResolverReport,
    pub traversal: Option<SyntheticBTreeTraversalReport>,
    pub lookup: OmapLookup,
}

#[derive(Debug, Clone, Serialize)]
pub struct GptReport {
    pub assumed_sector_size: u32,
    pub header: GptHeader,
    pub entries_checksum: GptEntriesChecksum,
    pub partitions: Vec<GptPartitionEntry>,
    pub apfs_partition_index: Option<usize>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CheckpointScanReport {
    pub descriptor_base_block: u64,
    pub descriptor_len_blocks: u32,
    pub scanned_blocks: u32,
    pub candidates: Vec<CheckpointCandidate>,
    pub checkpoint_maps: Vec<CheckpointMapReport>,
    pub container_object_map: Option<MappedObjectMapReport>,
    pub latest_valid_xid: Option<u64>,
    pub latest_valid_block: Option<u64>,
    pub notes: Vec<Diagnostic>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CheckpointCandidate {
    pub block_index: u64,
    pub xid: u64,
    pub oid: u64,
    pub checksum: ObjectChecksum,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct CheckpointMapReport {
    pub block_index: u64,
    pub xid: u64,
    pub oid: u64,
    pub flags: u32,
    pub is_last: bool,
    pub count: u32,
    pub checksum: ObjectChecksum,
    pub valid: bool,
    pub mappings: Vec<CheckpointMapping>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MappedObjectMapReport {
    pub source_checkpoint_map_block: u64,
    pub mapping: CheckpointMapping,
    pub object_block_index: u64,
    pub object_map: ObjectMap,
    pub tree_root: Option<MappedBTreeReport>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MappedBTreeReport {
    pub source_checkpoint_map_block: u64,
    pub mapping: CheckpointMapping,
    pub object_block_index: u64,
    pub node: BTreeNode,
    pub preliminary_omap_records: Vec<OmapRecord>,
    pub index_records: Vec<BTreeIndexRecord>,
    pub traversal_path: Option<BTreeTraversalPathReport>,
    pub additional_mapped_leaf_nodes: Vec<MappedBTreeLeafReport>,
    pub aggregate_omap_record_count: usize,
    pub lookup_samples: Vec<OmapLookup>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MappedBTreeLeafReport {
    pub source_checkpoint_map_block: u64,
    pub mapping: CheckpointMapping,
    pub object_block_index: u64,
    pub node: BTreeNode,
    pub preliminary_omap_records: Vec<OmapRecord>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyntheticBTreeTraversalReport {
    pub requested_oid: u64,
    pub requested_xid: u64,
    pub root_block_index: u64,
    pub root_level: u16,
    pub index_records: Vec<BTreeIndexRecord>,
    pub child_selection: BTreeChildSelection,
    pub selected_leaf_block_index: Option<u64>,
    pub selected_leaf_oid: Option<u64>,
    pub selected_leaf_records: Vec<OmapRecord>,
    pub lookup: OmapLookup,
    pub notes: Vec<Diagnostic>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BTreeTraversalPathReport {
    pub root_index_records: Vec<BTreeIndexRecord>,
    pub bounded_depth_limit: u8,
    pub synthetic_two_level_supported: bool,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BTreeCursorStatus {
    Available,
    Unavailable,
    Refused,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BTreeCursorMode {
    SyntheticOmapTwoLevel,
    AggregateDecodedRecordsFallback,
    Unavailable,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BTreeCursorStepKind {
    Root,
    IndexSelection,
    Leaf,
    AggregateFallback,
}

#[derive(Debug, Clone, Serialize)]
pub struct BTreeCursorEnvelope {
    pub schema_version: String,
    pub source_kind: String,
    pub source_size_bytes: u64,
    pub status: BTreeCursorStatus,
    pub cursor: Option<BTreeCursorReport>,
    pub errors: Vec<Diagnostic>,
    pub warnings: Vec<Diagnostic>,
    pub safety: SafetySummary,
}

#[derive(Debug, Clone, Serialize)]
pub struct BTreeCursorReport {
    pub mode: BTreeCursorMode,
    pub key_kind: String,
    pub requested_oid: u64,
    pub requested_xid: u64,
    pub root_block_index: u64,
    pub root_oid: u64,
    pub root_level: u16,
    pub depth_limit: u8,
    pub steps: Vec<BTreeCursorStepReport>,
    pub lookup: OmapLookup,
    pub production_general_traversal_supported: bool,
    pub notes: Vec<Diagnostic>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BTreeCursorStepReport {
    pub depth: u8,
    pub step_kind: BTreeCursorStepKind,
    pub block_index: Option<u64>,
    pub node_oid: Option<u64>,
    pub level: Option<u16>,
    pub is_leaf: Option<bool>,
    pub key_count: Option<u32>,
    pub decoded_index_record_count: usize,
    pub decoded_omap_record_count: usize,
    pub selected_child_oid: Option<u64>,
    pub selected_child_block_index: Option<u64>,
}


#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MappedObjectReadStatus {
    Found,
    NotFound,
    Refused,
}

#[derive(Debug, Clone, Serialize)]
pub struct MappedObjectReadEnvelope {
    pub schema_version: String,
    pub source_kind: String,
    pub source_size_bytes: u64,
    pub status: MappedObjectReadStatus,
    pub requested_oid: u64,
    pub requested_xid: u64,
    pub lookup: Option<OmapLookup>,
    pub resolver: Option<ObjectMapResolverReport>,
    pub traversal: Option<SyntheticBTreeTraversalReport>,
    pub object: Option<MappedObjectReadReport>,
    pub errors: Vec<Diagnostic>,
    pub warnings: Vec<Diagnostic>,
    pub safety: SafetySummary,
}

#[derive(Debug, Clone, Serialize)]
pub struct MappedObjectReadReport {
    pub physical_block_index: u64,
    pub block_size: u32,
    pub bytes_read: usize,
    pub header: ObjectHeader,
    pub checksum: ObjectChecksum,
    pub checksum_valid: bool,
    pub object_type_name: String,
    pub object_preview_hex: String,
    pub payload_decoding_supported: bool,
    pub notes: Vec<Diagnostic>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Diagnostic {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SafetySummary {
    pub read_only: bool,
    pub physical_write_supported: bool,
    pub mount_supported: bool,
    pub extraction_supported: bool,
}

impl Default for SafetySummary {
    fn default() -> Self {
        Self {
            read_only: true,
            physical_write_supported: false,
            mount_supported: false,
            extraction_supported: false,
        }
    }
}

pub fn inspect_device(device: &dyn ReadOnlyBlockDevice) -> Result<InspectReport, InspectError> {
    let source_size_bytes = device.size()?;
    if source_size_bytes < NX_SUPERBLOCK_MIN_SIZE as u64 {
        return Ok(refused_report(
            source_size_bytes,
            "APFS-E-INPUT-TOO-SHORT",
            format!(
                "source is too short for an APFS container superblock probe: need at least {NX_SUPERBLOCK_MIN_SIZE} bytes"
            ),
        ));
    }

    let direct_probe_len = DEFAULT_APFS_PROBE_BYTES.min(usize::try_from(source_size_bytes).unwrap_or(DEFAULT_APFS_PROBE_BYTES));
    let direct_probe = device.read_at(0, direct_probe_len)?;
    match parse_nx_superblock(&direct_probe) {
        Ok(container_probe) => {
            return inspect_container_at_offset(
                device,
                source_size_bytes,
                0,
                SourceLayout::DirectContainerAtBlockZero,
                None,
                container_probe.block_size,
            );
        }
        Err(ParseError::MagicMismatch { .. }) => {
            // Continue with GPT probing below.
        }
        Err(err) => {
            return Ok(refused_report(source_size_bytes, "APFS-E-PARSE-REFUSED", err.to_string()));
        }
    }

    match inspect_gpt_wrapped_apfs(device, source_size_bytes)? {
        Some(report) => Ok(report),
        None => Ok(not_apfs_report(
            source_size_bytes,
            "block zero does not contain NXSB magic and no APFS GPT partition was found".to_owned(),
        )),
    }
}

pub fn inspect_bytes(input: &[u8]) -> InspectReport {
    let device = apfs_blockdev::MemoryBlockDevice::new(input.to_vec());
    match inspect_device(&device) {
        Ok(report) => report,
        Err(err) => refused_report(input.len() as u64, "APFS-E-INSPECT-ERROR", err.to_string()),
    }
}

pub fn lookup_object_in_device(
    device: &dyn ReadOnlyBlockDevice,
    requested_oid: u64,
    requested_xid: u64,
) -> Result<ObjectLookupReport, InspectError> {
    let report = inspect_device(device)?;
    Ok(lookup_object_in_report(&report, requested_oid, requested_xid))
}

pub fn lookup_object_in_bytes(input: &[u8], requested_oid: u64, requested_xid: u64) -> ObjectLookupReport {
    let device = apfs_blockdev::MemoryBlockDevice::new(input.to_vec());
    match lookup_object_in_device(&device, requested_oid, requested_xid) {
        Ok(report) => report,
        Err(err) => ObjectLookupReport {
            schema_version: "0.15.0".to_owned(),
            source_kind: "image".to_owned(),
            source_size_bytes: input.len() as u64,
            status: ObjectLookupStatus::Refused,
            requested_oid,
            requested_xid,
            lookup: None,
            traversal: None,
            resolver: None,
            errors: vec![Diagnostic { code: "APFS-E-LOOKUP-ERROR".to_owned(), message: err.to_string() }],
            warnings: Vec::new(),
            safety: SafetySummary::default(),
        },
    }
}

pub fn resolver_report_in_device(device: &dyn ReadOnlyBlockDevice) -> Result<ObjectMapResolverEnvelope, InspectError> {
    let report = inspect_device(device)?;
    Ok(resolver_report_in_report(&report))
}

pub fn resolver_report_in_bytes(input: &[u8]) -> ObjectMapResolverEnvelope {
    let device = apfs_blockdev::MemoryBlockDevice::new(input.to_vec());
    match resolver_report_in_device(&device) {
        Ok(report) => report,
        Err(err) => ObjectMapResolverEnvelope {
            schema_version: "0.15.0".to_owned(),
            source_kind: "image".to_owned(),
            source_size_bytes: input.len() as u64,
            status: ObjectMapResolverStatus::Refused,
            resolver: None,
            errors: vec![Diagnostic { code: "APFS-E-RESOLVER-ERROR".to_owned(), message: err.to_string() }],
            warnings: Vec::new(),
            safety: SafetySummary::default(),
        },
    }
}


pub fn volume_report_in_device(device: &dyn ReadOnlyBlockDevice) -> Result<VolumeReportEnvelope, InspectError> {
    let report = inspect_device(device)?;
    volume_report_in_report(device, &report)
}

pub fn volume_report_in_bytes(input: &[u8]) -> VolumeReportEnvelope {
    let device = apfs_blockdev::MemoryBlockDevice::new(input.to_vec());
    match volume_report_in_device(&device) {
        Ok(report) => report,
        Err(err) => VolumeReportEnvelope {
            schema_version: "0.15.0".to_owned(),
            source_kind: "image".to_owned(),
            source_size_bytes: input.len() as u64,
            status: VolumeReportStatus::Refused,
            volume_count: 0,
            probes: Vec::new(),
            errors: vec![Diagnostic { code: "APFS-E-VOLUME-REPORT-ERROR".to_owned(), message: err.to_string() }],
            warnings: Vec::new(),
            safety: SafetySummary::default(),
        },
    }
}

pub fn directory_report_in_device(device: &dyn ReadOnlyBlockDevice) -> Result<DirectoryReportEnvelope, InspectError> {
    let report = inspect_device(device)?;
    directory_report_in_report(device, &report)
}

pub fn directory_report_in_bytes(input: &[u8]) -> DirectoryReportEnvelope {
    let device = apfs_blockdev::MemoryBlockDevice::new(input.to_vec());
    match directory_report_in_device(&device) {
        Ok(report) => report,
        Err(err) => DirectoryReportEnvelope {
            schema_version: "0.15.0".to_owned(),
            source_kind: "image".to_owned(),
            source_size_bytes: input.len() as u64,
            status: DirectoryReportStatus::Refused,
            volume_name: None,
            filesystem_oid: None,
            root_tree_oid: None,
            root_physical_block: None,
            entry_count: 0,
            entries: Vec::new(),
            errors: vec![Diagnostic { code: "APFS-E-DIRECTORY-REPORT-ERROR".to_owned(), message: err.to_string() }],
            warnings: Vec::new(),
            safety: SafetySummary::default(),
        },
    }
}

pub fn file_read_report_in_device(device: &dyn ReadOnlyBlockDevice, requested_name: &str) -> Result<FileReadReportEnvelope, InspectError> {
    let report = inspect_device(device)?;
    file_read_report_in_report(device, &report, requested_name)
}

pub fn file_read_report_in_bytes(input: &[u8], requested_name: &str) -> FileReadReportEnvelope {
    let device = apfs_blockdev::MemoryBlockDevice::new(input.to_vec());
    match file_read_report_in_device(&device, requested_name) {
        Ok(report) => report,
        Err(err) => FileReadReportEnvelope {
            schema_version: "0.15.0".to_owned(),
            source_kind: "image".to_owned(),
            source_size_bytes: input.len() as u64,
            status: FileReadReportStatus::Refused,
            requested_name: requested_name.to_owned(),
            matched_entry: None,
            content_preview_utf8: None,
            content_preview_hex: None,
            full_content_length: None,
            errors: vec![Diagnostic { code: "APFS-E-FILE-READ-REPORT-ERROR".to_owned(), message: err.to_string() }],
            warnings: Vec::new(),
            safety: SafetySummary::default(),
        },
    }
}

fn volume_report_in_report(device: &dyn ReadOnlyBlockDevice, report: &InspectReport) -> Result<VolumeReportEnvelope, InspectError> {
    let mut errors = Vec::new();
    let mut warnings = report.warnings.clone();
    if report.status != InspectStatus::ApfsContainerDetected {
        errors.push(Diagnostic {
            code: "APFS-E-VOLUMES-INSPECT-NOT-DETECTED".to_owned(),
            message: "volume probing requires a successfully inspected APFS container".to_owned(),
        });
        return Ok(VolumeReportEnvelope {
            schema_version: "0.15.0".to_owned(),
            source_kind: report.source_kind.clone(),
            source_size_bytes: report.source_size_bytes,
            status: VolumeReportStatus::Refused,
            volume_count: 0,
            probes: Vec::new(),
            errors,
            warnings,
            safety: SafetySummary::default(),
        });
    }

    let Some(container) = &report.container else {
        errors.push(Diagnostic {
            code: "APFS-E-VOLUMES-CONTAINER-MISSING".to_owned(),
            message: "inspect report did not include a container superblock".to_owned(),
        });
        return Ok(VolumeReportEnvelope {
            schema_version: "0.15.0".to_owned(),
            source_kind: report.source_kind.clone(),
            source_size_bytes: report.source_size_bytes,
            status: VolumeReportStatus::Refused,
            volume_count: 0,
            probes: Vec::new(),
            errors,
            warnings,
            safety: SafetySummary::default(),
        });
    };

    let Some(tree_root) = report
        .checkpoint_scan
        .as_ref()
        .and_then(|scan| scan.container_object_map.as_ref())
        .and_then(|omap| omap.tree_root.as_ref())
    else {
        errors.push(Diagnostic {
            code: "APFS-E-VOLUMES-OMAP-RESOLVER-NOT-AVAILABLE".to_owned(),
            message: "container OMAP resolver is required before APFS volume superblocks can be mapped".to_owned(),
        });
        return Ok(VolumeReportEnvelope {
            schema_version: "0.15.0".to_owned(),
            source_kind: report.source_kind.clone(),
            source_size_bytes: report.source_size_bytes,
            status: VolumeReportStatus::Unavailable,
            volume_count: 0,
            probes: Vec::new(),
            errors,
            warnings,
            safety: SafetySummary::default(),
        });
    };

    let apfs_offset = report.apfs_offset_bytes.unwrap_or(0);
    let block_size = usize::try_from(container.block_size).map_err(|_| InspectError::BlockSizeTooLarge(container.block_size))?;
    let lookup_xid = report.checkpoint_scan.as_ref().and_then(|scan| scan.latest_valid_xid).unwrap_or(container.object.xid);
    let mut probes = Vec::new();

    for filesystem_oid in &container.filesystem_oids {
        let resolved = resolve_object_with_resolver(tree_root, *filesystem_oid, lookup_xid);
        let mut probe = VolumeProbeReport {
            filesystem_oid: *filesystem_oid,
            physical_block: resolved.lookup.physical_address,
            lookup: Some(resolved.lookup.clone()),
            volume: None,
            errors: Vec::new(),
            warnings: Vec::new(),
        };

        let Some(block_index) = resolved.lookup.physical_address else {
            probe.errors.push(Diagnostic {
                code: "APFS-E-VOLUME-SUPERBLOCK-NOT-MAPPED".to_owned(),
                message: format!("container filesystem OID {filesystem_oid} was not resolved by the current object-map resolver"),
            });
            probes.push(probe);
            continue;
        };

        let block = match read_container_block(device, apfs_offset, block_index, container.block_size, block_size) {
            Ok(block) => block,
            Err(err) => {
                probe.errors.push(Diagnostic {
                    code: "APFS-E-VOLUME-SUPERBLOCK-READ-FAILED".to_owned(),
                    message: err.to_string(),
                });
                probes.push(probe);
                continue;
            }
        };

        match parse_apfs_volume_superblock_with_checksum(&block) {
            Ok(volume) => {
                if !volume.checksum.valid {
                    probe.errors.push(Diagnostic {
                        code: "APFS-E-VOLUME-SUPERBLOCK-CHECKSUM-MISMATCH".to_owned(),
                        message: format!(
                            "volume superblock checksum mismatch for filesystem OID {filesystem_oid}: stored {}, computed {}",
                            volume.checksum.stored_checksum_hex, volume.checksum.computed_checksum_hex
                        ),
                    });
                }
                probe.volume = Some(volume);
            }
            Err(err) => {
                probe.errors.push(Diagnostic {
                    code: "APFS-E-VOLUME-SUPERBLOCK-PARSE-FAILED".to_owned(),
                    message: err.to_string(),
                });
            }
        }
        probes.push(probe);
    }

    if container.filesystem_oids.is_empty() {
        warnings.push(Diagnostic {
            code: "APFS-W-VOLUMES-NO-FILESYSTEM-OIDS".to_owned(),
            message: "container superblock did not expose filesystem OIDs in the parsed prefix".to_owned(),
        });
    }

    let volume_count = probes.iter().filter(|probe| probe.volume.as_ref().map(|volume| volume.checksum.valid).unwrap_or(false)).count();
    let status = if volume_count > 0 { VolumeReportStatus::Available } else { VolumeReportStatus::Unavailable };

    Ok(VolumeReportEnvelope {
        schema_version: "0.15.0".to_owned(),
        source_kind: report.source_kind.clone(),
        source_size_bytes: report.source_size_bytes,
        status,
        volume_count,
        probes,
        errors,
        warnings,
        safety: SafetySummary::default(),
    })
}

fn directory_report_in_report(device: &dyn ReadOnlyBlockDevice, report: &InspectReport) -> Result<DirectoryReportEnvelope, InspectError> {
    let mut errors = Vec::new();
    let mut warnings = report.warnings.clone();
    if report.status != InspectStatus::ApfsContainerDetected {
        errors.push(Diagnostic {
            code: "APFS-E-DIRECTORY-INSPECT-NOT-DETECTED".to_owned(),
            message: "directory probing requires a successfully inspected APFS container".to_owned(),
        });
        return Ok(directory_envelope(report, DirectoryReportStatus::Refused, None, None, None, None, Vec::new(), errors, warnings));
    }

    let Some(container) = &report.container else {
        errors.push(Diagnostic { code: "APFS-E-DIRECTORY-CONTAINER-MISSING".to_owned(), message: "inspect report did not include a container superblock".to_owned() });
        return Ok(directory_envelope(report, DirectoryReportStatus::Refused, None, None, None, None, Vec::new(), errors, warnings));
    };

    let Some(tree_root) = report
        .checkpoint_scan
        .as_ref()
        .and_then(|scan| scan.container_object_map.as_ref())
        .and_then(|omap| omap.tree_root.as_ref())
    else {
        errors.push(Diagnostic { code: "APFS-E-DIRECTORY-OMAP-RESOLVER-NOT-AVAILABLE".to_owned(), message: "container OMAP resolver is required before APFS filesystem root trees can be mapped".to_owned() });
        return Ok(directory_envelope(report, DirectoryReportStatus::Unavailable, None, None, None, None, Vec::new(), errors, warnings));
    };

    let volumes = volume_report_in_report(device, report)?;
    let Some(probe) = volumes.probes.iter().find(|probe| probe.volume.as_ref().map(|volume| volume.checksum.valid).unwrap_or(false)) else {
        errors.push(Diagnostic { code: "APFS-E-DIRECTORY-VOLUME-NOT-AVAILABLE".to_owned(), message: "no valid APFS volume superblock was available for directory probing".to_owned() });
        return Ok(directory_envelope(report, DirectoryReportStatus::Unavailable, None, None, None, None, Vec::new(), errors, warnings));
    };
    let Some(volume) = probe.volume.as_ref() else {
        errors.push(Diagnostic { code: "APFS-E-DIRECTORY-VOLUME-MISSING".to_owned(), message: "volume probe did not include a parsed volume".to_owned() });
        return Ok(directory_envelope(report, DirectoryReportStatus::Unavailable, None, None, None, None, Vec::new(), errors, warnings));
    };

    let lookup_xid = report.checkpoint_scan.as_ref().and_then(|scan| scan.latest_valid_xid).unwrap_or(container.object.xid);
    let resolved = resolve_object_with_resolver(tree_root, volume.root_tree_oid, lookup_xid);
    let Some(root_block_index) = resolved.lookup.physical_address else {
        errors.push(Diagnostic { code: "APFS-E-DIRECTORY-ROOT-TREE-NOT-MAPPED".to_owned(), message: format!("volume root tree OID {} was not resolved by the current object-map resolver", volume.root_tree_oid) });
        return Ok(directory_envelope(report, DirectoryReportStatus::Unavailable, Some(volume.volume_name.clone()), Some(probe.filesystem_oid), Some(volume.root_tree_oid), None, Vec::new(), errors, warnings));
    };

    let apfs_offset = report.apfs_offset_bytes.unwrap_or(0);
    let block_size = usize::try_from(container.block_size).map_err(|_| InspectError::BlockSizeTooLarge(container.block_size))?;
    let block = read_container_block(device, apfs_offset, root_block_index, container.block_size, block_size)?;
    let node = match parse_btree_node_with_checksum(&block) {
        Ok(node) => node,
        Err(err) => {
            errors.push(Diagnostic { code: "APFS-E-DIRECTORY-ROOT-TREE-PARSE-FAILED".to_owned(), message: err.to_string() });
            return Ok(directory_envelope(report, DirectoryReportStatus::Refused, Some(volume.volume_name.clone()), Some(probe.filesystem_oid), Some(volume.root_tree_oid), Some(root_block_index), Vec::new(), errors, warnings));
        }
    };
    if !node.checksum.valid {
        errors.push(Diagnostic { code: "APFS-E-DIRECTORY-ROOT-TREE-CHECKSUM-MISMATCH".to_owned(), message: "synthetic filesystem root tree checksum mismatch".to_owned() });
        return Ok(directory_envelope(report, DirectoryReportStatus::Refused, Some(volume.volume_name.clone()), Some(probe.filesystem_oid), Some(volume.root_tree_oid), Some(root_block_index), Vec::new(), errors, warnings));
    }

    let entries = match parse_synthetic_directory_records_from_btree_node(&block, &node) {
        Ok(entries) => entries,
        Err(err) => {
            errors.push(Diagnostic { code: "APFS-E-DIRECTORY-RECORD-PARSE-FAILED".to_owned(), message: err.to_string() });
            return Ok(directory_envelope(report, DirectoryReportStatus::Refused, Some(volume.volume_name.clone()), Some(probe.filesystem_oid), Some(volume.root_tree_oid), Some(root_block_index), Vec::new(), errors, warnings));
        }
    };
    warnings.push(Diagnostic { code: "APFS-W-DIRECTORY-SYNTHETIC".to_owned(), message: "directory listing currently parses synthetic filesystem tree records; production APFS filesystem record decoding is not implemented yet".to_owned() });
    Ok(directory_envelope(report, DirectoryReportStatus::Available, Some(volume.volume_name.clone()), Some(probe.filesystem_oid), Some(volume.root_tree_oid), Some(root_block_index), entries, errors, warnings))
}

fn directory_envelope(
    report: &InspectReport,
    status: DirectoryReportStatus,
    volume_name: Option<String>,
    filesystem_oid: Option<u64>,
    root_tree_oid: Option<u64>,
    root_physical_block: Option<u64>,
    entries: Vec<FileSystemDirectoryRecord>,
    errors: Vec<Diagnostic>,
    warnings: Vec<Diagnostic>,
) -> DirectoryReportEnvelope {
    DirectoryReportEnvelope {
        schema_version: "0.15.0".to_owned(),
        source_kind: report.source_kind.clone(),
        source_size_bytes: report.source_size_bytes,
        status,
        volume_name,
        filesystem_oid,
        root_tree_oid,
        root_physical_block,
        entry_count: entries.len(),
        entries,
        errors,
        warnings,
        safety: SafetySummary::default(),
    }
}

fn file_read_report_in_report(device: &dyn ReadOnlyBlockDevice, report: &InspectReport, requested_name: &str) -> Result<FileReadReportEnvelope, InspectError> {
    let mut warnings = report.warnings.clone();
    let mut errors = Vec::new();
    let directory = directory_report_in_report(device, report)?;
    if directory.status != DirectoryReportStatus::Available {
        errors.extend(directory.errors.clone());
        errors.push(Diagnostic { code: "APFS-E-FILE-DIRECTORY-NOT-AVAILABLE".to_owned(), message: "file preview requires an available synthetic directory report".to_owned() });
        return Ok(file_envelope(report, FileReadReportStatus::Refused, requested_name, None, None, None, None, errors, warnings));
    }

    let Some(entry) = directory.entries.iter().find(|entry| entry.name == requested_name).cloned() else {
        return Ok(file_envelope(report, FileReadReportStatus::NotFound, requested_name, None, None, None, None, errors, warnings));
    };
    let Some(block_index) = entry.physical_block else {
        errors.push(Diagnostic { code: "APFS-E-FILE-NO-PHYSICAL-BLOCK".to_owned(), message: format!("directory entry {requested_name} did not include a physical block in the synthetic fixture") });
        return Ok(file_envelope(report, FileReadReportStatus::Refused, requested_name, Some(entry), None, None, None, errors, warnings));
    };
    let Some(container) = &report.container else {
        errors.push(Diagnostic { code: "APFS-E-FILE-CONTAINER-MISSING".to_owned(), message: "inspect report did not include a container superblock".to_owned() });
        return Ok(file_envelope(report, FileReadReportStatus::Refused, requested_name, Some(entry), None, None, None, errors, warnings));
    };
    let block_size = usize::try_from(container.block_size).map_err(|_| InspectError::BlockSizeTooLarge(container.block_size))?;
    let apfs_offset = report.apfs_offset_bytes.unwrap_or(0);
    let block = read_container_block(device, apfs_offset, block_index, container.block_size, block_size)?;
    let preview_len = usize::try_from(entry.logical_size).unwrap_or(block.len()).min(block.len()).min(512);
    let preview = &block[..preview_len];
    let utf8 = String::from_utf8_lossy(preview).into_owned();
    let hex = preview.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    warnings.push(Diagnostic { code: "APFS-W-FILE-PREVIEW-SYNTHETIC".to_owned(), message: "file read currently returns a bounded preview from a synthetic direct block pointer, not production APFS extent resolution".to_owned() });
    Ok(file_envelope(report, FileReadReportStatus::Available, requested_name, Some(entry), Some(utf8), Some(hex), Some(preview_len), errors, warnings))
}

fn file_envelope(
    report: &InspectReport,
    status: FileReadReportStatus,
    requested_name: &str,
    matched_entry: Option<FileSystemDirectoryRecord>,
    content_preview_utf8: Option<String>,
    content_preview_hex: Option<String>,
    full_content_length: Option<usize>,
    errors: Vec<Diagnostic>,
    warnings: Vec<Diagnostic>,
) -> FileReadReportEnvelope {
    FileReadReportEnvelope {
        schema_version: "0.15.0".to_owned(),
        source_kind: report.source_kind.clone(),
        source_size_bytes: report.source_size_bytes,
        status,
        requested_name: requested_name.to_owned(),
        matched_entry,
        content_preview_utf8,
        content_preview_hex,
        full_content_length,
        errors,
        warnings,
        safety: SafetySummary::default(),
    }
}

pub fn btree_cursor_report_in_device(
    device: &dyn ReadOnlyBlockDevice,
    requested_oid: u64,
    requested_xid: u64,
) -> Result<BTreeCursorEnvelope, InspectError> {
    let report = inspect_device(device)?;
    Ok(btree_cursor_report_in_report(&report, requested_oid, requested_xid))
}

pub fn btree_cursor_report_in_bytes(input: &[u8], requested_oid: u64, requested_xid: u64) -> BTreeCursorEnvelope {
    let device = apfs_blockdev::MemoryBlockDevice::new(input.to_vec());
    match btree_cursor_report_in_device(&device, requested_oid, requested_xid) {
        Ok(report) => report,
        Err(err) => BTreeCursorEnvelope {
            schema_version: "0.15.0".to_owned(),
            source_kind: "image".to_owned(),
            source_size_bytes: input.len() as u64,
            status: BTreeCursorStatus::Refused,
            cursor: None,
            errors: vec![Diagnostic { code: "APFS-E-BTREE-CURSOR-ERROR".to_owned(), message: err.to_string() }],
            warnings: Vec::new(),
            safety: SafetySummary::default(),
        },
    }
}


pub fn read_mapped_object_in_device(
    device: &dyn ReadOnlyBlockDevice,
    requested_oid: u64,
    requested_xid: u64,
) -> Result<MappedObjectReadEnvelope, InspectError> {
    let report = inspect_device(device)?;
    read_mapped_object_from_report_and_device(device, &report, requested_oid, requested_xid)
}

pub fn read_mapped_object_in_bytes(input: &[u8], requested_oid: u64, requested_xid: u64) -> MappedObjectReadEnvelope {
    let device = apfs_blockdev::MemoryBlockDevice::new(input.to_vec());
    match read_mapped_object_in_device(&device, requested_oid, requested_xid) {
        Ok(report) => report,
        Err(err) => MappedObjectReadEnvelope {
            schema_version: "0.15.0".to_owned(),
            source_kind: "image".to_owned(),
            source_size_bytes: input.len() as u64,
            status: MappedObjectReadStatus::Refused,
            requested_oid,
            requested_xid,
            lookup: None,
            resolver: None,
            traversal: None,
            object: None,
            errors: vec![Diagnostic { code: "APFS-E-READ-MAPPED-OBJECT-ERROR".to_owned(), message: err.to_string() }],
            warnings: Vec::new(),
            safety: SafetySummary::default(),
        },
    }
}

pub fn btree_cursor_report_in_report(report: &InspectReport, requested_oid: u64, requested_xid: u64) -> BTreeCursorEnvelope {
    let mut errors = Vec::new();
    let mut warnings = report.warnings.clone();

    if report.status != InspectStatus::ApfsContainerDetected {
        errors.push(Diagnostic {
            code: "APFS-E-BTREE-CURSOR-INSPECT-NOT-DETECTED".to_owned(),
            message: "B-tree cursor requires a successfully inspected APFS container".to_owned(),
        });
        return BTreeCursorEnvelope {
            schema_version: "0.15.0".to_owned(),
            source_kind: report.source_kind.clone(),
            source_size_bytes: report.source_size_bytes,
            status: BTreeCursorStatus::Refused,
            cursor: None,
            errors,
            warnings,
            safety: SafetySummary::default(),
        };
    }

    let Some(tree_root) = report
        .checkpoint_scan
        .as_ref()
        .and_then(|scan| scan.container_object_map.as_ref())
        .and_then(|omap| omap.tree_root.as_ref())
    else {
        errors.push(Diagnostic {
            code: "APFS-E-BTREE-CURSOR-ROOT-NOT-AVAILABLE".to_owned(),
            message: "container OMAP B-tree root is not available in this inspection slice".to_owned(),
        });
        return BTreeCursorEnvelope {
            schema_version: "0.15.0".to_owned(),
            source_kind: report.source_kind.clone(),
            source_size_bytes: report.source_size_bytes,
            status: BTreeCursorStatus::Unavailable,
            cursor: None,
            errors,
            warnings,
            safety: SafetySummary::default(),
        };
    };

    warnings.push(Diagnostic {
        code: "APFS-W-BTREE-CURSOR-SYNTHETIC".to_owned(),
        message: "B-tree cursor currently exposes a production-shaped API over synthetic OMAP fixtures; general APFS B-tree traversal is not implemented yet".to_owned(),
    });
    let cursor = build_btree_cursor_report(tree_root, requested_oid, requested_xid);
    BTreeCursorEnvelope {
        schema_version: "0.15.0".to_owned(),
        source_kind: report.source_kind.clone(),
        source_size_bytes: report.source_size_bytes,
        status: BTreeCursorStatus::Available,
        cursor: Some(cursor),
        errors,
        warnings,
        safety: SafetySummary::default(),
    }
}

pub fn resolver_report_in_report(report: &InspectReport) -> ObjectMapResolverEnvelope {
    let mut errors = Vec::new();
    let warnings = report.warnings.clone();

    if report.status != InspectStatus::ApfsContainerDetected {
        errors.push(Diagnostic {
            code: "APFS-E-RESOLVER-INSPECT-NOT-DETECTED".to_owned(),
            message: "object-map resolver requires a successfully inspected APFS container".to_owned(),
        });
        return ObjectMapResolverEnvelope {
            schema_version: "0.15.0".to_owned(),
            source_kind: report.source_kind.clone(),
            source_size_bytes: report.source_size_bytes,
            status: ObjectMapResolverStatus::Refused,
            resolver: None,
            errors,
            warnings,
            safety: SafetySummary::default(),
        };
    }

    let resolver = report
        .checkpoint_scan
        .as_ref()
        .and_then(|scan| scan.container_object_map.as_ref())
        .and_then(|omap| omap.tree_root.as_ref())
        .map(object_map_resolver_report);

    let status = if resolver.is_some() { ObjectMapResolverStatus::Available } else { ObjectMapResolverStatus::Unavailable };
    if resolver.is_none() {
        errors.push(Diagnostic {
            code: "APFS-E-RESOLVER-OMAP-TREE-NOT-AVAILABLE".to_owned(),
            message: "container OMAP B-tree root is not available in this inspection slice".to_owned(),
        });
    }

    ObjectMapResolverEnvelope {
        schema_version: "0.15.0".to_owned(),
        source_kind: report.source_kind.clone(),
        source_size_bytes: report.source_size_bytes,
        status,
        resolver,
        errors,
        warnings,
        safety: SafetySummary::default(),
    }
}

fn lookup_object_in_report(report: &InspectReport, requested_oid: u64, requested_xid: u64) -> ObjectLookupReport {
    let mut errors = Vec::new();
    let mut warnings = report.warnings.clone();

    if report.status != InspectStatus::ApfsContainerDetected {
        errors.push(Diagnostic {
            code: "APFS-E-LOOKUP-INSPECT-NOT-DETECTED".to_owned(),
            message: "object lookup requires a successfully inspected APFS container".to_owned(),
        });
        return ObjectLookupReport {
            schema_version: "0.15.0".to_owned(),
            source_kind: report.source_kind.clone(),
            source_size_bytes: report.source_size_bytes,
            status: ObjectLookupStatus::Refused,
            requested_oid,
            requested_xid,
            lookup: None,
            traversal: None,
            resolver: None,
            errors,
            warnings,
            safety: SafetySummary::default(),
        };
    }

    let Some(tree_root) = report
        .checkpoint_scan
        .as_ref()
        .and_then(|scan| scan.container_object_map.as_ref())
        .and_then(|omap| omap.tree_root.as_ref())
    else {
        errors.push(Diagnostic {
            code: "APFS-E-OMAP-TREE-NOT-AVAILABLE".to_owned(),
            message: "the container object map B-tree root is not available in this inspection slice".to_owned(),
        });
        return ObjectLookupReport {
            schema_version: "0.15.0".to_owned(),
            source_kind: report.source_kind.clone(),
            source_size_bytes: report.source_size_bytes,
            status: ObjectLookupStatus::Refused,
            requested_oid,
            requested_xid,
            lookup: None,
            traversal: None,
            resolver: None,
            errors,
            warnings,
            safety: SafetySummary::default(),
        };
    };

    warnings.push(Diagnostic {
        code: "APFS-W-LOOKUP-BOUNDED-SYNTHETIC-BTREE".to_owned(),
        message: "object lookup currently uses a bounded synthetic two-level OMAP B-tree traversal when root index records are available; full APFS B-tree traversal is not implemented yet".to_owned(),
    });

    let resolved = resolve_object_with_resolver(tree_root, requested_oid, requested_xid);
    let status = if resolved.lookup.matched { ObjectLookupStatus::Found } else { ObjectLookupStatus::NotFound };
    if !resolved.lookup.matched {
        errors.push(Diagnostic {
            code: "APFS-E-OMAP-LOOKUP-NOT-FOUND".to_owned(),
            message: format!("no decoded OMAP record matched oid {requested_oid} at or before xid {requested_xid}"),
        });
    }

    ObjectLookupReport {
        schema_version: "0.15.0".to_owned(),
        source_kind: report.source_kind.clone(),
        source_size_bytes: report.source_size_bytes,
        status,
        requested_oid,
        requested_xid,
        lookup: Some(resolved.lookup),
        traversal: resolved.traversal,
        resolver: Some(resolved.resolver),
        errors,
        warnings,
        safety: SafetySummary::default(),
    }
}

fn inspect_gpt_wrapped_apfs(
    device: &dyn ReadOnlyBlockDevice,
    source_size_bytes: u64,
) -> Result<Option<InspectReport>, InspectError> {
    if source_size_bytes < (GPT_SECTOR_SIZE * 2) as u64 {
        return Ok(None);
    }

    let header_sector = device.read_at(GPT_SECTOR_SIZE as u64, GPT_SECTOR_SIZE)?;
    let header = match parse_gpt_header(&header_sector) {
        Ok(header) => header,
        Err(ParseError::GptSignatureMismatch) => return Ok(None),
        Err(err) => return Ok(Some(refused_report(source_size_bytes, "APFS-E-GPT-PARSE-REFUSED", err.to_string()))),
    };

    let entries_len = gpt_entries_byte_len(&header)?;
    if entries_len > MAX_GPT_ENTRIES_BYTES {
        return Err(InspectError::GptEntriesTooLarge(entries_len));
    }
    let entries_offset = lba_to_offset(header.partition_entry_lba)?;
    let entries_bytes = device.read_at(entries_offset, entries_len)?;
    let entries_checksum = validate_gpt_entries_checksum(&entries_bytes, &header);

    let mut partitions = Vec::new();
    let mut apfs_partition_index = None;
    let entry_size = usize::try_from(header.size_of_partition_entry).map_err(|_| InspectError::ArithmeticOverflow)?;
    for index in 0..usize::try_from(header.number_of_partition_entries).map_err(|_| InspectError::ArithmeticOverflow)? {
        let start = index.checked_mul(entry_size).ok_or(InspectError::ArithmeticOverflow)?;
        let end = start.checked_add(entry_size).ok_or(InspectError::ArithmeticOverflow)?;
        if end > entries_bytes.len() {
            break;
        }
        if let Some(entry) = parse_gpt_partition_entry(&entries_bytes[start..end])? {
            if entry.is_apfs && apfs_partition_index.is_none() {
                apfs_partition_index = Some(partitions.len());
            }
            partitions.push(entry);
        }
    }

    let gpt_report = GptReport {
        assumed_sector_size: GPT_SECTOR_SIZE as u32,
        header,
        entries_checksum,
        partitions,
        apfs_partition_index,
    };

    let Some(apfs_index) = gpt_report.apfs_partition_index else {
        return Ok(None);
    };
    let apfs_entry = &gpt_report.partitions[apfs_index];
    let apfs_offset = lba_to_offset(apfs_entry.first_lba)?;
    let probe_len = DEFAULT_APFS_PROBE_BYTES.min(usize::try_from(source_size_bytes.saturating_sub(apfs_offset)).unwrap_or(0));
    if probe_len < NX_SUPERBLOCK_MIN_SIZE {
        return Ok(Some(refused_report(
            source_size_bytes,
            "APFS-E-APFS-PARTITION-TOO-SHORT",
            "APFS GPT partition is too short for an NX superblock probe".to_owned(),
        )));
    }
    let apfs_probe = device.read_at(apfs_offset, probe_len)?;
    match parse_nx_superblock(&apfs_probe) {
        Ok(container_probe) => Ok(Some(inspect_container_at_offset(
            device,
            source_size_bytes,
            apfs_offset,
            SourceLayout::GptWithApfsPartition,
            Some(gpt_report),
            container_probe.block_size,
        )?)),
        Err(err) => Ok(Some(refused_report(
            source_size_bytes,
            "APFS-E-GPT-APFS-PARTITION-PARSE-REFUSED",
            err.to_string(),
        ))),
    }
}

fn inspect_container_at_offset(
    device: &dyn ReadOnlyBlockDevice,
    source_size_bytes: u64,
    apfs_offset: u64,
    layout: SourceLayout,
    gpt: Option<GptReport>,
    block_size: u32,
) -> Result<InspectReport, InspectError> {
    let block_size_usize = usize::try_from(block_size).map_err(|_| InspectError::BlockSizeTooLarge(block_size))?;
    let block = device.read_at(apfs_offset, block_size_usize)?;
    let container = parse_nx_superblock_with_checksum(&block)?;
    let checksum = container.checksum.clone();
    let Some(checksum) = checksum else {
        return Ok(refused_report(source_size_bytes, "APFS-E-CHECKSUM-NOT-COMPUTED", "checksum was not computed".to_owned()));
    };
    if !checksum.valid {
        return Ok(refused_report(
            source_size_bytes,
            "APFS-E-CHECKSUM-MISMATCH",
            format!(
                "container superblock checksum mismatch: stored {}, computed {}",
                checksum.stored_checksum_hex, checksum.computed_checksum_hex
            ),
        ));
    }

    let mut warnings = Vec::new();
    if matches!(layout, SourceLayout::GptWithApfsPartition) {
        warnings.push(Diagnostic {
            code: "APFS-W-GPT-SECTOR-SIZE-ASSUMED".to_owned(),
            message: "GPT probing currently assumes 512-byte sectors".to_owned(),
        });
    }
    if let Some(gpt_report) = &gpt {
        if !gpt_report.header.header_crc32_valid {
            warnings.push(Diagnostic {
                code: "APFS-W-GPT-HEADER-CRC-MISMATCH".to_owned(),
                message: "primary GPT header CRC32 did not match; APFS probe continued because the APFS partition entry was parseable".to_owned(),
            });
        }
        if !gpt_report.entries_checksum.valid {
            warnings.push(Diagnostic {
                code: "APFS-W-GPT-ENTRIES-CRC-MISMATCH".to_owned(),
                message: "GPT partition-entry array CRC32 did not match; APFS probe continued because an APFS entry was parseable".to_owned(),
            });
        }
    }

    let checkpoint_scan = scan_checkpoint_area(device, apfs_offset, &container)?;
    Ok(detected_report(source_size_bytes, layout, Some(apfs_offset), gpt, container, checkpoint_scan, warnings))
}

fn scan_checkpoint_area(
    device: &dyn ReadOnlyBlockDevice,
    apfs_offset: u64,
    container: &ContainerSuperblock,
) -> Result<Option<CheckpointScanReport>, InspectError> {
    let descriptor_len = container.checkpoint_descriptor_len;
    let descriptor_base = container.checkpoint_descriptor_base;
    if descriptor_base == 0 || descriptor_len == 0 {
        return Ok(Some(CheckpointScanReport {
            descriptor_base_block: descriptor_base,
            descriptor_len_blocks: descriptor_len,
            scanned_blocks: 0,
            candidates: Vec::new(),
            checkpoint_maps: Vec::new(),
            container_object_map: None,
            latest_valid_xid: None,
            latest_valid_block: None,
            notes: vec![Diagnostic {
                code: "APFS-I-CHECKPOINT-SCAN-SKIPPED".to_owned(),
                message: "container superblock does not describe a checkpoint descriptor area in this fixture/input".to_owned(),
            }],
        }));
    }

    let block_size = usize::try_from(container.block_size).map_err(|_| InspectError::BlockSizeTooLarge(container.block_size))?;
    let blocks_to_scan = descriptor_len.min(MAX_CHECKPOINT_SCAN_BLOCKS);
    let mut candidates = Vec::new();
    let mut checkpoint_maps = Vec::new();
    let mut notes = Vec::new();

    if descriptor_len > MAX_CHECKPOINT_SCAN_BLOCKS {
        notes.push(Diagnostic {
            code: "APFS-W-CHECKPOINT-SCAN-LIMITED".to_owned(),
            message: format!("checkpoint descriptor scan limited to {MAX_CHECKPOINT_SCAN_BLOCKS} blocks"),
        });
    }

    for relative in 0..blocks_to_scan {
        let block_index = descriptor_base.checked_add(u64::from(relative)).ok_or(InspectError::ArithmeticOverflow)?;
        let block = match read_container_block(device, apfs_offset, block_index, container.block_size, block_size) {
            Ok(block) => block,
            Err(InspectError::BlockDevice(BlockDeviceError::OutOfBounds { .. })) => break,
            Err(err) => return Err(err),
        };

        if let Ok(candidate) = parse_nx_superblock_with_checksum(&block) {
            if let Some(checksum) = candidate.checksum.clone() {
                let valid = checksum.valid;
                candidates.push(CheckpointCandidate {
                    block_index,
                    xid: candidate.object.xid,
                    oid: candidate.object.oid,
                    checksum,
                    valid,
                });
            }
        }

        if let Ok(map_block) = parse_checkpoint_map_block_with_checksum(&block) {
            let valid = map_block.checksum.valid;
            checkpoint_maps.push(CheckpointMapReport {
                block_index,
                xid: map_block.object.xid,
                oid: map_block.object.oid,
                flags: map_block.flags,
                is_last: map_block.is_last,
                count: map_block.count,
                checksum: map_block.checksum,
                valid,
                mappings: map_block.mappings,
            });
        }
    }

    let latest_valid = candidates
        .iter()
        .filter(|candidate| candidate.valid)
        .max_by_key(|candidate| candidate.xid)
        .map(|candidate| (candidate.xid, candidate.block_index));
    let (latest_valid_xid, latest_valid_block) = latest_valid
        .map(|(xid, block)| (Some(xid), Some(block)))
        .unwrap_or((None, None));

    let container_object_map = resolve_container_omap_from_checkpoint_maps(device, apfs_offset, container, block_size, &checkpoint_maps, &mut notes)?;

    Ok(Some(CheckpointScanReport {
        descriptor_base_block: descriptor_base,
        descriptor_len_blocks: descriptor_len,
        scanned_blocks: blocks_to_scan,
        candidates,
        checkpoint_maps,
        container_object_map,
        latest_valid_xid,
        latest_valid_block,
        notes,
    }))
}

fn resolve_container_omap_from_checkpoint_maps(
    device: &dyn ReadOnlyBlockDevice,
    apfs_offset: u64,
    container: &ContainerSuperblock,
    block_size: usize,
    checkpoint_maps: &[CheckpointMapReport],
    notes: &mut Vec<Diagnostic>,
) -> Result<Option<MappedObjectMapReport>, InspectError> {
    if container.omap_oid == 0 {
        return Ok(None);
    }

    let mut selected: Option<(&CheckpointMapReport, &CheckpointMapping)> = None;
    for map in checkpoint_maps.iter().filter(|map| map.valid) {
        for mapping in &map.mappings {
            let exact_oid = mapping.oid == container.omap_oid;
            let omap_type = mapping.object_type == OBJECT_TYPE_OMAP;
            if exact_oid || (selected.is_none() && omap_type) {
                selected = Some((map, mapping));
                if exact_oid {
                    break;
                }
            }
        }
    }

    let Some((map, mapping)) = selected else {
        notes.push(Diagnostic {
            code: "APFS-I-CONTAINER-OMAP-NOT-MAPPED".to_owned(),
            message: format!("no valid checkpoint-map entry currently maps nx_omap_oid {}", container.omap_oid),
        });
        return Ok(None);
    };

    let object_block_index = mapping.physical_address;
    let object_block = read_container_block(device, apfs_offset, object_block_index, container.block_size, block_size)?;
    let object_map = match parse_omap_phys_with_checksum(&object_block) {
        Ok(object_map) => object_map,
        Err(err) => {
            notes.push(Diagnostic {
                code: "APFS-W-CONTAINER-OMAP-PARSE-FAILED".to_owned(),
                message: format!("checkpoint map pointed to block {object_block_index}, but omap parsing failed: {err}"),
            });
            return Ok(None);
        }
    };

    let tree_root = resolve_omap_tree_root(
        device,
        apfs_offset,
        container,
        block_size,
        checkpoint_maps,
        &object_map,
        notes,
    )?;

    Ok(Some(MappedObjectMapReport {
        source_checkpoint_map_block: map.block_index,
        mapping: mapping.clone(),
        object_block_index,
        object_map,
        tree_root,
    }))
}

fn resolve_omap_tree_root(
    device: &dyn ReadOnlyBlockDevice,
    apfs_offset: u64,
    container: &ContainerSuperblock,
    block_size: usize,
    checkpoint_maps: &[CheckpointMapReport],
    object_map: &ObjectMap,
    notes: &mut Vec<Diagnostic>,
) -> Result<Option<MappedBTreeReport>, InspectError> {
    if object_map.tree_oid == 0 {
        notes.push(Diagnostic {
            code: "APFS-I-OMAP-TREE-OID-ZERO".to_owned(),
            message: "container object map does not name an OMAP B-tree root".to_owned(),
        });
        return Ok(None);
    }

    let mut selected: Option<(&CheckpointMapReport, &CheckpointMapping)> = None;
    for map in checkpoint_maps.iter().filter(|map| map.valid) {
        for mapping in &map.mappings {
            let exact_oid = mapping.oid == object_map.tree_oid;
            let btree_type = mapping.object_type == OBJECT_TYPE_BTREE || mapping.object_type == OBJECT_TYPE_BTREE_NODE;
            if exact_oid || (selected.is_none() && btree_type) {
                selected = Some((map, mapping));
                if exact_oid {
                    break;
                }
            }
        }
    }

    let Some((map, mapping)) = selected else {
        notes.push(Diagnostic {
            code: "APFS-I-OMAP-BTREE-ROOT-NOT-MAPPED".to_owned(),
            message: format!("no valid checkpoint-map entry currently maps om_tree_oid {}", object_map.tree_oid),
        });
        return Ok(None);
    };

    let object_block_index = mapping.physical_address;
    let object_block = read_container_block(device, apfs_offset, object_block_index, container.block_size, block_size)?;
    match parse_btree_node_with_checksum(&object_block) {
        Ok(node) => {
            let preliminary_omap_records = match parse_omap_records_from_btree_node(&object_block, &node) {
                Ok(records) => records,
                Err(err) => {
                    notes.push(Diagnostic {
                        code: "APFS-W-OMAP-BTREE-RECORD-PARSE-FAILED".to_owned(),
                        message: format!("B-tree root was parsed, but preliminary OMAP record decoding failed: {err}"),
                    });
                    Vec::new()
                }
            };
            let index_records = match parse_omap_index_records_from_btree_node(&object_block, &node) {
                Ok(records) => records,
                Err(err) => {
                    notes.push(Diagnostic {
                        code: "APFS-W-OMAP-BTREE-INDEX-PARSE-FAILED".to_owned(),
                        message: format!("B-tree root was parsed, but synthetic index decoding failed: {err}"),
                    });
                    Vec::new()
                }
            };
            let traversal_path = if index_records.is_empty() {
                None
            } else {
                Some(BTreeTraversalPathReport {
                    root_index_records: index_records.clone(),
                    bounded_depth_limit: 2,
                    synthetic_two_level_supported: true,
                })
            };
            let additional_mapped_leaf_nodes = decode_additional_omap_leaf_nodes(
                device,
                apfs_offset,
                container,
                block_size,
                checkpoint_maps,
                object_map.tree_oid,
                notes,
            )?;
            let mut aggregate_records = preliminary_omap_records.clone();
            for leaf in &additional_mapped_leaf_nodes {
                aggregate_records.extend(leaf.preliminary_omap_records.iter().cloned());
            }
            let lookup_samples = aggregate_records
                .iter()
                .take(16)
                .map(|record| lookup_omap_record(&aggregate_records, record.oid, record.xid))
                .collect();
            Ok(Some(MappedBTreeReport {
                source_checkpoint_map_block: map.block_index,
                mapping: mapping.clone(),
                object_block_index,
                node,
                preliminary_omap_records,
                index_records,
                traversal_path,
                additional_mapped_leaf_nodes,
                aggregate_omap_record_count: aggregate_records.len(),
                lookup_samples,
            }))
        }
        Err(err) => {
            notes.push(Diagnostic {
                code: "APFS-W-OMAP-BTREE-ROOT-PARSE-FAILED".to_owned(),
                message: format!("checkpoint map pointed to OMAP tree block {object_block_index}, but B-tree parsing failed: {err}"),
            });
            Ok(None)
        }
    }
}

fn decode_additional_omap_leaf_nodes(
    device: &dyn ReadOnlyBlockDevice,
    apfs_offset: u64,
    container: &ContainerSuperblock,
    block_size: usize,
    checkpoint_maps: &[CheckpointMapReport],
    root_tree_oid: u64,
    notes: &mut Vec<Diagnostic>,
) -> Result<Vec<MappedBTreeLeafReport>, InspectError> {
    let mut leaves = Vec::new();
    for map in checkpoint_maps.iter().filter(|map| map.valid) {
        for mapping in &map.mappings {
            let btree_type = mapping.object_type == OBJECT_TYPE_BTREE || mapping.object_type == OBJECT_TYPE_BTREE_NODE;
            if !btree_type || mapping.oid == root_tree_oid {
                continue;
            }
            let object_block_index = mapping.physical_address;
            let object_block = match read_container_block(device, apfs_offset, object_block_index, container.block_size, block_size) {
                Ok(block) => block,
                Err(err) => {
                    notes.push(Diagnostic {
                        code: "APFS-W-OMAP-BTREE-LEAF-READ-FAILED".to_owned(),
                        message: format!("checkpoint map pointed to possible OMAP leaf block {object_block_index}, but reading failed: {err}"),
                    });
                    continue;
                }
            };
            let node = match parse_btree_node_with_checksum(&object_block) {
                Ok(node) => node,
                Err(err) => {
                    notes.push(Diagnostic {
                        code: "APFS-W-OMAP-BTREE-LEAF-PARSE-FAILED".to_owned(),
                        message: format!("checkpoint map pointed to possible OMAP leaf block {object_block_index}, but B-tree parsing failed: {err}"),
                    });
                    continue;
                }
            };
            if !node.is_leaf {
                notes.push(Diagnostic {
                    code: "APFS-I-OMAP-BTREE-NONLEAF-SKIPPED".to_owned(),
                    message: format!("mapped B-tree block {object_block_index} is not a leaf; general traversal is not implemented yet"),
                });
                continue;
            }
            let preliminary_omap_records = match parse_omap_records_from_btree_node(&object_block, &node) {
                Ok(records) => records,
                Err(err) => {
                    notes.push(Diagnostic {
                        code: "APFS-W-OMAP-BTREE-LEAF-RECORD-PARSE-FAILED".to_owned(),
                        message: format!("mapped OMAP leaf block {object_block_index} parsed as a B-tree node, but OMAP record decoding failed: {err}"),
                    });
                    continue;
                }
            };
            leaves.push(MappedBTreeLeafReport {
                source_checkpoint_map_block: map.block_index,
                mapping: mapping.clone(),
                object_block_index,
                node,
                preliminary_omap_records,
            });
        }
    }
    Ok(leaves)
}


fn read_mapped_object_from_report_and_device(
    device: &dyn ReadOnlyBlockDevice,
    report: &InspectReport,
    requested_oid: u64,
    requested_xid: u64,
) -> Result<MappedObjectReadEnvelope, InspectError> {
    let mut errors = Vec::new();
    let mut warnings = report.warnings.clone();

    if report.status != InspectStatus::ApfsContainerDetected {
        errors.push(Diagnostic {
            code: "APFS-E-READ-MAPPED-OBJECT-INSPECT-NOT-DETECTED".to_owned(),
            message: "mapped object read requires a successfully inspected APFS container".to_owned(),
        });
        return Ok(mapped_object_envelope(report, MappedObjectReadStatus::Refused, requested_oid, requested_xid, None, None, None, None, errors, warnings));
    }

    let Some(container) = report.container.as_ref() else {
        errors.push(Diagnostic {
            code: "APFS-E-READ-MAPPED-OBJECT-CONTAINER-MISSING".to_owned(),
            message: "inspect report did not include a container superblock".to_owned(),
        });
        return Ok(mapped_object_envelope(report, MappedObjectReadStatus::Refused, requested_oid, requested_xid, None, None, None, None, errors, warnings));
    };

    let Some(tree_root) = report
        .checkpoint_scan
        .as_ref()
        .and_then(|scan| scan.container_object_map.as_ref())
        .and_then(|omap| omap.tree_root.as_ref())
    else {
        errors.push(Diagnostic {
            code: "APFS-E-READ-MAPPED-OBJECT-RESOLVER-UNAVAILABLE".to_owned(),
            message: "container object-map resolver is not available in this inspection slice".to_owned(),
        });
        return Ok(mapped_object_envelope(report, MappedObjectReadStatus::Refused, requested_oid, requested_xid, None, None, None, None, errors, warnings));
    };

    warnings.push(Diagnostic {
        code: "APFS-W-READ-MAPPED-OBJECT-SYNTHETIC".to_owned(),
        message: "mapped-object read currently uses the synthetic object-map resolver facade; real APFS object-map traversal still requires a macOS fixture feedback pass".to_owned(),
    });

    let resolved = resolve_object_with_resolver(tree_root, requested_oid, requested_xid);
    if !resolved.lookup.matched {
        return Ok(mapped_object_envelope(
            report,
            MappedObjectReadStatus::NotFound,
            requested_oid,
            requested_xid,
            Some(resolved.lookup),
            Some(resolved.resolver),
            resolved.traversal,
            None,
            errors,
            warnings,
        ));
    }

    let Some(physical_block_index) = resolved.lookup.physical_address else {
        errors.push(Diagnostic {
            code: "APFS-E-READ-MAPPED-OBJECT-PADDR-MISSING".to_owned(),
            message: "object-map lookup matched but did not provide a physical block address".to_owned(),
        });
        return Ok(mapped_object_envelope(report, MappedObjectReadStatus::Refused, requested_oid, requested_xid, Some(resolved.lookup), Some(resolved.resolver), resolved.traversal, None, errors, warnings));
    };

    let block_size = usize::try_from(container.block_size).map_err(|_| InspectError::BlockSizeTooLarge(container.block_size))?;
    let apfs_offset = report.apfs_offset_bytes.unwrap_or(0);
    let block = match read_container_block(device, apfs_offset, physical_block_index, container.block_size, block_size) {
        Ok(block) => block,
        Err(err) => {
            errors.push(Diagnostic {
                code: "APFS-E-READ-MAPPED-OBJECT-BLOCK-READ-FAILED".to_owned(),
                message: err.to_string(),
            });
            return Ok(mapped_object_envelope(report, MappedObjectReadStatus::Refused, requested_oid, requested_xid, Some(resolved.lookup), Some(resolved.resolver), resolved.traversal, None, errors, warnings));
        }
    };

    let header = match parse_object_header(&block) {
        Ok(header) => header,
        Err(err) => {
            errors.push(Diagnostic {
                code: "APFS-E-READ-MAPPED-OBJECT-HEADER-PARSE-FAILED".to_owned(),
                message: err.to_string(),
            });
            return Ok(mapped_object_envelope(report, MappedObjectReadStatus::Refused, requested_oid, requested_xid, Some(resolved.lookup), Some(resolved.resolver), resolved.traversal, None, errors, warnings));
        }
    };
    let checksum = match validate_object_checksum(&block) {
        Ok(checksum) => checksum,
        Err(err) => {
            errors.push(Diagnostic {
                code: "APFS-E-READ-MAPPED-OBJECT-CHECKSUM-PARSE-FAILED".to_owned(),
                message: err.to_string(),
            });
            return Ok(mapped_object_envelope(report, MappedObjectReadStatus::Refused, requested_oid, requested_xid, Some(resolved.lookup), Some(resolved.resolver), resolved.traversal, None, errors, warnings));
        }
    };
    if !checksum.valid {
        errors.push(Diagnostic {
            code: "APFS-E-READ-MAPPED-OBJECT-CHECKSUM-MISMATCH".to_owned(),
            message: format!(
                "mapped object checksum mismatch at block {physical_block_index}: stored {}, computed {}",
                checksum.stored_checksum_hex, checksum.computed_checksum_hex
            ),
        });
        return Ok(mapped_object_envelope(report, MappedObjectReadStatus::Refused, requested_oid, requested_xid, Some(resolved.lookup), Some(resolved.resolver), resolved.traversal, None, errors, warnings));
    }

    let object = MappedObjectReadReport {
        physical_block_index,
        block_size: container.block_size,
        bytes_read: block.len(),
        header: header.clone(),
        checksum: checksum.clone(),
        checksum_valid: checksum.valid,
        object_type_name: object_type_name(header.object_type).to_owned(),
        object_preview_hex: hex_prefix(&block, 64),
        payload_decoding_supported: false,
        notes: vec![Diagnostic {
            code: "APFS-I-MAPPED-OBJECT-PAYLOAD-NOT-DECODED".to_owned(),
            message: "mapped object payload decoding is intentionally not implemented yet; this slice only proves resolver-backed object block reads and header/checksum validation".to_owned(),
        }],
    };

    Ok(mapped_object_envelope(
        report,
        MappedObjectReadStatus::Found,
        requested_oid,
        requested_xid,
        Some(resolved.lookup),
        Some(resolved.resolver),
        resolved.traversal,
        Some(object),
        errors,
        warnings,
    ))
}

#[allow(clippy::too_many_arguments)]
fn mapped_object_envelope(
    report: &InspectReport,
    status: MappedObjectReadStatus,
    requested_oid: u64,
    requested_xid: u64,
    lookup: Option<OmapLookup>,
    resolver: Option<ObjectMapResolverReport>,
    traversal: Option<SyntheticBTreeTraversalReport>,
    object: Option<MappedObjectReadReport>,
    errors: Vec<Diagnostic>,
    warnings: Vec<Diagnostic>,
) -> MappedObjectReadEnvelope {
    MappedObjectReadEnvelope {
        schema_version: "0.15.0".to_owned(),
        source_kind: report.source_kind.clone(),
        source_size_bytes: report.source_size_bytes,
        status,
        requested_oid,
        requested_xid,
        lookup,
        resolver,
        traversal,
        object,
        errors,
        warnings,
        safety: SafetySummary::default(),
    }
}

fn object_type_name(object_type: u16) -> &'static str {
    match object_type {
        OBJECT_TYPE_NX_SUPERBLOCK => "nx_superblock",
        OBJECT_TYPE_BTREE => "btree",
        OBJECT_TYPE_BTREE_NODE => "btree_node",
        OBJECT_TYPE_OMAP => "object_map",
        OBJECT_TYPE_CHECKPOINT_MAP => "checkpoint_map",
        _ => "unknown_or_not_yet_named",
    }
}

fn hex_prefix(bytes: &[u8], max_len: usize) -> String {
    let mut out = String::new();
    for byte in bytes.iter().take(max_len) {
        use core::fmt::Write as _;
        let _ = write!(&mut out, "{byte:02x}");
    }
    out
}

fn object_map_resolver_report(tree_root: &MappedBTreeReport) -> ObjectMapResolverReport {
    let aggregate_record_count = aggregate_omap_records(tree_root).len();
    let supports_synthetic_two_level_traversal = !tree_root.index_records.is_empty() && !tree_root.additional_mapped_leaf_nodes.is_empty();
    let mode = if supports_synthetic_two_level_traversal {
        ObjectMapResolverMode::BoundedSyntheticTwoLevelTraversal
    } else if aggregate_record_count > 0 {
        ObjectMapResolverMode::AggregateDecodedRecords
    } else {
        ObjectMapResolverMode::Unavailable
    };
    let lookup_strategy = match mode {
        ObjectMapResolverMode::BoundedSyntheticTwoLevelTraversal => "bounded_synthetic_two_level_traversal_then_leaf_lookup",
        ObjectMapResolverMode::AggregateDecodedRecords => "aggregate_decoded_record_lookup",
        ObjectMapResolverMode::Unavailable => "unavailable",
    }
    .to_owned();

    let mut notes = Vec::new();
    if supports_synthetic_two_level_traversal {
        notes.push(Diagnostic {
            code: "APFS-I-RESOLVER-BOUNDED-SYNTHETIC-TWO-LEVEL".to_owned(),
            message: "resolver can use the synthetic two-level B-tree traversal path for fixtures with root index records and mapped leaf nodes".to_owned(),
        });
    } else if aggregate_record_count > 0 {
        notes.push(Diagnostic {
            code: "APFS-W-RESOLVER-AGGREGATE-FALLBACK".to_owned(),
            message: "resolver is falling back to aggregate decoded records because no bounded traversal path is available".to_owned(),
        });
    } else {
        notes.push(Diagnostic {
            code: "APFS-E-RESOLVER-NO-RECORDS".to_owned(),
            message: "resolver has no decoded OMAP records available".to_owned(),
        });
    }

    ObjectMapResolverReport {
        mode,
        root_block_index: tree_root.object_block_index,
        root_level: tree_root.node.level,
        root_key_count: tree_root.node.key_count,
        bounded_depth_limit: 2,
        supports_synthetic_two_level_traversal,
        supports_general_btree_traversal: false,
        index_record_count: tree_root.index_records.len(),
        root_record_count: tree_root.preliminary_omap_records.len(),
        additional_leaf_node_count: tree_root.additional_mapped_leaf_nodes.len(),
        aggregate_record_count,
        lookup_strategy,
        notes,
    }
}

fn build_btree_cursor_report(tree_root: &MappedBTreeReport, requested_oid: u64, requested_xid: u64) -> BTreeCursorReport {
    let supports_two_level = !tree_root.index_records.is_empty() && !tree_root.additional_mapped_leaf_nodes.is_empty();
    let traversal = if supports_two_level {
        traverse_synthetic_omap_btree(tree_root, requested_oid, requested_xid)
    } else {
        None
    };

    let mode = if supports_two_level {
        BTreeCursorMode::SyntheticOmapTwoLevel
    } else if tree_root.aggregate_omap_record_count > 0 {
        BTreeCursorMode::AggregateDecodedRecordsFallback
    } else {
        BTreeCursorMode::Unavailable
    };

    let mut steps = Vec::new();
    steps.push(BTreeCursorStepReport {
        depth: 0,
        step_kind: BTreeCursorStepKind::Root,
        block_index: Some(tree_root.object_block_index),
        node_oid: Some(tree_root.node.object.oid),
        level: Some(tree_root.node.level),
        is_leaf: Some(tree_root.node.is_leaf),
        key_count: Some(tree_root.node.key_count),
        decoded_index_record_count: tree_root.index_records.len(),
        decoded_omap_record_count: tree_root.preliminary_omap_records.len(),
        selected_child_oid: traversal.as_ref().and_then(|t| t.child_selection.selected_child_oid),
        selected_child_block_index: traversal.as_ref().and_then(|t| t.selected_leaf_block_index),
    });

    if let Some(traversal) = &traversal {
        steps.push(BTreeCursorStepReport {
            depth: 1,
            step_kind: BTreeCursorStepKind::IndexSelection,
            block_index: Some(tree_root.object_block_index),
            node_oid: Some(tree_root.node.object.oid),
            level: Some(tree_root.node.level),
            is_leaf: Some(tree_root.node.is_leaf),
            key_count: Some(tree_root.node.key_count),
            decoded_index_record_count: traversal.index_records.len(),
            decoded_omap_record_count: 0,
            selected_child_oid: traversal.child_selection.selected_child_oid,
            selected_child_block_index: traversal.selected_leaf_block_index,
        });
        steps.push(BTreeCursorStepReport {
            depth: 2,
            step_kind: BTreeCursorStepKind::Leaf,
            block_index: traversal.selected_leaf_block_index,
            node_oid: traversal.selected_leaf_oid,
            level: Some(0),
            is_leaf: Some(true),
            key_count: Some(u32::try_from(traversal.selected_leaf_records.len()).unwrap_or(u32::MAX)),
            decoded_index_record_count: 0,
            decoded_omap_record_count: traversal.selected_leaf_records.len(),
            selected_child_oid: None,
            selected_child_block_index: None,
        });
    } else {
        steps.push(BTreeCursorStepReport {
            depth: 1,
            step_kind: BTreeCursorStepKind::AggregateFallback,
            block_index: None,
            node_oid: None,
            level: None,
            is_leaf: None,
            key_count: None,
            decoded_index_record_count: 0,
            decoded_omap_record_count: aggregate_omap_records(tree_root).len(),
            selected_child_oid: None,
            selected_child_block_index: None,
        });
    }

    let lookup = traversal
        .as_ref()
        .map(|traversal| traversal.lookup.clone())
        .unwrap_or_else(|| lookup_omap_record(&aggregate_omap_records(tree_root), requested_oid, requested_xid));

    let mut notes = vec![Diagnostic {
        code: "APFS-I-BTREE-CURSOR-BOUNDARY".to_owned(),
        message: "cursor API boundary is intentionally production-shaped, but this implementation remains limited to synthetic OMAP records and bounded two-level traversal".to_owned(),
    }];
    if !supports_two_level {
        notes.push(Diagnostic {
            code: "APFS-W-BTREE-CURSOR-FALLBACK".to_owned(),
            message: "cursor could not use two-level traversal and fell back to aggregate decoded records".to_owned(),
        });
    }

    BTreeCursorReport {
        mode,
        key_kind: "omap_key_t".to_owned(),
        requested_oid,
        requested_xid,
        root_block_index: tree_root.object_block_index,
        root_oid: tree_root.node.object.oid,
        root_level: tree_root.node.level,
        depth_limit: 2,
        steps,
        lookup,
        production_general_traversal_supported: false,
        notes,
    }
}

fn resolve_object_with_resolver(tree_root: &MappedBTreeReport, requested_oid: u64, requested_xid: u64) -> ResolvedObjectLookup {
    let resolver = object_map_resolver_report(tree_root);
    let traversal = if resolver.supports_synthetic_two_level_traversal {
        traverse_synthetic_omap_btree(tree_root, requested_oid, requested_xid)
    } else {
        None
    };
    let lookup = traversal
        .as_ref()
        .map(|traversal| traversal.lookup.clone())
        .unwrap_or_else(|| {
            let records = aggregate_omap_records(tree_root);
            lookup_omap_record(&records, requested_oid, requested_xid)
        });

    ResolvedObjectLookup { resolver, traversal, lookup }
}

fn traverse_synthetic_omap_btree(
    tree_root: &MappedBTreeReport,
    requested_oid: u64,
    requested_xid: u64,
) -> Option<SyntheticBTreeTraversalReport> {
    if tree_root.index_records.is_empty() {
        return None;
    }

    let child_selection = select_synthetic_btree_child(&tree_root.index_records, requested_oid, requested_xid);
    let selected_child_oid = child_selection.selected_child_oid?;
    let selected_leaf = tree_root
        .additional_mapped_leaf_nodes
        .iter()
        .find(|leaf| leaf.mapping.oid == selected_child_oid || leaf.node.object.oid == selected_child_oid);

    let mut notes = Vec::new();
    let selected_leaf_records = if let Some(leaf) = selected_leaf {
        leaf.preliminary_omap_records.clone()
    } else {
        notes.push(Diagnostic {
            code: "APFS-W-SYNTHETIC-BTREE-CHILD-NOT-MAPPED".to_owned(),
            message: format!("synthetic B-tree index selected child oid {selected_child_oid}, but no mapped leaf node was decoded"),
        });
        Vec::new()
    };

    let lookup = lookup_omap_record(&selected_leaf_records, requested_oid, requested_xid);
    Some(SyntheticBTreeTraversalReport {
        requested_oid,
        requested_xid,
        root_block_index: tree_root.object_block_index,
        root_level: tree_root.node.level,
        index_records: tree_root.index_records.clone(),
        child_selection,
        selected_leaf_block_index: selected_leaf.map(|leaf| leaf.object_block_index),
        selected_leaf_oid: selected_leaf.map(|leaf| leaf.node.object.oid),
        selected_leaf_records,
        lookup,
        notes,
    })
}

fn aggregate_omap_records(tree_root: &MappedBTreeReport) -> Vec<OmapRecord> {
    let mut records = tree_root.preliminary_omap_records.clone();
    for leaf in &tree_root.additional_mapped_leaf_nodes {
        records.extend(leaf.preliminary_omap_records.iter().cloned());
    }
    records
}

fn read_container_block(
    device: &dyn ReadOnlyBlockDevice,
    apfs_offset: u64,
    block_index: u64,
    block_size_u32: u32,
    block_size: usize,
) -> Result<Vec<u8>, InspectError> {
    let byte_offset = block_index.checked_mul(u64::from(block_size_u32)).ok_or(InspectError::ArithmeticOverflow)?;
    let absolute_offset = apfs_offset.checked_add(byte_offset).ok_or(InspectError::ArithmeticOverflow)?;
    device.read_at(absolute_offset, block_size).map_err(InspectError::from)
}

fn lba_to_offset(lba: u64) -> Result<u64, InspectError> {
    lba.checked_mul(GPT_SECTOR_SIZE as u64).ok_or(InspectError::ArithmeticOverflow)
}

fn detected_report(
    source_size_bytes: u64,
    layout: SourceLayout,
    apfs_offset_bytes: Option<u64>,
    gpt: Option<GptReport>,
    container: ContainerSuperblock,
    checkpoint_scan: Option<CheckpointScanReport>,
    warnings: Vec<Diagnostic>,
) -> InspectReport {
    InspectReport {
        schema_version: "0.15.0".to_owned(),
        source_kind: "image".to_owned(),
        source_size_bytes,
        status: InspectStatus::ApfsContainerDetected,
        layout,
        apfs_offset_bytes,
        gpt,
        container: Some(container),
        checkpoint_scan,
        errors: Vec::new(),
        warnings,
        safety: SafetySummary::default(),
    }
}

fn not_apfs_report(source_size_bytes: u64, message: String) -> InspectReport {
    InspectReport {
        schema_version: "0.15.0".to_owned(),
        source_kind: "image".to_owned(),
        source_size_bytes,
        status: InspectStatus::NotApfs,
        layout: SourceLayout::Unknown,
        apfs_offset_bytes: None,
        gpt: None,
        container: None,
        checkpoint_scan: None,
        errors: vec![Diagnostic { code: "APFS-E-NOT-APFS".to_owned(), message }],
        warnings: Vec::new(),
        safety: SafetySummary::default(),
    }
}

fn refused_report(source_size_bytes: u64, code: &str, message: String) -> InspectReport {
    InspectReport {
        schema_version: "0.15.0".to_owned(),
        source_kind: "image".to_owned(),
        source_size_bytes,
        status: InspectStatus::Refused,
        layout: SourceLayout::Unknown,
        apfs_offset_bytes: None,
        gpt: None,
        container: None,
        checkpoint_scan: None,
        errors: vec![Diagnostic { code: code.to_owned(), message }],
        warnings: Vec::new(),
        safety: SafetySummary::default(),
    }
}

#[cfg(test)]
mod tests {
    use apfs_types::{apfs_fletcher64, OBJ_EPHEMERAL, OBJECT_TYPE_NX_SUPERBLOCK};

    use super::{inspect_bytes, InspectStatus, SourceLayout};

    fn sign_block(block: &mut [u8]) {
        block[0..8].fill(0);
        let checksum = apfs_fletcher64(block).unwrap();
        block[0..8].copy_from_slice(&checksum.to_le_bytes());
    }

    fn minimal_nxsb(xid: u64) -> Vec<u8> {
        let mut block = vec![0_u8; 4096];
        block[8..16].copy_from_slice(&1_u64.to_le_bytes());
        block[16..24].copy_from_slice(&xid.to_le_bytes());
        block[24..28].copy_from_slice(&(OBJ_EPHEMERAL | u32::from(OBJECT_TYPE_NX_SUPERBLOCK)).to_le_bytes());
        block[32..36].copy_from_slice(b"NXSB");
        block[36..40].copy_from_slice(&4096_u32.to_le_bytes());
        block[40..48].copy_from_slice(&16_u64.to_le_bytes());
        block[112..120].copy_from_slice(&2_u64.to_le_bytes());
        block[120..128].copy_from_slice(&10_u64.to_le_bytes());
        block[140..144].copy_from_slice(&4_u32.to_le_bytes());
        block[148..152].copy_from_slice(&4_u32.to_le_bytes());
        block[160..168].copy_from_slice(&12_u64.to_le_bytes());
        block[180..184].copy_from_slice(&1_u32.to_le_bytes());
        block[184..192].copy_from_slice(&42_u64.to_le_bytes());
        sign_block(&mut block);
        block
    }

    #[test]
    fn detects_apfs_container() {
        let report = inspect_bytes(&minimal_nxsb(10));
        assert_eq!(report.status, InspectStatus::ApfsContainerDetected);
        assert_eq!(report.layout, SourceLayout::DirectContainerAtBlockZero);
        assert_eq!(report.container.unwrap().block_size, 4096);
    }

    #[test]
    fn reports_not_apfs() {
        let report = inspect_bytes(&vec![0_u8; 4096]);
        assert_eq!(report.status, InspectStatus::NotApfs);
        assert_eq!(report.errors[0].code, "APFS-E-NOT-APFS");
    }

    #[test]
    fn refuses_bad_checksum() {
        let mut block = minimal_nxsb(10);
        block[88] ^= 0xff;
        let report = inspect_bytes(&block);
        assert_eq!(report.status, InspectStatus::Refused);
        assert_eq!(report.errors[0].code, "APFS-E-CHECKSUM-MISMATCH");
    }

    #[test]
    fn lookup_refuses_when_tree_is_missing() {
        let block = minimal_nxsb(10);
        let lookup = super::lookup_object_in_bytes(&block, 500, 10);
        assert_eq!(lookup.status, super::ObjectLookupStatus::Refused);
        assert_eq!(lookup.errors[0].code, "APFS-E-OMAP-TREE-NOT-AVAILABLE");
    }
}
