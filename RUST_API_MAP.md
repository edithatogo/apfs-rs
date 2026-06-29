# APFS-RS Rust API Map

## `crates/apfs-blockdev/src/lib.rs`

- line 13: `enum BlockDeviceError`
- line 24: `trait ReadOnlyBlockDevice`
- line 36: `struct ImageBlockDevice`
- line 43: `fn open`
- line 50: `fn path`
- line 76: `struct MemoryBlockDevice`
- line 81: `fn new`

## `crates/apfs-cli/src/main.rs`

- No public items detected by cargoless scanner.

## `crates/apfs-core/src/lib.rs`

- line 20: `enum InspectError`
- line 35: `enum InspectStatus`
- line 43: `enum SourceLayout`
- line 51: `enum ObjectLookupStatus`
- line 59: `enum ObjectMapResolverStatus`
- line 67: `enum ObjectMapResolverMode`
- line 75: `enum VolumeReportStatus`
- line 83: `enum DirectoryReportStatus`
- line 91: `enum FileReadReportStatus`
- line 98: `struct InspectReport`
- line 114: `struct ObjectLookupReport`
- line 130: `struct ObjectMapResolverEnvelope`
- line 142: `struct VolumeReportEnvelope`
- line 155: `struct VolumeProbeReport`
- line 165: `struct DirectoryReportEnvelope`
- line 182: `struct FileReadReportEnvelope`
- line 198: `struct ObjectMapResolverReport`
- line 215: `struct ResolvedObjectLookup`
- line 222: `struct GptReport`
- line 231: `struct CheckpointScanReport`
- line 244: `struct CheckpointCandidate`
- line 253: `struct CheckpointMapReport`
- line 266: `struct MappedObjectMapReport`
- line 275: `struct MappedBTreeReport`
- line 289: `struct MappedBTreeLeafReport`
- line 298: `struct SyntheticBTreeTraversalReport`
- line 313: `struct BTreeTraversalPathReport`
- line 321: `enum BTreeCursorStatus`
- line 329: `enum BTreeCursorMode`
- line 337: `enum BTreeCursorStepKind`
- line 345: `struct BTreeCursorEnvelope`
- line 357: `struct BTreeCursorReport`
- line 373: `struct BTreeCursorStepReport`
- line 390: `enum MappedObjectReadStatus`
- line 397: `struct MappedObjectReadEnvelope`
- line 414: `struct MappedObjectReadReport`
- line 428: `struct Diagnostic`
- line 434: `struct SafetySummary`
- line 452: `fn inspect_device`
- line 494: `fn inspect_bytes`
- line 502: `fn lookup_object_in_device`
- line 511: `fn lookup_object_in_bytes`
- line 532: `fn resolver_report_in_device`
- line 537: `fn resolver_report_in_bytes`
- line 555: `fn volume_report_in_device`
- line 560: `fn volume_report_in_bytes`
- line 578: `fn directory_report_in_device`
- line 583: `fn directory_report_in_bytes`
- line 605: `fn file_read_report_in_device`
- line 610: `fn file_read_report_in_bytes`
- line 932: `fn btree_cursor_report_in_device`
- line 941: `fn btree_cursor_report_in_bytes`
- line 959: `fn read_mapped_object_in_device`
- line 968: `fn read_mapped_object_in_bytes`
- line 990: `fn btree_cursor_report_in_report`
- line 1050: `fn resolver_report_in_report`

## `crates/apfs-test/src/lib.rs`

- line 7: `struct FixtureManifest`
- line 21: `struct CreatedWith`
- line 28: `struct ApfsFeatureManifest`
- line 38: `struct ExpectedArtifacts`
- line 45: `struct RedactionManifest`
- line 51: `struct ManifestValidation`
- line 56: `fn helper_crate_ready`
- line 60: `fn load_fixture_manifest`
- line 65: `fn load_fixture_manifest_from_reader`
- line 71: `fn validate_fixture_manifest`

## `crates/apfs-types/src/lib.rs`

- line 56: `enum ParseError`
- line 84: `struct ObjectChecksum`
- line 94: `struct ObjectHeader`
- line 106: `struct ContainerSuperblock`
- line 138: `struct CheckpointMapping`
- line 151: `struct CheckpointMapBlock`
- line 161: `struct ObjectMap`
- line 176: `struct VolumeSuperblock`
- line 207: `struct BTreeTableSpace`
- line 213: `struct BTreeTocEntry`
- line 219: `struct BTreeNode`
- line 236: `struct OmapRecord`
- line 245: `struct BTreeIndexRecord`
- line 252: `struct BTreeChildSelection`
- line 263: `struct FileSystemDirectoryRecord`
- line 274: `struct OmapLookup`
- line 288: `struct SyntheticDirectoryRecord`
- line 300: `struct SyntheticFileExtentRecord`
- line 308: `struct GptHeader`
- line 326: `struct GptPartitionEntry`
- line 337: `struct GptEntriesChecksum`
- line 344: `fn parse_object_header`
- line 360: `fn parse_nx_superblock`
- line 364: `fn parse_nx_superblock_with_checksum`
- line 416: `fn parse_checkpoint_map_block_with_checksum`
- line 447: `fn parse_checkpoint_mapping`
- line 463: `fn parse_omap_phys_with_checksum`
- line 485: `fn parse_apfs_volume_superblock_with_checksum`
- line 527: `fn parse_btree_node_with_checksum`
- line 600: `fn parse_omap_records_from_btree_node`
- line 641: `fn parse_omap_index_records_from_btree_node`
- line 680: `fn select_synthetic_btree_child`
- line 714: `fn parse_synthetic_directory_records_from_btree_node`
- line 749: `fn lookup_omap_record`
- line 795: `fn parse_synthetic_directory_records_v2_from_btree_node`
- line 853: `fn parse_synthetic_file_extent_records_from_btree_node`
- line 901: `fn validate_object_checksum`
- line 917: `fn apfs_fletcher64`
- line 934: `fn parse_gpt_header`
- line 969: `fn gpt_header_crc32`
- line 979: `fn validate_gpt_entries_checksum`
- line 989: `fn parse_gpt_partition_entry`
- line 1016: `fn gpt_entries_byte_len`
- line 1023: `fn crc32_ieee`

## `crates/apfs-vfs/src/lib.rs`

- line 8: `enum VfsNodeKind`
- line 16: `struct VfsMetadata`
- line 32: `struct VfsEntry`
- line 38: `struct VfsReadResult`
- line 47: `enum VfsError`
- line 60: `trait ReadOnlyVfs`
- line 83: `fn reject_path_traversal`

## `crates/apfs-win/src/lib.rs`

- line 7: `enum WindowsMountPlanError`
- line 18: `enum WindowsMountPlanStatus`
- line 24: `struct WindowsMountPlan`
- line 40: `fn plan_read_only_mount`
- line 98: `fn is_simple_drive_mount_point`

## `xtask/src/main.rs`

- No public items detected by cargoless scanner.
