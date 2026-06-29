#![forbid(unsafe_code)]

use serde::Serialize;
use thiserror::Error;

pub const APFS_OBJECT_HEADER_SIZE: usize = 32;
pub const NX_SUPERBLOCK_MIN_SIZE: usize = 184;
pub const NX_MAGIC_BYTES: [u8; 4] = *b"NXSB";
pub const GPT_SECTOR_SIZE: usize = 512;
pub const GPT_HEADER_MIN_SIZE: usize = 92;
pub const GPT_PARTITION_ENTRY_MIN_SIZE: usize = 128;
pub const GPT_HEADER_SIGNATURE: [u8; 8] = *b"EFI PART";
pub const OBJECT_TYPE_NX_SUPERBLOCK: u16 = 0x0001;
pub const OBJECT_TYPE_BTREE: u16 = 0x0002;
pub const OBJECT_TYPE_BTREE_NODE: u16 = 0x0003;
pub const OBJECT_TYPE_OMAP: u16 = 0x000b;
pub const OBJECT_TYPE_CHECKPOINT_MAP: u16 = 0x000c;
pub const OBJECT_TYPE_FS: u16 = 0x000d;
pub const OBJ_EPHEMERAL: u32 = 0x8000_0000;
pub const OBJ_PHYSICAL: u32 = 0x4000_0000;
pub const CHECKPOINT_MAP_LAST: u32 = 0x0000_0001;
pub const CHECKPOINT_MAP_HEADER_SIZE: usize = 40;
pub const CHECKPOINT_MAPPING_SIZE: usize = 40;
pub const OMAP_PHYS_MIN_SIZE: usize = 88;
pub const APFS_VOLUME_MAGIC_BYTES: [u8; 4] = *b"APSB";
pub const APFS_VOLUME_SUPERBLOCK_MIN_SIZE: usize = 770;
pub const APFS_VOLUME_NAME_OFFSET: usize = 512;
pub const APFS_VOLUME_NAME_LEN: usize = 256;
pub const APFS_VOLUME_ROLE_OFFSET: usize = 768;
pub const BTREE_NODE_MIN_SIZE: usize = 56;
pub const BTREE_TOC_ENTRY_SIZE: usize = 4;
pub const BTREE_NODE_ROOT: u16 = 0x0001;
pub const BTREE_NODE_LEAF: u16 = 0x0002;
pub const BTREE_NODE_FIXED_KV_SIZE: u16 = 0x0004;
pub const OMAP_KEY_SIZE: usize = 16;
pub const OMAP_VALUE_SIZE: usize = 16;
pub const SYNTHETIC_DIR_KEY_SIZE: usize = 16;
pub const SYNTHETIC_DIR_VALUE_HEADER_SIZE: usize = 24;
pub const SYNTHETIC_EXTENT_KEY_SIZE: usize = 16;
pub const SYNTHETIC_EXTENT_VALUE_SIZE: usize = 16;
pub const SYNTHETIC_FILE_KIND_FILE: u16 = 1;
pub const SYNTHETIC_FILE_KIND_DIRECTORY: u16 = 2;
pub const SYNTHETIC_FILE_KIND_SYMLINK: u16 = 3;
pub const SYNTHETIC_FS_DIR_KEY_SIZE: usize = 16;
pub const SYNTHETIC_FS_DIR_VALUE_HEADER_SIZE: usize = 28;
pub const FLETCHER64_MODULUS: u64 = 0xffff_ffff;

/// APFS GPT partition type GUID, encoded exactly as it appears in GPT entries on disk.
/// Canonical GUID: 7c3457ef-0000-11aa-aa11-00306543ecac.
pub const APFS_GPT_TYPE_GUID_BYTES: [u8; 16] = [
    0xef, 0x57, 0x34, 0x7c, 0x00, 0x00, 0xaa, 0x11, 0xaa, 0x11, 0x00, 0x30, 0x65, 0x43, 0xec, 0xac,
];

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseError {
    #[error("input is too short for {structure}: need at least {needed} bytes, got {actual}")]
    TooShort {
        structure: &'static str,
        needed: usize,
        actual: usize,
    },
    #[error("APFS NX superblock magic mismatch at offset 32: expected NXSB, got {found:?}")]
    MagicMismatch { found: [u8; 4] },
    #[error("APFS volume superblock magic mismatch at offset 32: expected APSB, got {found:?}")]
    VolumeMagicMismatch { found: [u8; 4] },
    #[error("invalid APFS container block size {0}")]
    InvalidBlockSize(u32),
    #[error("object checksum input length must be at least 12 bytes and 4-byte aligned after checksum field; got {0}")]
    InvalidChecksumInputLength(usize),
    #[error("object type mismatch: expected low type 0x{expected:04x}, got 0x{actual:04x}")]
    ObjectTypeMismatch { expected: u16, actual: u16 },
    #[error("checkpoint map count {count} exceeds maximum entries {max} for the supplied block")]
    InvalidCheckpointMapCount { count: u32, max: usize },
    #[error("GPT header signature mismatch")]
    GptSignatureMismatch,
    #[error("invalid GPT header size {0}")]
    InvalidGptHeaderSize(u32),
    #[error("invalid GPT partition entry size {0}")]
    InvalidGptPartitionEntrySize(u32),
    #[error("B-tree node table of contents is out of bounds: offset={offset}, length={length}, block_len={block_len}")]
    BTreeTableOutOfBounds {
        offset: usize,
        length: usize,
        block_len: usize,
    },
    #[error("GPT arithmetic overflow")]
    GptArithmeticOverflow,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ObjectChecksum {
    pub stored_checksum: u64,
    pub stored_checksum_hex: String,
    pub computed_checksum: u64,
    pub computed_checksum_hex: String,
    pub valid: bool,
    pub checked_bytes: usize,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ObjectHeader {
    pub checksum_hex: String,
    pub checksum_u64: u64,
    pub oid: u64,
    pub xid: u64,
    pub object_type_raw: u32,
    pub object_type: u16,
    pub object_flags: u16,
    pub object_subtype_raw: u32,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ContainerSuperblock {
    pub object: ObjectHeader,
    pub checksum: Option<ObjectChecksum>,
    pub magic: String,
    pub block_size: u32,
    pub block_count: u64,
    pub container_size_bytes: Option<u128>,
    pub features: u64,
    pub readonly_compatible_features: u64,
    pub incompatible_features: u64,
    pub uuid: String,
    pub next_oid: u64,
    pub next_xid: u64,
    pub checkpoint_descriptor_blocks_raw: u32,
    pub checkpoint_data_blocks_raw: u32,
    pub checkpoint_descriptor_base: u64,
    pub checkpoint_data_base: u64,
    pub checkpoint_descriptor_next: u32,
    pub checkpoint_data_next: u32,
    pub checkpoint_descriptor_index: u32,
    pub checkpoint_descriptor_len: u32,
    pub checkpoint_data_index: u32,
    pub checkpoint_data_len: u32,
    pub spaceman_oid: u64,
    pub omap_oid: u64,
    pub reaper_oid: u64,
    pub test_type: u32,
    pub max_file_systems: u32,
    pub filesystem_oids: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CheckpointMapping {
    pub object_type_raw: u32,
    pub object_type: u16,
    pub object_flags: u16,
    pub object_subtype_raw: u32,
    pub size_bytes: u32,
    pub pad: u32,
    pub filesystem_oid: u64,
    pub oid: u64,
    pub physical_address: u64,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CheckpointMapBlock {
    pub object: ObjectHeader,
    pub checksum: ObjectChecksum,
    pub flags: u32,
    pub count: u32,
    pub is_last: bool,
    pub mappings: Vec<CheckpointMapping>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ObjectMap {
    pub object: ObjectHeader,
    pub checksum: ObjectChecksum,
    pub flags: u32,
    pub snapshot_count: u32,
    pub tree_type_raw: u32,
    pub snapshot_tree_type_raw: u32,
    pub tree_oid: u64,
    pub snapshot_tree_oid: u64,
    pub most_recent_snapshot_xid: u64,
    pub pending_revert_min_xid: u64,
    pub pending_revert_max_xid: u64,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct VolumeSuperblock {
    pub object: ObjectHeader,
    pub checksum: ObjectChecksum,
    pub magic: String,
    pub fs_index: u32,
    pub features: u64,
    pub readonly_compatible_features: u64,
    pub incompatible_features: u64,
    pub unmount_time: u64,
    pub fs_reserve_block_count: u64,
    pub fs_quota_block_count: u64,
    pub fs_alloc_count: u64,
    pub root_tree_oid: u64,
    pub extentref_tree_oid: u64,
    pub snap_meta_tree_oid: u64,
    pub next_obj_id: u64,
    pub num_files: u64,
    pub num_directories: u64,
    pub num_symlinks: u64,
    pub num_other_fsobjects: u64,
    pub num_snapshots: u64,
    pub total_blocks_alloced: u64,
    pub total_blocks_freed: u64,
    pub volume_uuid: String,
    pub last_mod_time: u64,
    pub fs_flags: u64,
    pub volume_name: String,
    pub role: u16,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct BTreeTableSpace {
    pub offset: u16,
    pub length: u16,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct BTreeTocEntry {
    pub key_offset: u16,
    pub value_offset: u16,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct BTreeNode {
    pub object: ObjectHeader,
    pub checksum: ObjectChecksum,
    pub flags_raw: u16,
    pub level: u16,
    pub key_count: u32,
    pub table_space: BTreeTableSpace,
    pub free_space: BTreeTableSpace,
    pub key_free_list: BTreeTableSpace,
    pub value_free_list: BTreeTableSpace,
    pub is_root: bool,
    pub is_leaf: bool,
    pub has_fixed_key_value_size: bool,
    pub toc_entries: Vec<BTreeTocEntry>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OmapRecord {
    pub oid: u64,
    pub xid: u64,
    pub flags: u32,
    pub size_bytes: u32,
    pub physical_address: u64,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct BTreeIndexRecord {
    pub max_oid: u64,
    pub max_xid: u64,
    pub child_oid: u64,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct BTreeChildSelection {
    pub requested_oid: u64,
    pub requested_xid: u64,
    pub matched: bool,
    pub selected_child_oid: Option<u64>,
    pub selected_max_oid: Option<u64>,
    pub selected_max_xid: Option<u64>,
    pub candidate_count: usize,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct FileSystemDirectoryRecord {
    pub parent_id: u64,
    pub name_hash: u64,
    pub object_id: u64,
    pub item_kind_raw: u16,
    pub name: String,
    pub logical_size: u64,
    pub physical_block: Option<u64>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OmapLookup {
    pub requested_oid: u64,
    pub requested_xid: u64,
    pub matched: bool,
    pub matched_oid: Option<u64>,
    pub matched_xid: Option<u64>,
    pub flags: Option<u32>,
    pub size_bytes: Option<u32>,
    pub physical_address: Option<u64>,
    pub candidate_count: usize,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SyntheticDirectoryRecord {
    pub parent_id: u64,
    pub object_id: u64,
    pub kind_raw: u16,
    pub kind: String,
    pub flags: u32,
    pub logical_size: u64,
    pub data_oid: u64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SyntheticFileExtentRecord {
    pub file_object_id: u64,
    pub logical_offset: u64,
    pub physical_block: u64,
    pub length_bytes: u64,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct GptHeader {
    pub revision: u32,
    pub header_size: u32,
    pub stored_header_crc32: u32,
    pub computed_header_crc32: u32,
    pub header_crc32_valid: bool,
    pub current_lba: u64,
    pub backup_lba: u64,
    pub first_usable_lba: u64,
    pub last_usable_lba: u64,
    pub disk_guid: String,
    pub partition_entry_lba: u64,
    pub number_of_partition_entries: u32,
    pub size_of_partition_entry: u32,
    pub stored_partition_entries_crc32: u32,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct GptPartitionEntry {
    pub type_guid: String,
    pub unique_guid: String,
    pub first_lba: u64,
    pub last_lba: u64,
    pub attributes: u64,
    pub name: String,
    pub is_apfs: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct GptEntriesChecksum {
    pub stored_crc32: u32,
    pub computed_crc32: u32,
    pub valid: bool,
    pub checked_bytes: usize,
}

pub fn parse_object_header(input: &[u8]) -> Result<ObjectHeader, ParseError> {
    require_len(input, APFS_OBJECT_HEADER_SIZE, "obj_phys_t")?;
    let checksum = read_u64_le(input, 0)?;
    let object_type_raw = read_u32_le(input, 24)?;
    Ok(ObjectHeader {
        checksum_hex: format_u64_hex(checksum),
        checksum_u64: checksum,
        oid: read_u64_le(input, 8)?,
        xid: read_u64_le(input, 16)?,
        object_type_raw,
        object_type: (object_type_raw & 0xffff) as u16,
        object_flags: (object_type_raw >> 16) as u16,
        object_subtype_raw: read_u32_le(input, 28)?,
    })
}

pub fn parse_nx_superblock(input: &[u8]) -> Result<ContainerSuperblock, ParseError> {
    parse_nx_superblock_inner(input, false)
}

pub fn parse_nx_superblock_with_checksum(input: &[u8]) -> Result<ContainerSuperblock, ParseError> {
    parse_nx_superblock_inner(input, true)
}

fn parse_nx_superblock_inner(
    input: &[u8],
    include_checksum: bool,
) -> Result<ContainerSuperblock, ParseError> {
    require_len(input, NX_SUPERBLOCK_MIN_SIZE, "nx_superblock_t")?;
    let object = parse_object_header(&input[..APFS_OBJECT_HEADER_SIZE])?;
    let magic_bytes = read_array_4(input, 32)?;
    if magic_bytes != NX_MAGIC_BYTES {
        return Err(ParseError::MagicMismatch { found: magic_bytes });
    }

    let block_size = read_u32_le(input, 36)?;
    validate_block_size(block_size)?;
    let block_count = read_u64_le(input, 40)?;
    let container_size_bytes = u128::from(block_size).checked_mul(u128::from(block_count));
    let max_file_systems = read_u32_le(input, 180)?;
    let filesystem_oids = parse_filesystem_oid_prefix(input, max_file_systems)?;
    let checksum = if include_checksum {
        Some(validate_object_checksum(input)?)
    } else {
        None
    };

    Ok(ContainerSuperblock {
        object,
        checksum,
        magic: String::from("NXSB"),
        block_size,
        block_count,
        container_size_bytes,
        features: read_u64_le(input, 48)?,
        readonly_compatible_features: read_u64_le(input, 56)?,
        incompatible_features: read_u64_le(input, 64)?,
        uuid: format_guid(&input[72..88]),
        next_oid: read_u64_le(input, 88)?,
        next_xid: read_u64_le(input, 96)?,
        checkpoint_descriptor_blocks_raw: read_u32_le(input, 104)?,
        checkpoint_data_blocks_raw: read_u32_le(input, 108)?,
        checkpoint_descriptor_base: read_u64_le(input, 112)?,
        checkpoint_data_base: read_u64_le(input, 120)?,
        checkpoint_descriptor_next: read_u32_le(input, 128)?,
        checkpoint_data_next: read_u32_le(input, 132)?,
        checkpoint_descriptor_index: read_u32_le(input, 136)?,
        checkpoint_descriptor_len: read_u32_le(input, 140)?,
        checkpoint_data_index: read_u32_le(input, 144)?,
        checkpoint_data_len: read_u32_le(input, 148)?,
        spaceman_oid: read_u64_le(input, 152)?,
        omap_oid: read_u64_le(input, 160)?,
        reaper_oid: read_u64_le(input, 168)?,
        test_type: read_u32_le(input, 176)?,
        max_file_systems,
        filesystem_oids,
    })
}

pub fn parse_checkpoint_map_block_with_checksum(
    input: &[u8],
) -> Result<CheckpointMapBlock, ParseError> {
    require_len(input, CHECKPOINT_MAP_HEADER_SIZE, "checkpoint_map_phys_t")?;
    let object = parse_object_header(input)?;
    if object.object_type != OBJECT_TYPE_CHECKPOINT_MAP {
        return Err(ParseError::ObjectTypeMismatch {
            expected: OBJECT_TYPE_CHECKPOINT_MAP,
            actual: object.object_type,
        });
    }
    let checksum = validate_object_checksum(input)?;
    let flags = read_u32_le(input, 32)?;
    let count = read_u32_le(input, 36)?;
    let max_count =
        input.len().saturating_sub(CHECKPOINT_MAP_HEADER_SIZE) / CHECKPOINT_MAPPING_SIZE;
    let requested = usize::try_from(count).unwrap_or(usize::MAX);
    if requested > max_count {
        return Err(ParseError::InvalidCheckpointMapCount {
            count,
            max: max_count,
        });
    }

    let mut mappings = Vec::with_capacity(requested);
    for index in 0..requested {
        let offset = CHECKPOINT_MAP_HEADER_SIZE + index * CHECKPOINT_MAPPING_SIZE;
        mappings.push(parse_checkpoint_mapping(
            &input[offset..offset + CHECKPOINT_MAPPING_SIZE],
        )?);
    }

    Ok(CheckpointMapBlock {
        object,
        checksum,
        flags,
        count,
        is_last: flags & CHECKPOINT_MAP_LAST != 0,
        mappings,
    })
}

pub fn parse_checkpoint_mapping(input: &[u8]) -> Result<CheckpointMapping, ParseError> {
    require_len(input, CHECKPOINT_MAPPING_SIZE, "checkpoint_mapping_t")?;
    let object_type_raw = read_u32_le(input, 0)?;
    Ok(CheckpointMapping {
        object_type_raw,
        object_type: (object_type_raw & 0xffff) as u16,
        object_flags: (object_type_raw >> 16) as u16,
        object_subtype_raw: read_u32_le(input, 4)?,
        size_bytes: read_u32_le(input, 8)?,
        pad: read_u32_le(input, 12)?,
        filesystem_oid: read_u64_le(input, 16)?,
        oid: read_u64_le(input, 24)?,
        physical_address: read_u64_le(input, 32)?,
    })
}

pub fn parse_omap_phys_with_checksum(input: &[u8]) -> Result<ObjectMap, ParseError> {
    require_len(input, OMAP_PHYS_MIN_SIZE, "omap_phys_t")?;
    let object = parse_object_header(input)?;
    if object.object_type != OBJECT_TYPE_OMAP {
        return Err(ParseError::ObjectTypeMismatch {
            expected: OBJECT_TYPE_OMAP,
            actual: object.object_type,
        });
    }
    let checksum = validate_object_checksum(input)?;
    Ok(ObjectMap {
        object,
        checksum,
        flags: read_u32_le(input, 32)?,
        snapshot_count: read_u32_le(input, 36)?,
        tree_type_raw: read_u32_le(input, 40)?,
        snapshot_tree_type_raw: read_u32_le(input, 44)?,
        tree_oid: read_u64_le(input, 48)?,
        snapshot_tree_oid: read_u64_le(input, 56)?,
        most_recent_snapshot_xid: read_u64_le(input, 64)?,
        pending_revert_min_xid: read_u64_le(input, 72)?,
        pending_revert_max_xid: read_u64_le(input, 80)?,
    })
}

pub fn parse_apfs_volume_superblock_with_checksum(
    input: &[u8],
) -> Result<VolumeSuperblock, ParseError> {
    require_len(input, APFS_VOLUME_SUPERBLOCK_MIN_SIZE, "apfs_superblock_t")?;
    let object = parse_object_header(input)?;
    if object.object_type != OBJECT_TYPE_FS {
        return Err(ParseError::ObjectTypeMismatch {
            expected: OBJECT_TYPE_FS,
            actual: object.object_type,
        });
    }
    let magic_bytes = read_array_4(input, 32)?;
    if magic_bytes != APFS_VOLUME_MAGIC_BYTES {
        return Err(ParseError::VolumeMagicMismatch { found: magic_bytes });
    }
    let checksum = validate_object_checksum(input)?;
    Ok(VolumeSuperblock {
        object,
        checksum,
        magic: "APSB".to_owned(),
        fs_index: read_u32_le(input, 36)?,
        features: read_u64_le(input, 40)?,
        readonly_compatible_features: read_u64_le(input, 48)?,
        incompatible_features: read_u64_le(input, 56)?,
        unmount_time: read_u64_le(input, 64)?,
        fs_reserve_block_count: read_u64_le(input, 72)?,
        fs_quota_block_count: read_u64_le(input, 80)?,
        fs_alloc_count: read_u64_le(input, 88)?,
        root_tree_oid: read_u64_le(input, 112)?,
        extentref_tree_oid: read_u64_le(input, 120)?,
        snap_meta_tree_oid: read_u64_le(input, 128)?,
        next_obj_id: read_u64_le(input, 152)?,
        num_files: read_u64_le(input, 160)?,
        num_directories: read_u64_le(input, 168)?,
        num_symlinks: read_u64_le(input, 176)?,
        num_other_fsobjects: read_u64_le(input, 184)?,
        num_snapshots: read_u64_le(input, 192)?,
        total_blocks_alloced: read_u64_le(input, 200)?,
        total_blocks_freed: read_u64_le(input, 208)?,
        volume_uuid: format_guid(&input[216..232]),
        last_mod_time: read_u64_le(input, 232)?,
        fs_flags: read_u64_le(input, 240)?,
        volume_name: decode_c_string(
            &input[APFS_VOLUME_NAME_OFFSET..APFS_VOLUME_NAME_OFFSET + APFS_VOLUME_NAME_LEN],
        ),
        role: read_u16_le(input, APFS_VOLUME_ROLE_OFFSET)?,
    })
}

pub fn parse_btree_node_with_checksum(input: &[u8]) -> Result<BTreeNode, ParseError> {
    require_len(input, BTREE_NODE_MIN_SIZE, "btree_node_phys_t")?;
    let object = parse_object_header(input)?;
    if object.object_type != OBJECT_TYPE_BTREE && object.object_type != OBJECT_TYPE_BTREE_NODE {
        return Err(ParseError::ObjectTypeMismatch {
            expected: OBJECT_TYPE_BTREE_NODE,
            actual: object.object_type,
        });
    }
    let checksum = validate_object_checksum(input)?;
    let flags_raw = read_u16_le(input, 32)?;
    let level = read_u16_le(input, 34)?;
    let key_count = read_u32_le(input, 36)?;
    let table_space = read_table_space(input, 40)?;
    let free_space = read_table_space(input, 44)?;
    let key_free_list = read_table_space(input, 48)?;
    let value_free_list = read_table_space(input, 52)?;
    let toc_entries = parse_btree_toc_entries(input, table_space, key_count)?;

    Ok(BTreeNode {
        object,
        checksum,
        flags_raw,
        level,
        key_count,
        table_space,
        free_space,
        key_free_list,
        value_free_list,
        is_root: flags_raw & BTREE_NODE_ROOT != 0,
        is_leaf: flags_raw & BTREE_NODE_LEAF != 0,
        has_fixed_key_value_size: flags_raw & BTREE_NODE_FIXED_KV_SIZE != 0,
        toc_entries,
    })
}

fn read_table_space(input: &[u8], offset: usize) -> Result<BTreeTableSpace, ParseError> {
    Ok(BTreeTableSpace {
        offset: read_u16_le(input, offset)?,
        length: read_u16_le(input, offset + 2)?,
    })
}

fn parse_btree_toc_entries(
    input: &[u8],
    table_space: BTreeTableSpace,
    key_count: u32,
) -> Result<Vec<BTreeTocEntry>, ParseError> {
    let toc_offset = BTREE_NODE_MIN_SIZE + usize::from(table_space.offset);
    let toc_len = usize::from(table_space.length);
    let toc_end = toc_offset
        .checked_add(toc_len)
        .ok_or(ParseError::BTreeTableOutOfBounds {
            offset: toc_offset,
            length: toc_len,
            block_len: input.len(),
        })?;
    if toc_end > input.len() {
        return Err(ParseError::BTreeTableOutOfBounds {
            offset: toc_offset,
            length: toc_len,
            block_len: input.len(),
        });
    }
    let max_entries_by_table = toc_len / BTREE_TOC_ENTRY_SIZE;
    let requested = usize::try_from(key_count)
        .unwrap_or(usize::MAX)
        .min(max_entries_by_table)
        .min(16_384);
    let mut entries = Vec::with_capacity(requested);
    for index in 0..requested {
        let offset = toc_offset + index * BTREE_TOC_ENTRY_SIZE;
        entries.push(BTreeTocEntry {
            key_offset: read_u16_le(input, offset)?,
            value_offset: read_u16_le(input, offset + 2)?,
        });
    }
    Ok(entries)
}

/// Interpret a B-tree leaf node as a preliminary OMAP leaf and parse key/value records.
///
/// This is intentionally conservative and only decodes records whose key and value offsets
/// fit inside the supplied node block. Offsets are interpreted relative to the APFS B-tree
/// node payload start (`BTREE_NODE_MIN_SIZE`), matching the synthetic fixtures in this package.
pub fn parse_omap_records_from_btree_node(
    input: &[u8],
    node: &BTreeNode,
) -> Result<Vec<OmapRecord>, ParseError> {
    if !node.is_leaf {
        return Ok(Vec::new());
    }
    let requested = node.toc_entries.len().min(16_384);
    let mut records = Vec::with_capacity(requested);
    for toc in node.toc_entries.iter().take(requested) {
        let key_offset = BTREE_NODE_MIN_SIZE + usize::from(toc.key_offset);
        let value_offset = BTREE_NODE_MIN_SIZE + usize::from(toc.value_offset);
        let key_end =
            key_offset
                .checked_add(OMAP_KEY_SIZE)
                .ok_or(ParseError::BTreeTableOutOfBounds {
                    offset: key_offset,
                    length: OMAP_KEY_SIZE,
                    block_len: input.len(),
                })?;
        let value_end =
            value_offset
                .checked_add(OMAP_VALUE_SIZE)
                .ok_or(ParseError::BTreeTableOutOfBounds {
                    offset: value_offset,
                    length: OMAP_VALUE_SIZE,
                    block_len: input.len(),
                })?;
        if key_end > input.len() {
            return Err(ParseError::BTreeTableOutOfBounds {
                offset: key_offset,
                length: OMAP_KEY_SIZE,
                block_len: input.len(),
            });
        }
        if value_end > input.len() {
            return Err(ParseError::BTreeTableOutOfBounds {
                offset: value_offset,
                length: OMAP_VALUE_SIZE,
                block_len: input.len(),
            });
        }
        records.push(OmapRecord {
            oid: read_u64_le(input, key_offset)?,
            xid: read_u64_le(input, key_offset + 8)?,
            flags: read_u32_le(input, value_offset)?,
            size_bytes: read_u32_le(input, value_offset + 4)?,
            physical_address: read_u64_le(input, value_offset + 8)?,
        });
    }
    Ok(records)
}

/// Interpret a non-leaf synthetic OMAP B-tree node as index records.
///
/// The key is decoded as `(max_oid, max_xid)` and the value is decoded as `child_oid`.
/// This is a deliberately bounded parser for the synthetic two-level fixture; it is not
/// full production APFS B-tree traversal yet.
pub fn parse_omap_index_records_from_btree_node(
    input: &[u8],
    node: &BTreeNode,
) -> Result<Vec<BTreeIndexRecord>, ParseError> {
    if node.is_leaf {
        return Ok(Vec::new());
    }
    let requested = node.toc_entries.len().min(16_384);
    let mut records = Vec::with_capacity(requested);
    for toc in node.toc_entries.iter().take(requested) {
        let key_offset = BTREE_NODE_MIN_SIZE + usize::from(toc.key_offset);
        let value_offset = BTREE_NODE_MIN_SIZE + usize::from(toc.value_offset);
        let key_end =
            key_offset
                .checked_add(OMAP_KEY_SIZE)
                .ok_or(ParseError::BTreeTableOutOfBounds {
                    offset: key_offset,
                    length: OMAP_KEY_SIZE,
                    block_len: input.len(),
                })?;
        let value_end = value_offset
            .checked_add(8)
            .ok_or(ParseError::BTreeTableOutOfBounds {
                offset: value_offset,
                length: 8,
                block_len: input.len(),
            })?;
        if key_end > input.len() {
            return Err(ParseError::BTreeTableOutOfBounds {
                offset: key_offset,
                length: OMAP_KEY_SIZE,
                block_len: input.len(),
            });
        }
        if value_end > input.len() {
            return Err(ParseError::BTreeTableOutOfBounds {
                offset: value_offset,
                length: 8,
                block_len: input.len(),
            });
        }
        records.push(BTreeIndexRecord {
            max_oid: read_u64_le(input, key_offset)?,
            max_xid: read_u64_le(input, key_offset + 8)?,
            child_oid: read_u64_le(input, value_offset)?,
        });
    }
    records.sort_by_key(|record| (record.max_oid, record.max_xid, record.child_oid));
    Ok(records)
}

/// Select the synthetic B-tree child whose `(max_oid, max_xid)` covers the request.
///
/// If no key is greater than or equal to the request, the last child is selected. This
/// matches the synthetic range-fixture convention and keeps the traversal bounded.
pub fn select_synthetic_btree_child(
    records: &[BTreeIndexRecord],
    requested_oid: u64,
    requested_xid: u64,
) -> BTreeChildSelection {
    if records.is_empty() {
        return BTreeChildSelection {
            requested_oid,
            requested_xid,
            matched: false,
            selected_child_oid: None,
            selected_max_oid: None,
            selected_max_xid: None,
            candidate_count: 0,
        };
    }

    let selected = records
        .iter()
        .find(|record| (requested_oid, requested_xid) <= (record.max_oid, record.max_xid))
        .unwrap_or_else(|| records.last().expect("records is not empty"));

    BTreeChildSelection {
        requested_oid,
        requested_xid,
        matched: true,
        selected_child_oid: Some(selected.child_oid),
        selected_max_oid: Some(selected.max_oid),
        selected_max_xid: Some(selected.max_xid),
        candidate_count: records.len(),
    }
}

/// Lookup an object ID in already-decoded OMAP records.
///
/// This is the first deliberately small object-map lookup helper. It does not traverse
/// a general APFS B-tree yet; it chooses the record with the requested object ID and
/// the greatest record transaction ID less than or equal to the requested transaction ID.
pub fn parse_synthetic_directory_records_from_btree_node(
    input: &[u8],
    node: &BTreeNode,
) -> Result<Vec<FileSystemDirectoryRecord>, ParseError> {
    let mut records = Vec::new();
    for entry in &node.toc_entries {
        let key_base = BTREE_NODE_MIN_SIZE + usize::from(entry.key_offset);
        let value_base = BTREE_NODE_MIN_SIZE + usize::from(entry.value_offset);
        require_len(
            input,
            key_base + SYNTHETIC_FS_DIR_KEY_SIZE,
            "synthetic fs directory key",
        )?;
        require_len(
            input,
            value_base + SYNTHETIC_FS_DIR_VALUE_HEADER_SIZE,
            "synthetic fs directory value",
        )?;

        let parent_id = read_u64_le(input, key_base)?;
        let name_hash = read_u64_le(input, key_base + 8)?;
        let object_id = read_u64_le(input, value_base)?;
        let item_kind_raw = read_u16_le(input, value_base + 8)?;
        let name_len = usize::from(read_u16_le(input, value_base + 10)?);
        let logical_size = read_u64_le(input, value_base + 12)?;
        let physical_block_raw = read_u64_le(input, value_base + 20)?;
        let name_start = value_base + SYNTHETIC_FS_DIR_VALUE_HEADER_SIZE;
        require_len(input, name_start + name_len, "synthetic fs directory name")?;
        let name = String::from_utf8_lossy(&input[name_start..name_start + name_len]).into_owned();

        records.push(FileSystemDirectoryRecord {
            parent_id,
            name_hash,
            object_id,
            item_kind_raw,
            name,
            logical_size,
            physical_block: if physical_block_raw == 0 {
                None
            } else {
                Some(physical_block_raw)
            },
        });
    }
    Ok(records)
}

pub fn lookup_omap_record(
    records: &[OmapRecord],
    requested_oid: u64,
    requested_xid: u64,
) -> OmapLookup {
    let mut best: Option<&OmapRecord> = None;
    let mut candidate_count = 0_usize;
    for record in records {
        if record.oid != requested_oid || record.xid > requested_xid {
            continue;
        }
        candidate_count += 1;
        let replace = best.map(|current| record.xid > current.xid).unwrap_or(true);
        if replace {
            best = Some(record);
        }
    }

    if let Some(record) = best {
        OmapLookup {
            requested_oid,
            requested_xid,
            matched: true,
            matched_oid: Some(record.oid),
            matched_xid: Some(record.xid),
            flags: Some(record.flags),
            size_bytes: Some(record.size_bytes),
            physical_address: Some(record.physical_address),
            candidate_count,
        }
    } else {
        OmapLookup {
            requested_oid,
            requested_xid,
            matched: false,
            matched_oid: None,
            matched_xid: None,
            flags: None,
            size_bytes: None,
            physical_address: None,
            candidate_count,
        }
    }
}

/// Parse synthetic directory records from a B-tree leaf node.
///
/// This is intentionally not a production APFS directory-record decoder. It supports
/// parser-development fixtures where each key is `(parent_id, object_id)` and each
/// value is `(kind, name_len, flags, logical_size, data_oid, name_bytes...)`.
pub fn parse_synthetic_directory_records_v2_from_btree_node(
    input: &[u8],
    node: &BTreeNode,
) -> Result<Vec<SyntheticDirectoryRecord>, ParseError> {
    if !node.is_leaf {
        return Ok(Vec::new());
    }
    let requested = node.toc_entries.len().min(16_384);
    let mut records = Vec::with_capacity(requested);
    for toc in node.toc_entries.iter().take(requested) {
        let key_offset = BTREE_NODE_MIN_SIZE + usize::from(toc.key_offset);
        let value_offset = BTREE_NODE_MIN_SIZE + usize::from(toc.value_offset);
        let key_end = key_offset.checked_add(SYNTHETIC_DIR_KEY_SIZE).ok_or(
            ParseError::BTreeTableOutOfBounds {
                offset: key_offset,
                length: SYNTHETIC_DIR_KEY_SIZE,
                block_len: input.len(),
            },
        )?;
        let value_header_end = value_offset
            .checked_add(SYNTHETIC_DIR_VALUE_HEADER_SIZE)
            .ok_or(ParseError::BTreeTableOutOfBounds {
                offset: value_offset,
                length: SYNTHETIC_DIR_VALUE_HEADER_SIZE,
                block_len: input.len(),
            })?;
        if key_end > input.len() {
            return Err(ParseError::BTreeTableOutOfBounds {
                offset: key_offset,
                length: SYNTHETIC_DIR_KEY_SIZE,
                block_len: input.len(),
            });
        }
        if value_header_end > input.len() {
            return Err(ParseError::BTreeTableOutOfBounds {
                offset: value_offset,
                length: SYNTHETIC_DIR_VALUE_HEADER_SIZE,
                block_len: input.len(),
            });
        }
        let kind_raw = read_u16_le(input, value_offset)?;
        let name_len = usize::from(read_u16_le(input, value_offset + 2)?);
        let name_offset = value_offset + SYNTHETIC_DIR_VALUE_HEADER_SIZE;
        let name_end =
            name_offset
                .checked_add(name_len)
                .ok_or(ParseError::BTreeTableOutOfBounds {
                    offset: name_offset,
                    length: name_len,
                    block_len: input.len(),
                })?;
        if name_end > input.len() {
            return Err(ParseError::BTreeTableOutOfBounds {
                offset: name_offset,
                length: name_len,
                block_len: input.len(),
            });
        }
        records.push(SyntheticDirectoryRecord {
            parent_id: read_u64_le(input, key_offset)?,
            object_id: read_u64_le(input, key_offset + 8)?,
            kind_raw,
            kind: synthetic_file_kind_name(kind_raw).to_owned(),
            flags: read_u32_le(input, value_offset + 4)?,
            logical_size: read_u64_le(input, value_offset + 8)?,
            data_oid: read_u64_le(input, value_offset + 16)?,
            name: String::from_utf8_lossy(&input[name_offset..name_end]).into_owned(),
        });
    }
    records.sort_by_key(|record| (record.parent_id, record.name.clone(), record.object_id));
    Ok(records)
}

/// Parse synthetic file extent records from a B-tree leaf node.
///
/// This parser is fixture-only. Keys are `(file_object_id, logical_offset)` and values
/// are `(physical_block, length_bytes)`.
pub fn parse_synthetic_file_extent_records_from_btree_node(
    input: &[u8],
    node: &BTreeNode,
) -> Result<Vec<SyntheticFileExtentRecord>, ParseError> {
    if !node.is_leaf {
        return Ok(Vec::new());
    }
    let requested = node.toc_entries.len().min(16_384);
    let mut records = Vec::with_capacity(requested);
    for toc in node.toc_entries.iter().take(requested) {
        let key_offset = BTREE_NODE_MIN_SIZE + usize::from(toc.key_offset);
        let value_offset = BTREE_NODE_MIN_SIZE + usize::from(toc.value_offset);
        let key_end = key_offset.checked_add(SYNTHETIC_EXTENT_KEY_SIZE).ok_or(
            ParseError::BTreeTableOutOfBounds {
                offset: key_offset,
                length: SYNTHETIC_EXTENT_KEY_SIZE,
                block_len: input.len(),
            },
        )?;
        let value_end = value_offset
            .checked_add(SYNTHETIC_EXTENT_VALUE_SIZE)
            .ok_or(ParseError::BTreeTableOutOfBounds {
                offset: value_offset,
                length: SYNTHETIC_EXTENT_VALUE_SIZE,
                block_len: input.len(),
            })?;
        if key_end > input.len() {
            return Err(ParseError::BTreeTableOutOfBounds {
                offset: key_offset,
                length: SYNTHETIC_EXTENT_KEY_SIZE,
                block_len: input.len(),
            });
        }
        if value_end > input.len() {
            return Err(ParseError::BTreeTableOutOfBounds {
                offset: value_offset,
                length: SYNTHETIC_EXTENT_VALUE_SIZE,
                block_len: input.len(),
            });
        }
        records.push(SyntheticFileExtentRecord {
            file_object_id: read_u64_le(input, key_offset)?,
            logical_offset: read_u64_le(input, key_offset + 8)?,
            physical_block: read_u64_le(input, value_offset)?,
            length_bytes: read_u64_le(input, value_offset + 8)?,
        });
    }
    records.sort_by_key(|record| {
        (
            record.file_object_id,
            record.logical_offset,
            record.physical_block,
        )
    });
    Ok(records)
}

fn synthetic_file_kind_name(kind_raw: u16) -> &'static str {
    match kind_raw {
        SYNTHETIC_FILE_KIND_FILE => "file",
        SYNTHETIC_FILE_KIND_DIRECTORY => "directory",
        SYNTHETIC_FILE_KIND_SYMLINK => "symlink",
        _ => "unknown",
    }
}

pub fn validate_object_checksum(object_bytes: &[u8]) -> Result<ObjectChecksum, ParseError> {
    let stored_checksum = read_u64_le(object_bytes, 0)?;
    let computed_checksum = apfs_fletcher64(object_bytes)?;
    Ok(ObjectChecksum {
        stored_checksum,
        stored_checksum_hex: format_u64_hex(stored_checksum),
        computed_checksum,
        computed_checksum_hex: format_u64_hex(computed_checksum),
        valid: stored_checksum == computed_checksum,
        checked_bytes: object_bytes.len(),
    })
}

/// Calculate the APFS object Fletcher-64 checksum for an object block.
///
/// The stored checksum field is treated as zero while the checksum is calculated.
pub fn apfs_fletcher64(object_bytes: &[u8]) -> Result<u64, ParseError> {
    if object_bytes.len() < 12 || (object_bytes.len() - 8) % 4 != 0 {
        return Err(ParseError::InvalidChecksumInputLength(object_bytes.len()));
    }

    let mut checksum_input = object_bytes.to_vec();
    checksum_input[..8].fill(0);

    let mut c0 = 0_u64;
    let mut c1 = 0_u64;
    for chunk in checksum_input.chunks_exact(4) {
        let value = u64::from(u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]));
        c0 = (c0 + value) % FLETCHER64_MODULUS;
        c1 = (c1 + c0) % FLETCHER64_MODULUS;
    }
    let checksum_lower =
        (FLETCHER64_MODULUS - ((c0 + c1) % FLETCHER64_MODULUS)) % FLETCHER64_MODULUS;
    let checksum_upper =
        (FLETCHER64_MODULUS - ((c0 + checksum_lower) % FLETCHER64_MODULUS)) % FLETCHER64_MODULUS;
    Ok((checksum_upper << 32) | checksum_lower)
}

pub fn parse_gpt_header(sector: &[u8]) -> Result<GptHeader, ParseError> {
    require_len(sector, GPT_HEADER_MIN_SIZE, "gpt_header")?;
    if sector[0..8] != GPT_HEADER_SIGNATURE[..] {
        return Err(ParseError::GptSignatureMismatch);
    }
    let header_size = read_u32_le(sector, 12)?;
    if header_size < GPT_HEADER_MIN_SIZE as u32 || header_size > GPT_SECTOR_SIZE as u32 {
        return Err(ParseError::InvalidGptHeaderSize(header_size));
    }
    let size_of_partition_entry = read_u32_le(sector, 84)?;
    if size_of_partition_entry < GPT_PARTITION_ENTRY_MIN_SIZE as u32 {
        return Err(ParseError::InvalidGptPartitionEntrySize(
            size_of_partition_entry,
        ));
    }

    let stored_header_crc32 = read_u32_le(sector, 16)?;
    let computed_header_crc32 = gpt_header_crc32(sector, header_size as usize)?;

    Ok(GptHeader {
        revision: read_u32_le(sector, 8)?,
        header_size,
        stored_header_crc32,
        computed_header_crc32,
        header_crc32_valid: stored_header_crc32 == computed_header_crc32,
        current_lba: read_u64_le(sector, 24)?,
        backup_lba: read_u64_le(sector, 32)?,
        first_usable_lba: read_u64_le(sector, 40)?,
        last_usable_lba: read_u64_le(sector, 48)?,
        disk_guid: format_guid(&sector[56..72]),
        partition_entry_lba: read_u64_le(sector, 72)?,
        number_of_partition_entries: read_u32_le(sector, 80)?,
        size_of_partition_entry,
        stored_partition_entries_crc32: read_u32_le(sector, 88)?,
    })
}

pub fn gpt_header_crc32(sector: &[u8], header_size: usize) -> Result<u32, ParseError> {
    require_len(sector, header_size, "gpt_header_crc_input")?;
    if !(GPT_HEADER_MIN_SIZE..=GPT_SECTOR_SIZE).contains(&header_size) {
        return Err(ParseError::InvalidGptHeaderSize(header_size as u32));
    }
    let mut bytes = sector[..header_size].to_vec();
    bytes[16..20].fill(0);
    Ok(crc32_ieee(&bytes))
}

pub fn validate_gpt_entries_checksum(entries: &[u8], header: &GptHeader) -> GptEntriesChecksum {
    let computed_crc32 = crc32_ieee(entries);
    GptEntriesChecksum {
        stored_crc32: header.stored_partition_entries_crc32,
        computed_crc32,
        valid: header.stored_partition_entries_crc32 == computed_crc32,
        checked_bytes: entries.len(),
    }
}

pub fn parse_gpt_partition_entry(entry: &[u8]) -> Result<Option<GptPartitionEntry>, ParseError> {
    require_len(entry, GPT_PARTITION_ENTRY_MIN_SIZE, "gpt_partition_entry")?;
    let type_guid_bytes: [u8; 16] = entry[0..16].try_into().map_err(|_| ParseError::TooShort {
        structure: "gpt_partition_type_guid",
        needed: 16,
        actual: entry.len(),
    })?;
    if type_guid_bytes == [0_u8; 16] {
        return Ok(None);
    }
    let unique_guid_bytes: [u8; 16] =
        entry[16..32].try_into().map_err(|_| ParseError::TooShort {
            structure: "gpt_partition_unique_guid",
            needed: 32,
            actual: entry.len(),
        })?;
    let name_bytes = &entry[56..GPT_PARTITION_ENTRY_MIN_SIZE];
    Ok(Some(GptPartitionEntry {
        type_guid: format_guid(&type_guid_bytes),
        unique_guid: format_guid(&unique_guid_bytes),
        first_lba: read_u64_le(entry, 32)?,
        last_lba: read_u64_le(entry, 40)?,
        attributes: read_u64_le(entry, 48)?,
        name: decode_utf16le_name(name_bytes),
        is_apfs: type_guid_bytes == APFS_GPT_TYPE_GUID_BYTES,
    }))
}

pub fn gpt_entries_byte_len(header: &GptHeader) -> Result<usize, ParseError> {
    let entries = u64::from(header.number_of_partition_entries);
    let entry_size = u64::from(header.size_of_partition_entry);
    let total = entries
        .checked_mul(entry_size)
        .ok_or(ParseError::GptArithmeticOverflow)?;
    usize::try_from(total).map_err(|_| ParseError::GptArithmeticOverflow)
}

pub fn crc32_ieee(bytes: &[u8]) -> u32 {
    let mut crc = 0xffff_ffff_u32;
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xedb8_8320;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}

fn parse_filesystem_oid_prefix(
    input: &[u8],
    max_file_systems: u32,
) -> Result<Vec<u64>, ParseError> {
    let available_bytes = input.len().saturating_sub(184);
    let available_oids = available_bytes / 8;
    let requested = usize::try_from(max_file_systems)
        .unwrap_or(usize::MAX)
        .min(available_oids)
        .min(100);
    let mut oids = Vec::with_capacity(requested);
    for index in 0..requested {
        let offset = 184 + index * 8;
        let oid = read_u64_le(input, offset)?;
        if oid != 0 {
            oids.push(oid);
        }
    }
    Ok(oids)
}

fn validate_block_size(block_size: u32) -> Result<(), ParseError> {
    if !(512..=1_048_576).contains(&block_size) || !block_size.is_power_of_two() {
        return Err(ParseError::InvalidBlockSize(block_size));
    }
    Ok(())
}

fn require_len(input: &[u8], needed: usize, structure: &'static str) -> Result<(), ParseError> {
    if input.len() < needed {
        return Err(ParseError::TooShort {
            structure,
            needed,
            actual: input.len(),
        });
    }
    Ok(())
}

fn read_array_4(input: &[u8], offset: usize) -> Result<[u8; 4], ParseError> {
    require_len(input, offset + 4, "u32")?;
    let mut out = [0_u8; 4];
    out.copy_from_slice(&input[offset..offset + 4]);
    Ok(out)
}

fn read_u16_le(input: &[u8], offset: usize) -> Result<u16, ParseError> {
    require_len(input, offset + 2, "u16")?;
    Ok(u16::from_le_bytes([input[offset], input[offset + 1]]))
}

fn read_u32_le(input: &[u8], offset: usize) -> Result<u32, ParseError> {
    Ok(u32::from_le_bytes(read_array_4(input, offset)?))
}

fn read_u64_le(input: &[u8], offset: usize) -> Result<u64, ParseError> {
    require_len(input, offset + 8, "u64")?;
    let mut out = [0_u8; 8];
    out.copy_from_slice(&input[offset..offset + 8]);
    Ok(u64::from_le_bytes(out))
}

fn format_guid(bytes: &[u8]) -> String {
    debug_assert_eq!(bytes.len(), 16);
    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[3], bytes[2], bytes[1], bytes[0], bytes[5], bytes[4], bytes[7], bytes[6],
        bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
    )
}

fn decode_c_string(bytes: &[u8]) -> String {
    let end = bytes
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).into_owned()
}

fn decode_utf16le_name(bytes: &[u8]) -> String {
    let mut code_units = Vec::new();
    for chunk in bytes.chunks_exact(2) {
        let code_unit = u16::from_le_bytes([chunk[0], chunk[1]]);
        if code_unit == 0 {
            break;
        }
        code_units.push(code_unit);
    }
    String::from_utf16_lossy(&code_units)
}

fn format_u64_hex(value: u64) -> String {
    format!("0x{value:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sign_block(block: &mut [u8]) {
        block[0..8].fill(0);
        let checksum = apfs_fletcher64(block).unwrap();
        block[0..8].copy_from_slice(&checksum.to_le_bytes());
    }

    fn minimal_nxsb() -> [u8; 4096] {
        let mut block = [0_u8; 4096];
        block[8..16].copy_from_slice(&1_u64.to_le_bytes());
        block[16..24].copy_from_slice(&10_u64.to_le_bytes());
        block[24..28]
            .copy_from_slice(&(OBJ_EPHEMERAL | u32::from(OBJECT_TYPE_NX_SUPERBLOCK)).to_le_bytes());
        block[32..36].copy_from_slice(b"NXSB");
        block[36..40].copy_from_slice(&4096_u32.to_le_bytes());
        block[40..48].copy_from_slice(&1024_u64.to_le_bytes());
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
    fn parses_minimal_nx_superblock_fields_and_checksum() {
        let block = minimal_nxsb();
        let parsed = parse_nx_superblock_with_checksum(&block).expect("valid minimal superblock");
        assert_eq!(parsed.magic, "NXSB");
        assert_eq!(parsed.block_size, 4096);
        assert_eq!(parsed.block_count, 1024);
        assert_eq!(parsed.container_size_bytes, Some(4_194_304));
        assert_eq!(parsed.max_file_systems, 1);
        assert_eq!(parsed.filesystem_oids, vec![42]);
        assert_eq!(parsed.omap_oid, 12);
        assert!(parsed.checksum.unwrap().valid);
    }

    #[test]
    fn refuses_wrong_magic() {
        let mut block = minimal_nxsb();
        block[32..36].copy_from_slice(b"NOPE");
        let err = parse_nx_superblock(&block).unwrap_err();
        assert!(matches!(err, ParseError::MagicMismatch { .. }));
    }

    #[test]
    fn detects_checksum_mismatch() {
        let mut block = minimal_nxsb();
        block[88] ^= 0xff;
        let checksum = validate_object_checksum(&block).unwrap();
        assert!(!checksum.valid);
    }

    #[test]
    fn apfs_fletcher64_matches_real_superblock_style_checksum() {
        let mut block = [0_u8; 4096];
        for (idx, byte) in block.iter_mut().enumerate().skip(8) {
            *byte = ((idx as u8).wrapping_mul(37)).wrapping_add(11);
        }
        let checksum = apfs_fletcher64(&block).unwrap();
        assert_eq!(checksum, 0xea0a_ad82_a13a_4125);
    }

    #[test]
    fn parses_checkpoint_map_block() {
        let mut block = [0_u8; 4096];
        block[8..16].copy_from_slice(&100_u64.to_le_bytes());
        block[16..24].copy_from_slice(&10_u64.to_le_bytes());
        block[24..28].copy_from_slice(
            &(OBJ_EPHEMERAL | u32::from(OBJECT_TYPE_CHECKPOINT_MAP)).to_le_bytes(),
        );
        block[32..36].copy_from_slice(&CHECKPOINT_MAP_LAST.to_le_bytes());
        block[36..40].copy_from_slice(&1_u32.to_le_bytes());
        block[40..44].copy_from_slice(&(OBJ_EPHEMERAL | u32::from(OBJECT_TYPE_OMAP)).to_le_bytes());
        block[48..52].copy_from_slice(&4096_u32.to_le_bytes());
        block[64..72].copy_from_slice(&12_u64.to_le_bytes());
        block[72..80].copy_from_slice(&10_u64.to_le_bytes());
        sign_block(&mut block);
        let parsed = parse_checkpoint_map_block_with_checksum(&block).unwrap();
        assert!(parsed.is_last);
        assert!(parsed.checksum.valid);
        assert_eq!(parsed.mappings.len(), 1);
        assert_eq!(parsed.mappings[0].object_type, OBJECT_TYPE_OMAP);
        assert_eq!(parsed.mappings[0].physical_address, 10);
    }

    #[test]
    fn parses_omap_phys() {
        let mut block = [0_u8; 4096];
        block[8..16].copy_from_slice(&12_u64.to_le_bytes());
        block[16..24].copy_from_slice(&10_u64.to_le_bytes());
        block[24..28].copy_from_slice(&(OBJ_EPHEMERAL | u32::from(OBJECT_TYPE_OMAP)).to_le_bytes());
        block[32..36].copy_from_slice(&1_u32.to_le_bytes());
        block[48..56].copy_from_slice(&99_u64.to_le_bytes());
        sign_block(&mut block);
        let omap = parse_omap_phys_with_checksum(&block).unwrap();
        assert!(omap.checksum.valid);
        assert_eq!(omap.object.oid, 12);
        assert_eq!(omap.tree_oid, 99);
    }

    #[test]
    fn parses_btree_node_header_and_toc() {
        let mut block = [0_u8; 4096];
        block[8..16].copy_from_slice(&99_u64.to_le_bytes());
        block[16..24].copy_from_slice(&10_u64.to_le_bytes());
        block[24..28]
            .copy_from_slice(&(OBJ_PHYSICAL | u32::from(OBJECT_TYPE_BTREE_NODE)).to_le_bytes());
        block[32..34].copy_from_slice(&(BTREE_NODE_ROOT | BTREE_NODE_LEAF).to_le_bytes());
        block[34..36].copy_from_slice(&0_u16.to_le_bytes());
        block[36..40].copy_from_slice(&2_u32.to_le_bytes());
        block[40..42].copy_from_slice(&0_u16.to_le_bytes());
        block[42..44].copy_from_slice(&8_u16.to_le_bytes());
        block[56..58].copy_from_slice(&100_u16.to_le_bytes());
        block[58..60].copy_from_slice(&200_u16.to_le_bytes());
        block[60..62].copy_from_slice(&120_u16.to_le_bytes());
        block[62..64].copy_from_slice(&240_u16.to_le_bytes());
        let key0 = BTREE_NODE_MIN_SIZE + 100;
        let val0 = BTREE_NODE_MIN_SIZE + 200;
        block[key0..key0 + 8].copy_from_slice(&500_u64.to_le_bytes());
        block[key0 + 8..key0 + 16].copy_from_slice(&10_u64.to_le_bytes());
        block[val0..val0 + 4].copy_from_slice(&0_u32.to_le_bytes());
        block[val0 + 4..val0 + 8].copy_from_slice(&4096_u32.to_le_bytes());
        block[val0 + 8..val0 + 16].copy_from_slice(&20_u64.to_le_bytes());
        sign_block(&mut block);
        let parsed = parse_btree_node_with_checksum(&block).unwrap();
        assert!(parsed.checksum.valid);
        assert!(parsed.is_root);
        assert!(parsed.is_leaf);
        assert_eq!(parsed.key_count, 2);
        assert_eq!(parsed.toc_entries.len(), 2);
        assert_eq!(parsed.toc_entries[0].key_offset, 100);
        assert_eq!(parsed.toc_entries[1].value_offset, 240);
        let records = parse_omap_records_from_btree_node(&block, &parsed).unwrap();
        assert_eq!(records[0].oid, 500);
        assert_eq!(records[0].physical_address, 20);
        let lookup = lookup_omap_record(&records, 500, 10);
        assert!(lookup.matched);
        assert_eq!(lookup.physical_address, Some(20));
        let missing = lookup_omap_record(&records, 500, 9);
        assert!(!missing.matched);
    }

    #[test]
    fn parses_apfs_gpt_partition_entry() {
        let mut entry = [0_u8; GPT_PARTITION_ENTRY_MIN_SIZE];
        entry[0..16].copy_from_slice(&APFS_GPT_TYPE_GUID_BYTES);
        entry[32..40].copy_from_slice(&2048_u64.to_le_bytes());
        entry[40..48].copy_from_slice(&4095_u64.to_le_bytes());
        let parsed = parse_gpt_partition_entry(&entry).unwrap().unwrap();
        assert!(parsed.is_apfs);
        assert_eq!(parsed.type_guid, "7c3457ef-0000-11aa-aa11-00306543ecac");
        assert_eq!(parsed.first_lba, 2048);
    }

    #[test]
    fn crc32_matches_standard_check_value() {
        assert_eq!(crc32_ieee(b"123456789"), 0xcbf4_3926);
    }
    #[test]
    fn parses_synthetic_volume_superblock() {
        let mut block = [0_u8; 4096];
        block[8..16].copy_from_slice(&1000_u64.to_le_bytes());
        block[16..24].copy_from_slice(&80_u64.to_le_bytes());
        block[24..28].copy_from_slice(&(OBJ_PHYSICAL | u32::from(OBJECT_TYPE_FS)).to_le_bytes());
        block[32..36].copy_from_slice(b"APSB");
        block[36..40].copy_from_slice(&0_u32.to_le_bytes());
        block[88..96].copy_from_slice(&7_u64.to_le_bytes());
        block[112..120].copy_from_slice(&2000_u64.to_le_bytes());
        block[160..168].copy_from_slice(&3_u64.to_le_bytes());
        block[168..176].copy_from_slice(&2_u64.to_le_bytes());
        block[216..232].copy_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
        block[512..523].copy_from_slice(b"SyntheticHD");
        block[768..770].copy_from_slice(&0x0040_u16.to_le_bytes());
        sign_block(&mut block);
        let parsed = parse_apfs_volume_superblock_with_checksum(&block).unwrap();
        assert!(parsed.checksum.valid);
        assert_eq!(parsed.magic, "APSB");
        assert_eq!(parsed.object.oid, 1000);
        assert_eq!(parsed.root_tree_oid, 2000);
        assert_eq!(parsed.volume_name, "SyntheticHD");
        assert_eq!(parsed.role, 0x0040);
    }

    #[test]
    fn parses_synthetic_btree_index_records_and_selects_child() {
        let mut block = vec![0_u8; 4096];
        block[8..16].copy_from_slice(&99_u64.to_le_bytes());
        block[16..24].copy_from_slice(&70_u64.to_le_bytes());
        block[24..28]
            .copy_from_slice(&(OBJ_PHYSICAL | u32::from(OBJECT_TYPE_BTREE_NODE)).to_le_bytes());
        block[32..34].copy_from_slice(&BTREE_NODE_ROOT.to_le_bytes());
        block[34..36].copy_from_slice(&1_u16.to_le_bytes());
        block[36..40].copy_from_slice(&2_u32.to_le_bytes());
        block[40..42].copy_from_slice(&0_u16.to_le_bytes());
        block[42..44].copy_from_slice(&8_u16.to_le_bytes());
        // TOC entries
        block[56..58].copy_from_slice(&128_u16.to_le_bytes());
        block[58..60].copy_from_slice(&512_u16.to_le_bytes());
        block[60..62].copy_from_slice(&160_u16.to_le_bytes());
        block[62..64].copy_from_slice(&544_u16.to_le_bytes());
        // keys and child values
        block[56 + 128..56 + 136].copy_from_slice(&700_u64.to_le_bytes());
        block[56 + 136..56 + 144].copy_from_slice(&70_u64.to_le_bytes());
        block[56 + 512..56 + 520].copy_from_slice(&100_u64.to_le_bytes());
        block[56 + 160..56 + 168].copy_from_slice(&900_u64.to_le_bytes());
        block[56 + 168..56 + 176].copy_from_slice(&70_u64.to_le_bytes());
        block[56 + 544..56 + 552].copy_from_slice(&101_u64.to_le_bytes());
        sign_block(&mut block);
        let node = parse_btree_node_with_checksum(&block).unwrap();
        let records = parse_omap_index_records_from_btree_node(&block, &node).unwrap();
        assert_eq!(records.len(), 2);
        let selected = select_synthetic_btree_child(&records, 800, 70);
        assert_eq!(selected.selected_child_oid, Some(101));
    }
}
